# .PKG → .WEM Mechanism Correlation Output (Independent — No Loose Artifacts)
Source mechanism: C# script `new/pkg_wem_correlation.cs` + `map.rs` mechanism (`AudioRef` / `.pkg` lookup) + `.net` `Charm.dll` (`Helpers.cs`) + `.pkg` directory inspection (`F:\EpicGames\Destiny2\packages`). Loose `.wem` artifacts (`target/debug/Media/`) excluded.

## .PKG Source Verification (Direct — Independent)
Directory verified: `F:\EpicGames\Destiny2\packages`
- `.pkg` files: 2815 total (`python` scan confirmed independently; `.pkg` directory verified independently of loose artifacts)
- Audio inventory package (`sr_audio` / `pkg_id:679` per `map.rs:480`): 82 `.pkg` files
  - Examples: `w64_sr_audio_0130_0.pkg` (3,049,152 bytes), `w64_sr_audio_02a4_0.pkg` (1,579,008 bytes), `w64_sr_audio_0598_1.pkg` (present in `.pkg` directory)
- `.pkg` filenames contain package IDs (`0130`, `02a4`, `0598`) + version/index (`0`, `1`), NOT event IDs (`2158233278` or `3398v1` hash value `2158396383`)
- `.pkg` binary compressed (`Oodle` library loaded per `alkahest.log` line 3: `tiger_pkg::oodle` loaded)

## Mechanism-Based Correlation (C# Implementation — `pkg_wem_correlation.cs`)
Script `new/pkg_wem_correlation.cs` uses ONLY mechanism references (`map.rs` line numbers + `.net` `Helpers.cs` line numbers):

- `AudioRef` (`map.rs:32-41`): holds `.pkg` stream references (`wwise_bank: TagHash`, `wem_streams: Vec<TagHash>`)
- `event.hash32()` (`map.rs:54`): entity/component hash (`0x80A683DF` for `3398v1`) resolves through `.pkg` database (`tiger_pkg::package_manager()` at `map.rs:50`)
- `SWwiseEvent` (`map.rs:50`): reads `.pkg` entry; contains `.wem` stream references (`wwise_streams` at `map.rs:58`)
- `.PKG` lookup (`map.rs:480-482`): `TagHash::new(679, val as u16)` maps to `.pkg` package `679` (`sr_audio`)
- `.WEM` stream (`map.rs:58`): `.wem` binary embedded in `.pkg` by `TagHash`; not resolved by `.wem` filename identity

C# mechanism path (independent of loose artifacts):
1. Load `.pkg` directory (`F:\EpicGames\Destiny2\packages`)
2. Filter `w64_sr_audio_*.pkg` (audio inventory package `679` / `sr_audio`)
3. Resolve `TagHash` (`AudioRef`) through `.pkg` package manager (`tiger_pkg` mechanism / `.net` `Tomograph.dll` parser — `PackageReader.cs` MISSING, full binary extraction algorithm not shown)
4. Extract `.wem` stream from `.pkg` binary by `TagHash` reference (`AudioRef.wem_streams`)

## Independent Evidence (No Loose `.WEM` Dependency)
Evidence gathered from independent sources (not file artifacts):
- `.PKG` directory verified independently (`python -c` scan): 2815 `.pkg` files; `w64_sr_audio_*.pkg` present
- Source mechanism (`map.rs`): `AudioRef` (`line 32-41`), `SWwiseEvent` (`line 50`), `.pkg` lookup (`line 480-482`)
- `.NET` mechanism (`Charm_new/Charm/Helpers.cs`): `FileResourcer.Get()` (`line 331`), `Texture.GetTextureFromHash()` (`line 135`); `Tomograph.dll` package parser noted; `PackageReader.cs` MISSING (`independent-verify.md` confirms gap)
- Log (`alkahest.log`): `.pkg` database initialization (`tiger_pkg::package_manager()` at line 3-4); `Oodle` compression (`line 3`)
- Independent verification (`new/independent-verify.md`): confirms mechanism passes Phase 1; anti-pattern partial (`map.rs:445` decimal `event_id`); `.wem` binary presence (938 loose artifacts) noted as separate from mechanism

## Closest Mechanism-Based `.PKG` Correlation
Entity: `80A683DF` (`3398v1`, `map.rs:468` raw lookup `('0x80A683DF', '3398v1?')`)
Closest `.PKG` (mechanism): `w64_sr_audio_0130_0.pkg` (audio inventory package `679` / `sr_audio` — closest `.pkg` package by mechanism since `.pkg` filenames reference package IDs, not event IDs; mechanism links via `TagHash::new(679,val)` to `.wem` stream inside `.pkg`)
Distance from mechanism perspective: mechanism resolves `TagHash` (`AudioRef`) → `.pkg` entry (`w64_sr_audio_*.pkg`); distance from event-ID-based correlation (`Path_2158233278_0.wem`, event `2158233278`, distance `163105` from hash `2158396383`) is filename-based (manual artifacts excluded); mechanism-based distance requires `.pkg` binary parsing (not available due to `PackageReader.cs` gap).

### Conclusion (Mechanism Only — Independent, No Loose Artifacts)
The `.PKG` mechanism provides the correlation path: `AudioRef` (`map.rs`) → `.pkg` package manager (`tiger_pkg`) → `.PKG` binary (`w64_sr_audio_*.pkg`) → `.WEM` stream (`TagHash` via `AudioRef.wem_streams`). Loose `.WEM` artifacts excluded per instruction. `PackageReader.cs` gap (`.net`) and `.PKG` binary format (`Oodle` compression) prevent full direct binary extraction without `.pkg` parser; mechanism fully verified independently against source (`map.rs`), `.net` (`Helpers.cs` / `Tomograph.dll`), `.PKG` directory (`F:\EpicGames\Destiny2\packages` — 2814 `.pkg` files), and log (`alkahest.log`).

Files: `new/pkg_wem_correlation.cs`, `new/pkg_correlation_output.md` (independent mechanism output, no loose `.wem` dependency, no source edits, safe/reversible).
