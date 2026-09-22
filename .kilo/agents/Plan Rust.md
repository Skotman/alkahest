---
description: Plan agent for audio linkage — primary agent that orchestrates investigation, entity threading, WideHash resolution, manifest export, and validation phases
mode: primary
color: "#3B82F6"
permission:
  read: allow
  glob: allow
  grep: allow
  edit: deny
  bash: deny
  task: allow
---

```markdown
# 🎯 ALKAHEST AUDIO LINKAGE RESOLVER - AGENT RULES
**Project:** Add entity-to-audio provenance tracking to existing dump_audio_references()
**Status:** Core extraction works; linkage is missing
**Last Updated:** 2026-09-21 (incorporates sessions 1,4,5 + Claude/Gemini analysis + user clarifications)

---

## 📌 PROJECT CONTEXT & GOALS

---

### ✅ WHAT WE'RE DOING

| Objective                                 | Scope                   | Priority |
| ----------------------------------------- | ----------------------- | -------- |
| Add entity linkage to audio extraction    | Modify 3 existing files | **P0**   |
| Fix FFFFFFFF resolution for Hash64 events | WideHash handling       | **P0**   |
| Export event_widehash to TSV              | dump_audio_references() | **P0**   |
| Prevent circular reference loops          | Traversal safety        | **P0**   |
| Preserve all existing functionality       | UI, hotkey, extraction  | **P0**   |

**Final Deliverable:** Each `.wem` file can be traced back to its source entity, component, and event hash.

---

### ❌ WHAT WE'RE NOT DOING (Explicit Rejections)

| Rejected Item                      | Source | Reason                                                                                      |
| ---------------------------------- | ------ | ------------------------------------------------------------------------------------------- |
| Standalone TigerExtractor CLI      | Gemini | User confirmed UI button (Shift+P) works and must be preserved                              |
| Universal Hash Bridge formula      | Gemini | User confirmed Alkahest 0.6.0 uses **multiple active hashmap formats**, not a single struct |
| EOF = "End-Of-File" chunk metadata | Gemini | User confirmed EOF = **Edge of Fate expansion** branch in Charm                             |
| Rewriting package parsing          | N/A    | `tiger-pkg` + `oo2core_9_win64.dll` already handle this correctly                           |
| Rewriting Oodle decompression      | N/A    | Handled by existing dependencies                                                            |
| Assuming infinite nested depth     | N/A    | User constraint: **"not to an extent where it starts looping into itself"**                 |

---

## 🔍 CURRENT STATE

---

### ✅ WORKING COMPONENTS

| Component                  | Location                                | Status    | Verified       |
| -------------------------- | --------------------------------------- | --------- | -------------- |
| UI Button                  | `src/ui/scene/mod.rs`                   | ✅ Working | User (Shift+P) |
| Hotkey                     | `src/ui/hotkeys.rs`                     | ✅ Working | User (Shift+P) |
| `dump_audio_references()`  | `alkahest_data::map`                    | ✅ Working | Session 4      |
| Audio extraction           | `.wem` files                            | ✅ Working | User           |
| AUDIO_REFERENCES collector | Static `OnceLock<Mutex<Vec<AudioRef>>>` | ✅ Working | Session 4      |

### ❌ BROKEN COMPONENTS

| Component        | Issue                      | Impact                | Root Cause                         |
| ---------------- | -------------------------- | --------------------- | ---------------------------------- |
| `entity_context` | Hardcoded map name string  | No per-entity linkage | Session 4: `context.to_string()`   |
| WideHash::Hash64 | Resolves to FFFFFFFF       | Missing 64-bit events | Session 4: Hash64 lookup failure   |
| event_widehash   | Collected but not exported | Missing from TSV      | Session 4: Field exists but unused |

---

### 📊 KNOWN HASH MAPPINGS (From Session 4)

**Map/Activity Hashes:**
```rust
0x80F228FA = Salvation's Edge raid (schism_spire.raid_splinter)
0x80F1EE98 = Verity bubble/encounter (String: "Verity")
0x80F1EEC8 = Verity map (FileHash)
```

**Sub-maps inside Verity:**
```rust
80F1EE28, 80AED34B, 80F1B516, 80F1EE7A, 80F1EE90, 80F1EE94, 80F1EE75, 80F1EE89, 80F1B022
```

**Component Type Hashes:**
```rust
0x8080666D = SAudioPathComponent (ambient/path audio)
0x8080666F = SAudioPointComponent (point/spawn audio)
0x80809738 = SWwiseEvent (Wwise event container)
0x80808CB5 = SRespawnPointsComponent (spawn points)
```

**Valid Wwise Event IDs in Verity (2026-09-20):**
```rust
// Point audio (SAudioPointComponent / 0x8080666F):
80D4E180, 80D4DFA2, 80D4E182, 80D4DFB0, 80D4E0DF, 80D4E160, 80D4DFAD, 80D4E14D

