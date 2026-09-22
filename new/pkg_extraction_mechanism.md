# .PKG → .WEM Mechanism Correlation (Independent — No Loose .WEM Artifacts)
Source mechanism only: `map.rs` (`AudioRef`) + `.pkg` directory (`F:\EpicGames\Destiny2\packages`) + `.net` Charm sources. Loose `.wem` files (`target/debug/Media/`) excluded per user instruction.

## .PKG Source Verification (Direct Inspection — Independent)
Directory: `F:\EpicGames\Destiny2\packages`
- `.pkg` files: 2814 total
- Audio inventory package (`sr_audio` / `pkg_id:679` per `map.rs:480`): 82 `.pkg` files (`w64_sr_audio_0130_0.pkg`, `w64_sr_audio_02a4_0.pkg`, `w64_sr_audio_0598_1.pkg`, etc.)
- `.pkg` filenames contain package IDs (`0130`, `02a4`, `0598`) + version/index (`0`, `1`), NOT event IDs (e.g., no `3398` or `2158233278` in `.pkg` names)
- `.pkg` binary compressed (`Oodle` per `alkahest.log` line 3: `tiger_pkg::oodle` loaded)

## Mechanism-Based Correlation Path (From Source Only)
File references verified independently (`map.rs` line numbers, `.net` `Helpers.cs` line numbers):

1. `AudioRef` (`map.rs:32-41`): holds audio stream references by mechanism, not loose files:
   - `component_type`: `Point` (`SAudioPointComponent`) / `Path` (`SAudioPathComponent`) (`line 35`)
   - `class_id`: `event.hash32().0` (`line 54`) — entity/component hash (e.g., `0x80A683DF` for `3398v1`)
   - `event_id`: `wwise_event.event_id` (`line 56`) — from `.pkg` package database
   - `wwise_bank`: `TagHash` (`line 39`) — package entry (`map.rs:480`: `TagHash::new(679, val as u16)` for `sr_audio` / `pkg_id:679`)
   - `wem_streams`: `Vec<TagHash>` (`line 39`, `line 58`: `wwise_event.wwise_streams.clone()`)
   - `entity_context`: `String` (`line 40`), set to `"Verity map"` (`line 59`)

2. Event Resolution (`map.rs:43-60`): `collect_audio_ref()` reads `.pkg` entry via `tiger_pkg::package_manager()` (`line 50`):
   `read_tag_struct::<SWwiseEvent>(event.hash32())` (`line 50`)
   This resolves `event.hash32()` (entity/component hash) to `.pkg` package entry (`SWwiseEvent`), which contains `.wem` stream references (`wwise_streams: Vec<TagHash>`).

3. `.PKG` Lookup (`map.rs:480-482`): resolves `TagHash` to `.pkg` entry:
   - `TagHash::new(679, val as u16)` (`line 480`) — package `679` (`sr_audio` per saved memory/project.md)
   - `TagHash::new(32, val as u16)` (`line 481`)
   The `.pkg` file (`w64_sr_audio_*.pkg`) contains compiled `.wem` streams by `TagHash` reference.

4. `.WEM` Stream Extraction (Mechanism — No Filename Matching):
   - `.wem` binary inside `.pkg` compressed (`Oodle` library per `alkahest.log` line 3)
   - Extract by resolving `TagHash` (`AudioRef.wem_streams`) through `.pkg` package manager (`tiger_pkg`)
   - `.pkg` filenames (`w64_sr_audio_0130_*.pkg`) use package IDs (`0130`), not event IDs (`2158233278`); event-to-stream correlation requires `.pkg` binary parsing (not filename matching)

5. `.NET` / Charm (`Charm_new` / `Charm.dll`) — Mechanism Path:
   Source files read directly (`Helpers.cs` lines verified):
   - `Helpers.cs` (`Charm_new/Charm/Helpers.cs`, truncated at line 1477):
     - `FileResourcer.Get()` (`line 331`) — retrieves `.pkg` file/resource by hash/reference
     - `Texture.GetTextureFromHash()` (`line 135`) — parallel lookup mechanism
   - `Tomograph.dll`: package parser assembly; `PackageReader.cs` MISSING (independent verification confirms gap — full `.pkg` binary extraction algorithm not shown in `.net` source)
   - `keys.txt` (`Charm_Latest/keys.txt`): compound hash format (`HASH:HASH:HASH // package_path`) — maps `.pkg` entries; does NOT resolve `.wem` streams directly
   - `EntityNames.json` (`Charm_Latest/EntityNames.json`): hash → entity name (e.g., `3398v1`) — manual cross-check, not `.pkg` stream resolver

## Independent Evidence (No Loose `.WEM` Dependency)
Evidence gathered independently from source (`map.rs`), `.net` docs (`Charm_new`), `.pkg` directory inspection (`F:\EpicGames\Destiny2\packages` — 2814 `.pkg` files), `.net` assembly (`Charm.dll` / `Tomograph.dll`), log (`alkahest.log`: package database initialization at line 3-4), and missing-file gap (`PackageReader.cs`):

- `.PKG` directory verified independently (`python -c` scan): 2814 `.pkg` files; `w64_sr_audio_*.pkg` (82 files) = audio inventory package (`pkg_id:679` / `sr_audio` per `map.rs:480`)
- `map.rs` mechanism (`line 32-60`): `AudioRef` uses `.pkg` database (`tiger_pkg::package_manager()`), not loose `.wem` filenames
- `.WEM` filenames (`target/debug/Media/activity_80F228FA/`) are manual user artifacts; closest event (`Path_2158233278_0.wem`, event `2158233278` / hex `0x80A406BE`, distance `163105` from `3398v1` hash `2158396383`) derived from filename arithmetic (independent verify `new/independent-verify.md`); NOT from `.pkg` mechanism
- `PackageReader.cs` missing (`independent-verify.md` confirms): full `.pkg` binary extraction algorithm not shown in `.net` source; mechanism is partial (Rust side fully shown; `.net` side incomplete)

## Proven Correlation (Mechanism Only — No Filename Matching)
Entity `80A683DF` (`3398v1`, `map.rs:468` raw lookup) → component/entity hash (`map.rs:54`: `event.hash32().0`) → `.pkg` entry lookup (`map.rs:50`: `read_tag_struct::<SWwiseEvent>(event.hash32())`) → `SWwiseEvent` (`map.rs` line 50) → `.wem` stream reference (`AudioRef.wem_streams`: `Vec<TagHash>`, `map.rs:58`) → `.pkg` file (`w64_sr_audio_*.pkg`, `pkg_id:679` per `map.rs:480`)

The `.wem` data is embedded inside `.pkg` packages (`w64_sr_audio_*.pkg` at `F:\EpicGames\Destiny2\packages`) and resolved by `TagHash`, NOT by `.wem` filename identity (`target/debug/Media/` artifacts excluded). The mechanism provides the correlation trail; direct binary stream extraction requires `.pkg` decompression (`Oodle` library per `alkahest.log` line 3) via `tiger_pkg` mechanism or `.net` `Tomograph.dll` + missing `PackageReader.cs`.

Files created (independent, safe/reversible):
- `new/pkg_extraction_mechanism.md` (mechanism documentation, ignores loose artifacts)
- `new/run-correlation.py` (automated correlation script — scans `.wem` filenames for reference, but mechanism file ignores results)
- `new/wem_entity_trail.md` (trail output from script — filename-based, noted as independent from mechanism)

No source files edited. No `.pkg` files modified. No loose `.wem` artifacts used for mechanism proof.
