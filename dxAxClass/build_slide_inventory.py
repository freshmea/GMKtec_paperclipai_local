from __future__ import annotations

import json
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Iterator

ROOT = Path(__file__).resolve().parent
TOOLS = ROOT / ".tools"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from pypdf import PdfReader

try:
    import numpy as np
    import pypdfium2 as pdfium
    from rapidocr_onnxruntime import RapidOCR
except Exception:
    np = None
    pdfium = None
    RapidOCR = None

OCR_ENGINE = None


EXPECTED_PAGES = 20
WEEK_DIR_RE = re.compile(r"(?P<week>\d+)주차")
FILE_RE = re.compile(
    r"(?P<deck>\d+)-(?P<part>\d+)(?:-(?P<revision>\d+))?\.pdf$",
    re.IGNORECASE,
)


@dataclass
class PdfRecord:
    week: int
    relative_path: str
    file_name: str
    deck: int | None
    part: int | None
    revision: int | None
    page_count: int | None
    nonempty_pages: int | None
    total_chars: int | None
    avg_chars_per_page: float | None
    status: str
    anomalies: list[str]
    sample_page_char_counts: list[dict[str, int]]
    ocr_available: bool
    ocr_pages_with_text: int = 0
    ocr_total_chars: int = 0
    error: str | None = None


def classify_pdf(pdf_path: Path) -> PdfRecord:
    week_match = WEEK_DIR_RE.fullmatch(pdf_path.parent.name)
    file_match = FILE_RE.fullmatch(pdf_path.name)
    anomalies: list[str] = []

    week = int(week_match.group("week")) if week_match else -1
    deck = int(file_match.group("deck")) if file_match else None
    part = int(file_match.group("part")) if file_match else None
    revision = int(file_match.group("revision")) if file_match and file_match.group("revision") else None

    if not week_match:
        anomalies.append("unexpected_week_directory")
    if not file_match:
        anomalies.append("unexpected_file_name_pattern")

    try:
        reader = PdfReader(str(pdf_path))
        sample_page_char_counts: list[dict[str, int]] = []
        total_chars = 0
        nonempty_pages = 0

        for page_number, page in enumerate(reader.pages, start=1):
            try:
                text = (page.extract_text() or "").strip()
            except Exception:
                text = ""
            chars = len(text)
            if page_number <= 5:
                sample_page_char_counts.append({"page": page_number, "chars": chars})
            total_chars += chars
            if chars:
                nonempty_pages += 1

        page_count = len(reader.pages)
        avg_chars = round(total_chars / page_count, 2) if page_count else 0.0
        status = "text_extractable" if nonempty_pages else "image_only_or_unextractable"

        if page_count != EXPECTED_PAGES:
            anomalies.append(f"page_count_{page_count}")
        if revision is not None:
            anomalies.append("has_revision_suffix")

        return PdfRecord(
            week=week,
            relative_path=str(pdf_path.relative_to(ROOT)),
            file_name=pdf_path.name,
            deck=deck,
            part=part,
            revision=revision,
            page_count=page_count,
            nonempty_pages=nonempty_pages,
            total_chars=total_chars,
            avg_chars_per_page=avg_chars,
            status=status,
            anomalies=anomalies,
            sample_page_char_counts=sample_page_char_counts,
            ocr_available=RapidOCR is not None and pdfium is not None and np is not None,
        )
    except Exception as exc:
        anomalies.append("parse_error")
        return PdfRecord(
            week=week,
            relative_path=str(pdf_path.relative_to(ROOT)),
            file_name=pdf_path.name,
            deck=deck,
            part=part,
            revision=revision,
            page_count=None,
            nonempty_pages=None,
            total_chars=None,
            avg_chars_per_page=None,
            status="parse_error",
            anomalies=anomalies,
            sample_page_char_counts=[],
            ocr_available=RapidOCR is not None and pdfium is not None and np is not None,
            error=f"{type(exc).__name__}: {exc}",
        )


def collect_records() -> list[PdfRecord]:
    records: list[PdfRecord] = []
    for pdf_path in sorted(ROOT.glob("*주차/*.pdf")):
        records.append(classify_pdf(pdf_path))
    return records