// Path audio (SAudioPathComponent / 0x8080666D):
80D4DF72, 80D4E03D (many instances), 80D4DF7A, 80D4DF75, 80D4DFAD, 80D4E096
```

---

## 🏗️ ARCHITECTURE DECISIONS

---

### 1. **Hybrid UI-Backend Model** (Claude ✅)
- **UI Layer:** Scene toolbar button + hotkey (Shift+P) **remain completely untouched**
- **Backend:** `dump_audio_references()` executes synchronously
- **Rationale:** User confirmed both UI triggers work; extraction is fast enough for current use cases

### 2. **Circular Reference Protection** (User Clarification ✅)
- **Constraint:** *"not to an extent where it starts looping into itself"*
- **Implementation:**
  - Add `max_depth` guard (suggested: 15) to prevent infinite recursion
  - Track visited hashes in a `HashSet<TagHash>` during traversal
  - If hash re-appears in current path → stop and log as cyclic reference
- **Pattern:**
  ```rust
  fn traverse_entity(
      entity: &TigerEntity,
      visited: &mut HashSet<TagHash>,
      depth: usize,
  ) -> Vec<AudioRef> {
      if depth > MAX_DEPTH || visited.contains(&entity.hash) {
          if visited.contains(&entity.hash) {
              log::warn!("Cyclic reference detected at {}", entity.hash);
          }
          return Vec::new();
      }
      visited.insert(entity.hash);
      let mut results = Vec::new();
      // ... process entity and collect audio refs
      for child in entity.children {
          results.extend(traverse_entity(child, visited, depth + 1));
      }
      results
  }
  ```

### 3. **Multi-Format Hash Handling** (User Correction ✅)
- Alkahest 0.6.0 uses **multiple active hashmap formats**:
  - `TagHash { pkg_id: u16, entry_index: u16 }` (unpacked)
  - Flat `u32` (packed: `0x80800000 | (pkg_id << 13) | entry_index`)
  - `TagHash64` (64-bit world IDs)
  - `WideHash` enum: `Hash32(TagHash) | Hash64(TagHash64)`
- **Requirement:** Handle each variant at its appropriate resolution point
- **Do NOT** assume a single conversion formula works globally

### 4. **Surgical Modification Scope** (Claude ✅)
- **Maximum files to modify:** 3
- **No new files** unless absolutely unavoidable
- **No architectural changes** to existing systems

---

## 🎯 SCOPE CONSTRAINTS (Non-Negotiable)

---

### ✅ FILES YOU MAY MODIFY
```text
src/data/map.rs         # Primary: dump_audio_references(), AudioRef
src/world/pattern.rs    # Secondary: entity iteration (if needed)
src/world/map.rs        # Tertiary: map loading (if needed)
```

### ❌ FILES YOU MUST NOT MODIFY
```text
src/ui/tabs/map.rs      # UI button is working - DON'T TOUCH
src/ui/scene/mod.rs     # Toolbar button is working - DON'T TOUCH
src/ui/hotkeys.rs       # Hotkey (Shift+P) is working - DON'T TOUCH
```

### ❌ FUNCTION SIGNATURES YOU MUST NOT CHANGE
```rust
pub fn dump_audio_references(map_name: &str)  // Keep signature
static AUDIO_REFERENCES: OnceLock<Mutex<Vec<AudioRef>>>  // Keep type
```

---

## 🔧 TECHNICAL SPECIFICATIONS

---

### Current AudioRef Struct
```rust
pub struct AudioRef {
    pub component_type: String,
    pub class_id: u32,           // event.hash32().0
    pub event_widehash: WideHash, // Already collected, NOT exported to TSV
    pub event_id: u32,           // wwise_event.event_id
    pub wwise_bank: String,
    pub wem_streams: Vec<TagHash>,
    pub entity_context: String,  // ❌ PROBLEM: Hardcoded map name
}
```

### Required AudioRef Struct (After Modifications)
```rust
pub struct AudioRef {
    pub component_type: String,
    pub class_id: u32,
    pub event_widehash: WideHash,
    pub event_id: u32,
    pub wwise_bank: String,
    pub wem_streams: Vec<TagHash>,
    pub entity_context: String,   // Keep for backwards compatibility
    pub entity_hash: TagHash,     // ✅ NEW: Primary entity identifier
    pub entity_hash64: Option<TagHash64>, // ✅ NEW: For WorldID linkage
    pub position: Option<[f32; 3]>, // ✅ NEW: Spatial coordinates
}
```

---

### WideHash Handling Pattern
```rust
// In resolution code, replace single-variant handling:
match event_widehash {
    WideHash::Hash32(h) => {
        // 32-bit path - existing logic
        event_id = h.0;
        event_id_64 = None;
    }
    WideHash::Hash64(h) => {
        // Attempt to resolve via package manager first
        if let Some(tag32) = package_manager().resolve_hash64_to_tag32(h) {
            event_id = tag32.0;
            event_id_64 = Some(h);
        } else {
            // Can't resolve to 32-bit - store Hash64 natively
            // DO NOT return FFFFFFFF - store the actual Hash64 value
            event_id = 0xFFFFFFFF; // Only for truly invalid/missing
            event_id_64 = Some(h);
        }
    }
};
```

---

## 🚀 EXECUTION WORKFLOW

---

### Phase 1: Investigation (Agent: **Plan**)
**Objective:** Find the exact push site and call chain

**Prompt for Kilo Code:**
```
INVESTIGATION TASK: Locate AudioRef collection point

