# hardware/bom

`bom_v1.0_delta.csv` is the single BOM source of truth (no shadow spreadsheets). `servo_map.csv` maps bus IDs to joints.
Column `line` is a stable BOM line number; lines are referenced by number in reviews and approvals, so numbers are never reused.

## Provenance (moved from former data row 1)

Recreated 2026-11-22 ~21:15 Paris after the Team Computer rebuild (~20:55) wiped hardware/. Rebuilt from Woz chat record (20:31 grep + approvals); not byte-identical to the 19:31 file. Itemization of baseline core (281.08) and unpriced estimates (81) was lost and must be re-derived from the upstream BOM sheet in the 2026-11-25 quotes.

Licensing: CSVs CERN-OHL-S-2.0, docs CC-BY-SA-4.0 (see REUSE.toml, GOVERNANCE.md section 3).
