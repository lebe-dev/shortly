### PROJECT STACK ###

- Backend: Rust
  - Don't use async-trait

You run in an environment where `ast-grep` is available; whenever a search requires syntax-aware or structural matching, 
default to `ast-grep --lang rust -p '<pattern>'` (or set `--lang` appropriately, `ts` for typescript) and avoid falling back to text-only tools like `rg` or `grep` unless I explicitly request a plain-text search.|

Use "cargo build --bin webapp" to build.