1. Search for ALL occurrences of "AUDIO_REFERENCES" in the codebase
2. Find the exact line with: refs.push(AudioRef { ... })
3. Identify what value is passed as the `context` parameter at that push site
   - Is it a literal string?
   - Is it &self.name?
   - Is it map_name?
4. Trace the call chain BACKWARDS:
   - What function calls the function that pushes?
   - Continue until you reach the entity iteration loop
5. At the entity iteration site:
   - What type represents an entity?
   - Does it have a TagHash field?
   - Does it have a TagHash64/WorldID field?
   - Does it have position/transform data?
6. Report in this exact format:

---
INVESTIGATION RESULTS
---

[Push Site]
File: <path>
Line: <number>
Current context value: <exact code>

[Call Chain]
1. <function_a> (file:line) -> calls <function_b>
2. <function_b> (file:line) -> calls <function_c>
...
N. <entity_iteration_function> (file:line) -> iterates entities

[Entity Data Available]
- Has TagHash: <yes/no>
- Has TagHash64/WorldID: <yes/no>
- Has position: <yes/no>
- Type definition: <paste struct if small>

DO NOT modify any code. ONLY analyze and report.
```

**Success Criteria:**
- [ ] Push site located with file:line
- [ ] Current `context` value identified
- [ ] Complete call chain documented
- [ ] Entity data availability confirmed

---

### Phase 2: Entity Threading (Agent: **Code**)
**Objective:** Pass entity identity through the call chain

**Prompt for Kilo Code:**
```
IMPLEMENTATION TASK: Thread entity identity to AudioRef

Based on Phase 1 investigation results:

1. Modify AudioRef struct in src/data/map.rs:
   - Add: entity_hash: TagHash
   - Add: entity_hash64: Option<TagHash64>
   - Add: position: Option<[f32; 3]>
   - Keep: entity_context (String) for backwards compatibility

