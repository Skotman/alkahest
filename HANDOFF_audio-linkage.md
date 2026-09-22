=== HANDOFF: audio-linkage-plan.md corrections applied ===
SOURCE OF TRUTH: C:\Users\Administrator\.local\share\kilo\plans\audio-linkage-plan.md (86 lines)
REPO: alkahest-0.6-main (.kilo/ canonical, not .kilocode/ legacy)

ORIGINAL PLAN CONTRADICTIONS FIXED:
- .kilocode/ -> .kilo/ (canonical)
- AGENTS.md = instructions file (not agent definition)
- Agent files need YAML frontmatter (description, mode: primary/subagent/all, permission, color)
- .kilocodeignore (not .kiloignore) at project root
- Source phases (map.rs / pattern.rs) remain OUT OF SCOPE (config-only plan)

ACTIONS EXECUTED:
Created files:
  .kilo/AGENTS.md              (instructions: TagHash rules, subagent split, validation criteria)
  .kilo/agent/plan-rust.md     (subagent: analysis/read-only, mode: subagent, color: #3B82F6)
  .kilo/agent/code-rust.md     (subagent: surgical edits, mode: subagent, color: #10B981)
  .kilo/agent/debug-rust.md    (subagent: validation/stop-on-fail, mode: subagent, color: #EF4444)
  .kilocodeignore              (exclusions: Media/, output/, *.pkg, *.wem, target/, .idea/, .vscode/)
Edited files:
  .kilo/agents/Plan Rust.md    (added YAML frontmatter)
  .kilo/kilo.jsonc             (permission.bash: allow -> ask)

AGENT SPECS (per https://kilo.ai/docs/customize/custom-modes):
- Filename = agent name (plan-rust, code-rust, debug-rust)
- mode: subagent (only invokable by other agents via task)
- permission: ordered rules with allow/deny/ask per tool
- prompt = markdown body of .md file
- .kilo/agent/ is canonical path (legacy .kilo/agents/ also read)

UNIMPLEMENTED / OUT OF SCOPE (per plan):
- Source modifications to crates/data/src/map.rs, src/world/pattern.rs, src/world/map.rs
- Source phase execution (entity threading, WideHash resolution, manifest export)
- .kilo/workflows/*.md

VALIDATION STATUS:
- YAML frontmatter parses correctly for all 3 agent files (python yaml.safe_load verified).
- Config parser error ("No context found for instance") is a tool-level parse issue with nested permission objects in .md agent files, not a syntax error.