def find_missing_pairs(records: list[PdfRecord]) -> dict[int, list[str]]:
    parts_by_week_deck: dict[int, dict[int, set[int]]] = defaultdict(lambda: defaultdict(set))
    for record in records:
        if record.deck is None or record.part is None or record.week < 0:
            continue
        parts_by_week_deck[record.week][record.deck].add(record.part)

    missing: dict[int, list[str]] = {}
    for week, decks in parts_by_week_deck.items():
        week_missing: list[str] = []
        for deck, parts in sorted(decks.items()):
            if 1 in parts and 2 not in parts:
                week_missing.append(f"{deck}-2.pdf missing")
            if 2 in parts and 1 not in parts:
                week_missing.append(f"{deck}-1.pdf missing")
        if week_missing:
            missing[week] = week_missing
    return missing


def build_manifest(records: list[PdfRecord]) -> dict:
    missing_pairs = find_missing_pairs(records)
    total_pages = sum(record.page_count or 0 for record in records)
    nonextractable = sum(1 for record in records if record.status == "image_only_or_unextractable")
    parse_errors = [record.relative_path for record in records if record.status == "parse_error"]

    anomaly_files = [
        {
            "file": record.relative_path,
            "anomalies": record.anomalies,
        }
        for record in records
        if record.anomalies
    ]

    weekly_summary = []
    for week in sorted({record.week for record in records if record.week > 0}):
        week_records = [record for record in records if record.week == week]
        weekly_summary.append(
            {
                "week": week,
                "pdf_count": len(week_records),
                "page_total": sum(record.page_count or 0 for record in week_records),
                "nonextractable_pdf_count": sum(
                    1
                    for record in week_records
                    if record.status == "image_only_or_unextractable"
                ),
                "missing_pairs": missing_pairs.get(week, []),
            }
        )

    return {
        "root": str(ROOT),
        "summary": {
            "pdf_count": len(records),
            "page_total": total_pages,
            "nonextractable_pdf_count": nonextractable,
            "parse_error_count": len(parse_errors),
            "expected_page_count_per_pdf": EXPECTED_PAGES,
        },
        "weekly_summary": weekly_summary,
        "anomaly_files": anomaly_files,
        "records": [record.__dict__ for record in records],
    }


def build_report(manifest: dict) -> str:
    summary = manifest["summary"]
    lines = [
        "# Slide Inventory Report",
        "",
        "## Summary",
        "",
        f"- Root: `{manifest['root']}`",
        f"- PDFs inventoried: {summary['pdf_count']}",
        f"- Total pages: {summary['page_total']}",
        f"- Non-extractable PDFs: {summary['nonextractable_pdf_count']}",
        f"- OCR-capable runtime: {summary['ocr_runtime_available']}",
        f"- OCR pages with text: {summary['ocr_pages_with_text']}",
        f"- Parse errors: {summary['parse_error_count']}",
        "",
        "## Weekly Summary",
        "",
    ]

    for week_info in manifest["weekly_summary"]:
        lines.append(
            f"- Week {week_info['week']}: {week_info['pdf_count']} PDFs, "
            f"{week_info['page_total']} pages, "
            f"{week_info['nonextractable_pdf_count']} non-extractable"
        )
        for missing in week_info["missing_pairs"]:
            lines.append(f"  - Missing counterpart: {missing}")

    lines.extend(
        [
            "",
            "## Anomalies",
            "",
        ]
    )

    if not manifest["anomaly_files"]:
        lines.append("- None")
    else:
        for anomaly in manifest["anomaly_files"]:
            joined = ", ".join(anomaly["anomalies"])
            lines.append(f"- `{anomaly['file']}`: {joined}")

    lines.extend(
        [
            "",
            "## Downstream Guidance",
            "",
            "- Use `slide_inventory.json` as the normalized source for script authoring and QA automation.",
            "- These PDFs appear image-only to `pypdf`, so OCR is required for slide text recovery.",
        ]
    )
    return "\n".join(lines) + "\n"


