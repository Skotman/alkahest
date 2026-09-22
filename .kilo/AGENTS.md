# Agent Instructions — Audio Linkage / .kilo Config

Scope: Config-only. Source modifications (`map.rs`, `pattern.rs`) remain OUT OF SCOPE until explicitly requested.

## Critical Rules (from Claude/Gemini analysis + user's proposal)

- Never write custom bit-packing logic for `TagHash`. `TagHash` is a transparent `u32` newtype (`tiger-pkg`). Use `.0`, `.pkg_id()`, `.entry_index()` natively. No custom bridge module needed.
- Keep `entity_context` structured (entity hash / position), not a hardcoded `map_name` string. The current `context.to_string()` pushes a literal map name; replace with actual entity identity at the entity-iteration site.
- Multi-subagent split (Plan / Code / Debug) is preferred. Do NOT merge into a single agent/file. Each subagent handles a different action layer (analysis, edit, validation) and requires rollback isolation.

## Subagent Split (5 phases, preserved)

1. Plan — investigation only (search/analyze, NO edits). Report push site, call chain, entity data availability.
2. Code — modifications scoped to `crates/data/src/map.rs` (data layer for `AudioRef` / `event_widehash`) and `src/world/pattern.rs` or `map.rs` (UI/tab layer). Max 3 files.
3. Code — `WideHash::Hash64` resolution. Handle `Hash32(TagHash)` and `Hash64(TagHash64)` separately. Do NOT return `FFFFFFFF` for valid Hash64 events.
4. Code — manifest enhancement (TSV + JSON). Add columns: `entity_hash`, `entity_hash64`, `event_widehash`, `position`. No deduplication.
5. Debug — validation against Verity map (`0x80F1EE98`). Checkpoints before Phase 3 and Phase 4. If validation fails: STOP, report failure, do NOT proceed.

## Scope Constraints (Non-Negotiable)

Allowed to modify (if source phase executed): `src/data/map.rs`, `src/world/pattern.rs`, `src/world/map.rs`.
MUST NOT modify: `src/ui/tabs/map.rs`, `src/ui/scene/mod.rs`, `src/ui/hotkeys.rs` (Shift+P button/hotkey preserved).
MUST NOT change signatures: `pub fn dump_audio_references(map_name: &str)`, `static AUDIO_REFERENCES: OnceLock<Mutex<Vec<AudioRef>>>`.

## Validation Criteria (mechanical, before any unattended run)

- `entity_hash` column varies per TSV row (not identical per row).
- `FFFFFFFF` values resolved to real hash formats or stored natively for `Hash64`.
- `event_widehash` exported in TSV + JSON manifest.
- `.wem` count matches `wem_count` column.
- TSV + JSON manifest both produced; no omissions.

## Context Hygiene

- `.kilocodeignore` exclusions: `Media/`, `output/`, `*.pkg`, `*.wem`, `target/`, `.idea/`, `.vscode/`, `*.fbx`, `*.tmp`, `.DS_Store`.
- Checkpoints: `enabled: true`, `interval: 5`. Rollback via `/checkpoint restore`, not `git stash`.
- Scoped permissions (`kilo.jsonc`): `read`/`glob`/`grep` freely allowed; `edit`/`bash` gated; `external_directory` denied; `task` allowed for subagent delegation.

## Source Phase Note

Source phases (`map.rs` modifications, `WideHash` resolution, manifest export) are NOT implemented in this config-only plan. This file defines the rules; execution requires separate agent invocation.
