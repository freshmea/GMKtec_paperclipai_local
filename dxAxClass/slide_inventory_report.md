# Slide Inventory Report

## Summary

- Root: `/home/aa/kuBig2026/dxAxClass`
- PDFs inventoried: 40
- Total pages: 790
- Non-extractable PDFs: 40
- Parse errors: 0

## Weekly Summary

- Week 1: 11 PDFs, 215 pages, 11 non-extractable
- Week 2: 10 PDFs, 200 pages, 10 non-extractable
- Week 3: 10 PDFs, 200 pages, 10 non-extractable
- Week 4: 9 PDFs, 175 pages, 9 non-extractable
  - Missing counterpart: 18-1.pdf missing

## Anomalies

- `1주차/2-2.pdf`: page_count_15
- `1주차/5-1-1.pdf`: has_revision_suffix
- `4주차/18-2.pdf`: page_count_15

## Downstream Guidance

- Use `slide_inventory.json` as the normalized source for script authoring and QA automation.
- These PDFs appear image-only to `pypdf`, so OCR or editable source files are still required for slide text recovery.