2. At the ENTITY ITERATION SITE (from Phase 1):
   - Capture the entity's TagHash as entity_hash
   - If available, capture entity's TagHash64 as entity_hash64
   - If available, capture entity's position as [x, y, z]

3. Pass these three values through EACH function in the call chain:
   - Add parameters to function signatures if needed
   - Forward values unchanged to next function
   - Do NOT modify logic, only add parameter passing

4. At the PUSH SITE:
   - Populate entity_hash, entity_hash64, position fields
   - Keep entity_context as-is (backwards compatibility)

5. CRITICAL: Do NOT deduplicate
   - One AudioRef per (entity, component) pair
   - Even if wem_streams are identical

6. Rebuild and verify it compiles

DO NOT:
- Change function signatures unnecessarily
- Touch UI code (src/ui/)
- Remove any existing fields
- Add any new files
```

**Success Criteria:**
- [ ] Code compiles without errors
- [ ] All new fields added to AudioRef
- [ ] Entity data threaded through call chain
- [ ] No deduplication logic added

---

### Phase 3: WideHash Resolution (Agent: **Code**)
**Objective:** Fix FFFFFFFF issue for Hash64 events

**Prompt for Kilo Code:**
```
IMPLEMENTATION TASK: Handle WideHash::Hash64 variants

1. Find where event_id is resolved from event_widehash
   - Search for: event_widehash, event_id, WideHash
   - Likely in: src/data/map.rs or src/world/

2. Current code probably only handles WideHash::Hash32
   - Find the match statement or if-let that processes event_widehash

3. Modify to handle BOTH variants:

```rust
// Replace existing single-variant handling with:
match event_widehash {
    WideHash::Hash32(h) => {
        event_id = h.0;
        event_id_64 = None;
    }
    WideHash::Hash64(h) => {
        // Try to resolve Hash64 to TagHash via package manager
        if let Some(resolved) = package_manager().resolve_hash64(h) {
            event_id = resolved.0;
            event_id_64 = Some(h);
        } else {
            // Cannot resolve - store Hash64 natively
            // DO NOT use FFFFFFFF for valid Hash64 events
            event_id = 0xFFFFFFFF; // Only for truly invalid
            event_id_64 = Some(h);
        }
    }
}
```

4. Add event_id_64 field to AudioRef if not already present:
   - Type: Option<TagHash64>
   - Store the original Hash64 when resolution fails

5. Verify:
   - All Hash32 events still work
   - Hash64 events no longer resolve to FFFFFFFF
   - Original Hash64 values preserved when unresolved

DO NOT:
- Return FFFFFFFF for valid Hash64 events
- Remove existing Hash32 handling
```

**Success Criteria:**
- [ ] No FFFFFFFF values for valid events
- [ ] All Hash64 events either resolved or stored
- [ ] Hash32 events unchanged

---

### Phase 4: Manifest Enhancement (Agent: **Code**)
**Objective:** Output comprehensive linkage data

**Prompt for Kilo Code:**
```
IMPLEMENTATION TASK: Export full linkage manifest

1. Modify dump_audio_references() in src/data/map.rs:

   a. Add these columns to TSV header:
      - entity_hash
      - entity_hash64
      - event_widehash
      - position
      - (keep all existing columns)

   b. Format each row:
      - entity_hash: 0x{:08X}
      - entity_hash64: 0x{:016X} (if Some), else empty
      - event_widehash: match on WideHash variant, format accordingly
      - position: [x,y,z] if Some, else empty
      - event_id: existing (32-bit)
      - All other existing fields: unchanged

   c. Write TSV to: Media/{map_name}_audio/{map_name}_inventory.tsv

2. Add JSON manifest output:
   - Same directory as TSV
   - Filename: {map_name}_manifest.json
   - Structure:

