# Independent Verification — Alkahest 0.6.0

Timestamp: 2026-09-21T04:36:00Z (independent run, no parent session dependency)
Subagent ID: independent-verify-subagent
Repo root: F:\d2\alkahest-0.6-main
Plan file: plans/plan-alkahest-full-analysis.md (47 lines, read fully)

---

## Phase 0: Source File Discovery

All 5 core files confirmed present:

- src/world/label.rs — 1316 bytes
- src/world/pattern.rs — 26528 bytes
- src/ui/scene/mod.rs — 61044 bytes
- crates/data/map.rs — 21294 bytes
- crates/data/strings.rs — 4780 bytes

Status: PASS

---

## Phase 1: Implementation / Anti-Pattern Verification (map.rs)

File: crates/data/map.rs (541 lines, 21294 bytes)

- `fnv_hash_u32` present: lines 389-393 (FnvHasher::default(), write_u32, finish as u32). PASS.
- `Panel::bottom`: NOT present in file. PASS (removed).
- `WidgetType::Panel` import: NOT present in file. PASS (removed).
- `StringContainer` reference: line 386 (`StringContainer::load_all_global()`); also lines 487, 489. PASS.
- Line 427 area (JSON index): line 445 shows `strings.try_get(r.event_id)` — decimal `event_id` used directly, NOT converted via `fnv_hash_u32`. Anti-pattern PARTIALLY REMAINS. TSV lookup (line 397) uses `fnv_hash_u32(r.event_id)` correctly; JSON index does not.

Status: PASS (fix applied); ANTI-PATTERN PARTIAL (line 445 decimal event_id remains).

---

## Phase 2: .NET / Charm Cross-Reference

- `Charm_new` directory: NOT present under repo (`F:\d2\alkahest-0.6-main\Charm_new` missing). Note: exists at `F:\d2\Charm_new` (separate from repo).
- `EntityNames.json`: found at `F:\d2\Charm_Latest\EntityNames.json`.
- `keys.txt`: found at `F:\d2\Charm_Latest\keys.txt`.
- `Charm.dll`: found at `F:\d2\Charm_Latest\Charm.dll` and `F:\d2\Charm_new\Charm\bin\Debug\net...`.
- `PackageReader.cs`: NOT found (gap confirmed, per plan line 39).
- `Helpers.cs`: truncated at 1477 lines (gap confirmed, per plan line 38).

Status: PARTIAL — Charm_new not under repo; .net data files located externally; PackageReader.cs absent.

---

## Phase 3: Output / Build Artifacts

- `target/debug/Media`: EXISTS (947 items in directory tree).
- `.wem` files (recursive under repo): 938 files.
  - Sum: 82,007,680 bytes (~78.2 MB)
  - Average: 87,428.23 bytes
  - Max: 515,878 bytes (~504 KB)
  - Min: 5,817 bytes (~5.7 KB)
- `new/findings-report.md`: EXISTS, 58 lines.
- `new/findings-structured.json`: EXISTS, JSON VALID.
- `new/run-full-analysis.ps1`: EXISTS; executed independently via `pwsh -File`. Completed with non-fatal null-path error at line 125 (Charm_new missing under repo — expected). Key output lines: Phase 0 PASS, Phase 1 PASS, Phase 2 FAIL (Charm_new missing), Phase 3 PASS, anti-pattern NOTE (line 445 decimal event_id remains).

Status: PASS (media + .wem verified); script executed independently.

---

## Anti-Pattern Status (Own Assessment)

Fixed:
- `WidgetType::Panel` import removed from map.rs.
- `Panel::bottom` usage removed from map.rs.
- `fnv_hash_u32` applied for TSV string lookup (map.rs:397-403).

Remaining (partial):
- JSON index at map.rs:445 (`strings.try_get(r.event_id)`) uses decimal `event_id` instead of `fnv_hash_u32(r.event_id)`. This is the same anti-pattern documented in plan line 27 and findings-report.md line 30. TSV lookup is aligned; JSON index is not.

No new anti-patterns introduced by this verification.

---

## Gaps Confirmed by Independent Inspection

1. JSON index alignment (map.rs:445) — decimal event_id remains; should use `fnv_hash_u32`.
2. `.wem` binary duration approximation — `bps` (bits per sample) approximated; exact `fmt` chunk parsing not fully verified (plan line 40).
3. Full `FnvHash` encoding match — `StringContainer` keys derived from `SLocalizedStrings` (`Vec<FnvHash>`); direct decimal-to-FnvHash conversion (`fnv_hash_u32`) may not reproduce original encoding (plan line 41).
4. `Helpers.cs` truncated at 1477 lines — full hash lookup algorithm not shown.
5. `PackageReader.cs` missing (`Charm_new` / `.net` package parsing logic not fully shown).
6. `Charm_new` directory not under repo (exists at `F:\d2\Charm_new` separately).

---

## Discrepancies vs Plan File Claims

- Plan claims Phase 2 COMPLETE (`.net` source structure analyzed; `Charm.dll`/`EntityNames.json`/`keys.txt` connected). Independent verification confirms files exist externally (`F:\d2\Charm_Latest`, `F:\d2\Charm_new`) but `Charm_new` is NOT under repo root. Plan line 11 references `Charm_new` as if under repo; actual location is separate. This is a location discrepancy, not a content discrepancy.
- Plan claims Phase 3 COMPLETE (anti-pattern grep passes). Independent verification confirms `Panel::bottom` and `WidgetType::Panel` removed, but JSON index anti-pattern at line 445 remains. Plan line 27 explicitly notes this partial state; findings-report.md line 30 confirms. No contradiction — just confirmation that "complete" refers to verification performed, not full resolution.
- `.wem` count: plan line 13 mentions 469 `.wem` files; independent count shows 938. This is a significant discrepancy. The 469 figure may refer to a specific subdirectory (`target/debug/Media/activity_80F228FA_map_4_audio/`) rather than the full repo recursive count. Independent verification reports the full recursive count (938) and notes the subdirectory count separately if needed.

---

## Verification Script (`run-full-analysis.ps1`)

Executed independently: `pwsh -File F:\d2\alkahest-0.6-main\new\run-full-analysis.ps1`
Result: Completed. Non-fatal error at line 125 (`Test-Path` null path for Charm_new). All phases reported: Phase 0 PASS, Phase 1 PASS, Phase 2 FAIL (Charm_new missing), Phase 3 PASS, anti-pattern NOTE (line 445 partial). No modifications made to script.

---

## Findings-Report Cross-Reference Block

`new/findings-report.md` line 36: session-history cross-reference block IS present.
First line of block: `=== SESSION HISTORY CROSS-REFERENCE: "egui button not appearing in map tab" (ses_f3ed4c8b2ffennAqcp3Fo0CLcJ) ===`
Block covers root cause (`WidgetType::Panel` import, `Panel::bottom` overlap), fix applied, anti-pattern status, audio dump (`map.rs:348`), cross-reference (`StringContainer.load_all_global()`, `fnv_hash_u32()`), and gaps verified.

---

## Final Statement

Verification performed independently of parent session. No dependency on session memory. All checks executed directly against disk files (`map.rs`, `label.rs`, `pattern.rs`, `scene/mod.rs`, `strings.rs`, `Charm_new`/`Charm_Latest`, `target/debug/Media`, `.wem` files, `findings-report.md`, `findings-structured.json`, `run-full-analysis.ps1`).

Subagent timestamp: 2026-09-21T04:36:00Z
