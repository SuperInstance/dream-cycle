# dream-cycle

> **When the cortex sleeps, it dreams. Consolidation, creativity, anomaly detection.**

[![crates.io](https://img.shields.io/crates/v/dream-cycle.svg)](https://crates.io/crates/dream-cycle)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

The Dream Cycle engine for agent cognition. When agents are idle, the cortex enters a dream state — replaying memories, consolidating patterns, generating creative associations, and detecting anomalies.

## The Idea

Human cognition isn't all wakefulness. During sleep, the brain:
1. **Consolidates** memories (replay → compress → store)
2. **Generates** creative associations (dream-like narrative)
3. **Detects** anomalies (nightmares → alert system)

`dream-cycle` implements this for AI agents. When the cortex is idle, it dreams — and wakes up smarter.

## Quick Start

```rust
use dream_cycle::{DreamState, MemoryConsolidator};

// Enter dream state
let state = DreamState::new(DreamPhase::REM, 3);
assert_eq!(state.depth(), 3);
```

## Part of [Exocortex](https://github.com/SuperInstance/exocortex)

The Dream Cycle is a core subsystem: idle cortex cycles through REM/NREM phases, replaying the day's events and consolidating patterns.

## License

MIT © [SuperInstance](https://github.com/SuperInstance)