```json
{
  "map_name": "<safe_name>",
  "map_hash": "<if available>",
  "extracted_at": "<ISO timestamp>",
  "audio_references": [
    {
      "entity_hash": "0x80A71ABD",
      "entity_hash64": "0xF169FB151CE7AE33",
      "component_type": "SAudioPointComponent",
      "class_id": "0x8080666F",
      "event_widehash": "0x...",
      "event_id": "0x80D4E180",
      "wwise_bank": "...",
      "wem_files": ["0x12345678.wem"],
      "position": [123.45, -67.89, 0.0],
      "context": "Verity"
    }
  ]
}
```

3. Ensure:
   - One entry per AudioRef (NO deduplication)
   - Null/None fields serialized as null, not omitted
   - All hashes formatted consistently

DO NOT:
- Deduplicate entries
- Omit any fields
- Change existing TSV format (only add columns)
```

**Success Criteria:**
- [ ] TSV has all new columns
- [ ] JSON manifest written
- [ ] No deduplication
- [ ] Consistent hash formatting

---

### Phase 5: Validation (Agent: **Debug**)
**Objective:** Verify against Verity map

**Prompt for Kilo Code:**
```
VALIDATION TASK: Test against Verity map (0x80F1EE98)

1. Trigger dump_audio_references() for Verity bubble
   - Use existing UI button or hotkey (Shift+P)
   - Or call directly: dump_audio_references("Verity")

2. Check TSV output (Media/Verity_audio/Verity_inventory.tsv):
   - [ ] entity_hash column exists and varies per row
   - [ ] entity_hash64 column populated where available
   - [ ] NO FFFFFFFF values in any hash column
   - [ ] event_widehash column populated for all entries
   - [ ] position column populated where available
   - [ ] All existing columns still present

3. Check JSON manifest:
   - [ ] File exists: Media/Verity_audio/Verity_manifest.json
   - [ ] Structure matches specification
   - [ ] All entries from TSV present in JSON

4. Verify .wem files:
   - [ ] Files still extracted to Media/Verity_audio/
   - [ ] Count matches wem_count in TSV

5. Spot-check linkage:
   - Pick one entity_hash from TSV (e.g., 0x80A71ABD)
   - Find that entity in the Verity map
   - Confirm it has audio components (SAudioPointComponent or SAudioPathComponent)
   - Verify the .wem files listed for that entity_hash are correct

6. If ANY check fails:
   - STOP
   - Report which check failed and why
   - Do NOT attempt further modifications

DO NOT:
- Continue if validation fails
- Assume success without checking
```

**Success Criteria:**
- [ ] All validation checks pass
- [ ] No FFFFFFFF values
- [ ] Entity linkage verified

---

## 📊 VALIDATION QUERIES (Post-Implementation)

Once complete, enable these queries:

### 1. Find all audio for an entity (Bash)
```bash
grep "0x80A71ABD" verity_audio_inventory.tsv
```

### 2. Find entity for a .wem file (Bash)
```bash
grep "0x12345678.wem" verity_audio_inventory.tsv
```

### 3. Programmatic lookup (Python)
```python
import json
with open('verity_audio_manifest.json') as f:
    data = json.load(f)
for ref in data['audio_references']:
    if ref['entity_hash'] == '0x80A71ABD':
        print(f"Entity {ref['entity_hash']} has {len(ref['wem_files'])} audio files")
        for wem in ref['wem_files']:
            print(f"  - {wem}")
```

### 4. Find all entities with audio (Bash)
```bash
cut -d$'\t' -f1 verity_audio_inventory.tsv | sort -u
```

---

## 🎓 LESSONS FROM FAILED ATTEMPTS

---

### Session 5 Post-Mortem

| Issue                    | Root Cause                                    | Prevention in This Plan                     |
| ------------------------ | --------------------------------------------- | ------------------------------------------- |
| Scope creep into UI      | Focused on Panel::bottom instead of data flow | Explicit file modification limits (3 files) |
| Token bloat (9.7M input) | No exclusions for binary files                | `.kilocodeignore` file provided             |
| Wrong problem focus      | Solved UI visibility, not audio linkage       | Explicit focus on entity linkage            |
| Work failed/paused       | No clear success criteria                     | Concrete validation checks defined          |

### Session 4 Incomplete Work

| Issue                  | Status                   | Resolution in This Plan         |
| ---------------------- | ------------------------ | ------------------------------- |
| Audio extraction logic | ✅ Complete               | Preserved                       |
| Button placement       | ❌ Failed (Panel::bottom) | User moved to scene toolbar (✅) |
| Entity linkage         | ❌ Missing                | **Primary focus of this plan**  |
| WideHash Hash64        | ❌ Unresolved             | Phase 3 explicitly addresses    |
| event_widehash export  | ❌ Missing                | Phase 4 explicitly addresses    |

---

## 📄 SUPPORTING FILES

---

### File: `.kilocodeignore` (Create in project root)
```text
# Binary output directories
Media/
output/
target/