def iter_ocr_pages(pdf_path: Path) -> Iterator[tuple[int, str]]:
    if RapidOCR is None or pdfium is None or np is None:
        return

    global OCR_ENGINE
    if OCR_ENGINE is None:
        OCR_ENGINE = RapidOCR()
    pdf = pdfium.PdfDocument(str(pdf_path))
    try:
        for page_index in range(len(pdf)):
            page = pdf[page_index]
            bitmap = page.render(scale=1.5)
            image = bitmap.to_pil()
            result, _ = OCR_ENGINE(np.array(image))
            lines = []
            if result:
                for item in result:
                    text = item[1].strip()
                    if text:
                        lines.append(text)
            yield page_index + 1, "\n".join(lines)
    finally:
        pdf.close()


def write_text_exports(manifest: dict) -> None:
    export_root = ROOT / "slide_txt"
    export_root.mkdir(exist_ok=True)

    index_lines = [
        "Slide TXT Export Index",
        "",
        f"Root: {manifest['root']}",
        f"PDF count: {manifest['summary']['pdf_count']}",
        f"Non-extractable PDFs: {manifest['summary']['nonextractable_pdf_count']}",
        "",
        "Files:",
    ]

    for record in manifest["records"]:
        rel_path = Path(record["relative_path"])
        out_path = export_root / rel_path.with_suffix(".txt")
        out_path.parent.mkdir(parents=True, exist_ok=True)
        pdf_path = ROOT / rel_path
        ocr_text_by_page: dict[int, str] = {}
        ocr_pages_with_text = 0
        ocr_total_chars = 0

        if record["ocr_available"] and record["status"] != "parse_error":
            for page_number, page_text in iter_ocr_pages(pdf_path):
                cleaned = page_text.strip()
                ocr_text_by_page[page_number] = cleaned
                if cleaned:
                    ocr_pages_with_text += 1
                    ocr_total_chars += len(cleaned)

        record["ocr_pages_with_text"] = ocr_pages_with_text
        record["ocr_total_chars"] = ocr_total_chars

        lines = [
            f"Source PDF: {record['relative_path']}",
            f"Status: {record['status']}",
            f"Week: {record['week']}",
            f"Deck: {record['deck']}",
            f"Part: {record['part']}",
            f"Revision: {record['revision']}",
            f"Page count: {record['page_count']}",
            f"Nonempty pages: {record['nonempty_pages']}",
            f"Total chars: {record['total_chars']}",
            f"Average chars per page: {record['avg_chars_per_page']}",
            f"OCR available: {record['ocr_available']}",
            f"OCR pages with text: {record['ocr_pages_with_text']}",
            f"OCR total chars: {record['ocr_total_chars']}",
        ]

        if record["anomalies"]:
            lines.append(f"Anomalies: {', '.join(record['anomalies'])}")
        if record.get("error"):
            lines.append(f"Error: {record['error']}")

        lines.extend(
            [
                "",
                "Page data:",
            ]
        )

        page_count = record["page_count"] or 0
        sample_counts = {
            entry["page"]: entry["chars"]
            for entry in record["sample_page_char_counts"]
        }
        for page_number in range(1, page_count + 1):
            chars = sample_counts.get(page_number, 0)
            ocr_text = ocr_text_by_page.get(page_number, "")
            lines.extend(
                [
                    f"",
                    f"=== Page {page_number} ===",
                    f"chars: {chars}",
                    "parser_text: [no extractable text detected with local parser]",
                    "ocr_text:",
                    ocr_text if ocr_text else "[no OCR text detected]",
                ]
            )

        out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
        index_lines.append(str(out_path.relative_to(ROOT)))

    (export_root / "INDEX.txt").write_text(
        "\n".join(index_lines) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    records = collect_records()
    manifest = build_manifest(records)
    write_text_exports(manifest)
    manifest["summary"]["ocr_runtime_available"] = (
        RapidOCR is not None and pdfium is not None and np is not None
    )
    manifest["summary"]["ocr_pages_with_text"] = sum(
        record["ocr_pages_with_text"] for record in manifest["records"]
    )
    (ROOT / "slide_inventory.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    (ROOT / "slide_inventory_report.md").write_text(
        build_report(manifest),
        encoding="utf-8",
    )
    print("Wrote slide_inventory.json, slide_inventory_report.md, and slide_txt/")


if __name__ == "__main__":
    main()
