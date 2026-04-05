from __future__ import annotations

import json
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent
TOOLS = ROOT / ".tools"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

import pypdfium2 as pdfium
from PIL import Image

IMAGE_ROOT = ROOT / "slide_images"
BUNDLE_JSON = ROOT / "slide_review_bundle.json"
BUNDLE_MD = ROOT / "slide_review_bundle.md"
SOURCE_MANIFEST = ROOT / "slide_inventory.json"
JPEG_QUALITY = 70
RENDER_SCALE = 1.0


@dataclass
class BundleRecord:
    relative_pdf_path: str
    page_count: int
    image_dir: str
    image_files: list[str]
    anomalies: list[str]


def load_source_manifest() -> dict:
    return json.loads(SOURCE_MANIFEST.read_text(encoding="utf-8"))


def render_pdf(pdf_path: Path, output_dir: Path) -> tuple[int, list[str]]:
    output_dir.mkdir(parents=True, exist_ok=True)
    pdf = pdfium.PdfDocument(str(pdf_path))
    written: list[str] = []
    try:
        for page_index in range(len(pdf)):
            page = pdf[page_index]
            bitmap = page.render(scale=RENDER_SCALE)
            image = bitmap.to_pil().convert("RGB")
            image_name = f"page-{page_index + 1:02d}.jpg"
            image_path = output_dir / image_name
            image.save(
                image_path,
                format="JPEG",
                quality=JPEG_QUALITY,
                optimize=True,
            )
            written.append(str(image_path.relative_to(ROOT)))
    finally:
        pdf.close()

    return len(written), written


def build_bundle() -> dict:
    IMAGE_ROOT.mkdir(exist_ok=True)
    source = load_source_manifest()
    records: list[BundleRecord] = []

    for record in source["records"]:
        rel_pdf = Path(record["relative_path"])
        pdf_path = ROOT / rel_pdf
        output_dir = IMAGE_ROOT / rel_pdf.with_suffix("")
        page_count, image_files = render_pdf(pdf_path, output_dir)
        records.append(
            BundleRecord(
                relative_pdf_path=record["relative_path"],
                page_count=page_count,
                image_dir=str(output_dir.relative_to(ROOT)),
                image_files=image_files,
                anomalies=record.get("anomalies", []),
            )
        )

    total_images = sum(record.page_count for record in records)
    total_size_bytes = sum(path.stat().st_size for path in IMAGE_ROOT.rglob("*.jpg"))
    weeks = sorted(
        {
            int(Path(record.relative_pdf_path).parts[0].replace("주차", ""))
            for record in records
        }
    )

    return {
        "root": str(ROOT),
        "image_root": str(IMAGE_ROOT.relative_to(ROOT)),
        "render": {
            "format": "jpeg",
            "quality": JPEG_QUALITY,
            "scale": RENDER_SCALE,
        },
        "summary": {
            "pdf_count": len(records),
            "week_count": len(weeks),
            "total_images": total_images,
            "total_size_bytes": total_size_bytes,
        },
        "records": [record.__dict__ for record in records],
    }


def build_report(bundle: dict) -> str:
    summary = bundle["summary"]
    lines = [
        "# Slide Review Bundle",
        "",
        "## Summary",
        "",
        f"- Root: `{bundle['root']}`",
        f"- Image root: `{bundle['image_root']}`",
        f"- PDFs rendered: {summary['pdf_count']}",
        f"- Slide images rendered: {summary['total_images']}",
        f"- Bundle size: {summary['total_size_bytes'] / (1024 * 1024):.1f} MiB",
        f"- JPEG quality: {bundle['render']['quality']}",
        f"- Render scale: {bundle['render']['scale']}",
        "",
        "## Reviewer Notes",
        "",
        "- Use the per-PDF `slide_images/.../page-XX.jpg` files for multimodal review.",
        "- Cross-reference OCR placeholders in `slide_txt/` when a slide image is hard to read.",
        "- Prioritize anomaly decks first because they are most likely to break downstream script assumptions.",
        "",
        "## Anomaly Decks",
        "",
    ]

    anomaly_records = [record for record in bundle["records"] if record["anomalies"]]
    if not anomaly_records:
        lines.append("- None")
    else:
        for record in anomaly_records:
            lines.append(
                f"- `{record['relative_pdf_path']}` -> `{record['image_dir']}` "
                f"({', '.join(record['anomalies'])})"
            )

    lines.extend(
        [
            "",
            "## First Image Per Deck",
            "",
        ]
    )
    for record in bundle["records"]:
        first_image = record["image_files"][0] if record["image_files"] else "[no images]"
        lines.append(f"- `{record['relative_pdf_path']}` -> `{first_image}`")

    return "\n".join(lines) + "\n"


def main() -> None:
    bundle = build_bundle()
    BUNDLE_JSON.write_text(
        json.dumps(bundle, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    BUNDLE_MD.write_text(build_report(bundle), encoding="utf-8")
    print("Wrote slide_review_bundle.json, slide_review_bundle.md, and slide_images/")


if __name__ == "__main__":
    main()