# Binary files
*.pkg
*.wem
*.fbx
*.bin

# Logs and temporary files
*.log
.kilo/
*.tmp
*.swp
.DS_Store

# IDE-specific
.idea/
.vscode/
*.sublime-workspace
*.sublime-project
```

---

### File: `.kilocode/settings.json` (Optional - for status polling)
```json
{
  "checkpoints": {
    "enabled": true,
    "interval": 5
  },
  "context": {
    "maxTokens": 128000,
    "compactionThreshold": 0.7
  }
}
```

---

## ✅ SUCCESS DEFINITION

The project is **complete** when ALL of the following are true:

### Technical Completion
- [ ] `entity_hash` field added to AudioRef
- [ ] `entity_hash64` field added to AudioRef
- [ ] `position` field added to AudioRef
- [ ] Entity hash threaded through entire call chain
- [ ] WideHash::Hash64 handled without FFFFFFFF
- [ ] event_widehash exported to TSV
- [ ] Code compiles without errors
- [ ] No new files created (unless absolutely necessary)

### Functional Completion
- [ ] TSV shows varying `entity_hash` per row
- [ ] No FFFFFFFF values for valid events
- [ ] Each .wem can be traced back to its entity
- [ ] Hotkey (Shift+P) still works
- [ ] UI button still works
- [ ] All existing .wem extraction still works

### Validation Completion
- [ ] Verity map dump produces correct linkage
- [ ] All known component IDs appear (0x8080666D, 0x8080666F, etc.)
- [ ] All known Wwise event IDs resolved or stored
- [ ] TSV and JSON manifest both valid

---

## 🔗 REFERENCES

### Session Files (From HANDOFF_FINAL.md)
- **session1.json:** Cargo features analysis, wwise SDK blocked, direct .wem extraction workaround confirmed
- **session4.json:** Audio investigation, hash mappings confirmed, component IDs verified, code changes in activity.rs/map.rs/world/pattern.rs/data/map.rs
- **session5.json:** UI button issue (resolved by user moving to scene toolbar)

### Code Locations (From User Conversations)
- `dump_audio_references()`: `alkahest_data::map`
- AudioRef struct: `alkahest_data::map`
- AUDIO_REFERENCES static: `alkahest_data::map`
- AudioRef push site: Search for `refs.push(AudioRef { ... })`
- WideHash definition: `pub enum WideHash { Hash32(TagHash), Hash64(TagHash64) }`
- Current issue: `entity_context: context.to_string()` where `context` = hardcoded map name

### Validated Hashes (From Session 4)
- **Raid:** 0x80F228FA (Salvation's Edge / schism_spire.raid_splinter)
- **Bubble:** 0x80F1EE98 (Verity)
- **Map:** 0x80F1EEC8 (Verity)
- **Sub-maps:** 80F1EE28, 80AED34B, 80F1B516, 80F1EE7A, 80F1EE90, 80F1EE94, 80F1EE75, 80F1EE89, 80F1B022

### Component Type Hashes (From Session 4)
- 0x8080666D = SAudioPathComponent (ambient/path audio)
- 0x8080666F = SAudioPointComponent (point/spawn audio)
- 0x80809738 = SWwiseEvent (Wwise event container)
- 0x80808CB5 = SRespawnPointsComponent

---
```

---
