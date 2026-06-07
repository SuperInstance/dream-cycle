# dream-cycle

> **When the cortex sleeps, it dreams. Consolidation, creativity, anomaly detection.**

[![crates.io](https://img.shields.io/crates/v/dream-cycle.svg)](https://crates.io/crates/dream-cycle)
[![docs.rs](https://docs.rs/dream-cycle/badge.svg)](https://docs.rs/dream-cycle)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library implementing the Dream Cycle engine for agent cognition — modeled on mammalian sleep architecture. When agents are idle, the cortex enters dream states: replaying memories, consolidating patterns through compression, generating surreal creative associations, and detecting anomalies (nightmares). Wakes up smarter than it went to sleep.

---

## Table of Contents

- [What is the Dream Cycle?](#what-is-the-dream-cycle)
- [Why Does This Matter?](#why-does-this-matter)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Mathematical/Technical Background](#mathematicaltechnical-background)
- [Installation](#installation)
- [Related Crates](#related-crates)
- [License](#license)

---

## What is the Dream Cycle?

The human brain doesn't process information only during wakefulness. During sleep, it cycles through distinct phases — each with a specific cognitive function:

```
┌─────────────────────────────────────────────────────────┐
│              90-minute Sleep Cycle                       │
│                                                         │
│  NREM Light ──► NREM Deep ──► REM ──► NREM Light ──►…  │
│  (scanning)    (consolidating)  (creative)               │
│                                                         │
│  Depth: 25%        75%          50%                      │
│  Brain: theta      delta        beta (active)            │
│  Func:  sort       compress     recombine                │
└─────────────────────────────────────────────────────────┘
```

This library implements the same architecture for AI agents:

1. **NREM Light** — Shallow memory scanning, sorting events by salience
2. **NREM Deep** — Slow-wave consolidation, compressing memories by merging similar fragments
3. **REM** — Rapid recombination, generating surreal narratives from memory fragments
4. **Nightmare Detection** — Anomalies that exceed a surprise threshold trigger alerts

The key insight: **idle time is not wasted time**. When an agent has nothing urgent to do, it should dream — processing accumulated experiences and preparing for future challenges.

## Why Does This Matter?

**For long-running agents**: An agent that runs 24/7 accumulates vast experience. Without consolidation, memory grows unbounded. Dream cycling compresses memories while preserving important patterns.

**For creativity**: REM-style recombination of unrelated memories produces novel associations — the computational equivalent of "sleeping on a problem" and waking with a solution.

**For anomaly detection**: Nightmares (memories with unusually high surprise) flag experiences that don't fit existing patterns — potential threats, opportunities, or model failures.

**For resource efficiency**: Memory compression during deep NREM reduces storage requirements while maintaining the most salient information. The agent gets smarter while using less memory.

## Architecture

```
dream-cycle
│
├── DreamPhase (enum)          ← Sleep stage
│   ├── NremLight                  Shallow scanning
│   ├── NremDeep                   Deep consolidation
│   └── Rem                        Creative recombination
│
├── DreamState                 ← Current dream state
│   ├── new()                      Start at NREM Light, depth 0
│   ├── advance()                  Phase transition: Light→Deep→REM→Light
│   └── is_idle_threshold(events)  Should the agent start dreaming?
│
├── MemoryFragment              ← Unit of experience
│   ├── id: u64                    Unique identifier
│   ├── content: String            What happened
│   ├── salience: f64             How important (0-1)
│   └── timestamp: u64            When it happened
│
├── MemoryConsolidator          ← NREM Deep: compress memories
│   ├── new()                      Empty consolidator
│   ├── load(fragments)            Load memory fragments
│   ├── consolidate()              Merge similar memories → returns count
│   └── compressed()               Get compressed fragments
│
├── SurrealNarrative            ← REM: creative recombination
│   ├── generate(fragments)        Create narrative from fragments
│   ├── len() / is_empty()         Narrative length
│   └── (internally: chains fragments via shared keywords)
│
├── NightmareDetector           ← Anomaly detection
│   ├── new(threshold)             Set surprise threshold
│   ├── detect(fragments)          Find anomalous fragments
│   └── is_nightmare(fragments)    Any fragment exceeds threshold?
│
└── DreamCycleOrchestrator      ← The full cycle manager
    ├── new()                      Start orchestrator
    ├── tick(recent_events)        Advance one tick (returns true if dreaming)
    ├── state()                    Current DreamState
    └── cycles_completed()         Total completed cycles
```

## Quick Start

```rust
use dream_cycle::{
    DreamState, DreamPhase,
    MemoryFragment, MemoryConsolidator,
    SurrealNarrative, NightmareDetector,
    DreamCycleOrchestrator,
};

// The orchestrator manages the full dream cycle
let mut orchestrator = DreamCycleOrchestrator::new();

// Tick: if idle, the agent starts dreaming
let dreaming = orchestrator.tick(1); // only 1 recent event → idle
println!("Dreaming: {}, State: {:?}", dreaming, orchestrator.state());

// Load some memories
let memories = vec![
    MemoryFragment { id: 1, content: "User asked about weather".into(), salience: 0.3, timestamp: 1000 },
    MemoryFragment { id: 2, content: "Detected anomaly in sensor data".into(), salience: 0.9, timestamp: 1001 },
    MemoryFragment { id: 3, content: "User asked about weather again".into(), salience: 0.2, timestamp: 1002 },
    MemoryFragment { id: 4, content: "System recovered from error".into(), salience: 0.7, timestamp: 1003 },
];

// Consolidate: merge similar memories
let mut consolidator = MemoryConsolidator::new();
consolidator.load(memories.clone());
let merged_count = consolidator.consolidate();
println!("Merged {} similar fragments", merged_count);
println!("Compressed: {} fragments", consolidator.compressed().len());

// Generate a surreal narrative (REM phase)
let narrative = SurrealNarrative::generate(&memories);
println!("Dream narrative: {} elements", narrative.len());

// Detect nightmares (anomalies)
let detector = NightmareDetector::new(0.8); // threshold for surprise
let nightmares = detector.detect(&memories);
println!("Nightmare fragments: {:?}", nightmares); // [2] (salience 0.9)

let is_nightmare = detector.is_nightmare(&memories);
println!("Contains nightmares: {}", is_nightmare); // true

// Advance the dream cycle manually
let mut state = DreamState::new();
println!("Phase: {:?}, Depth: {}", state.phase, state.depth);
state.advance(); // NremLight → NremDeep
state.advance(); // NremDeep → Rem
state.advance(); // Rem → NremLight (cycle complete)
```

## API Reference

### DreamPhase & DreamState

| Method | Returns | Description |
|--------|---------|-------------|
| `DreamState::new()` | `DreamState` | NremLight, depth 0 |
| `state.advance()` | `()` | Phase transition (Light→Deep→REM→Light) |
| `DreamState::is_idle_threshold(events)` | `bool` | events < 3 → idle |

### MemoryConsolidator

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Self` | Empty consolidator |
| `load(fragments)` | `()` | Load memory fragments |
| `consolidate()` | `usize` | Merge similar, return count of merges |
| `compressed()` | `&[MemoryFragment]` | Get compressed fragments |

### SurrealNarrative

| Method | Returns | Description |
|--------|---------|-------------|
| `generate(fragments)` | `SurrealNarrative` | Create dream narrative |
| `len()` | `usize` | Narrative length |
| `is_empty()` | `bool` | No content |

### NightmareDetector

| Method | Returns | Description |
|--------|---------|-------------|
| `new(threshold)` | `Self` | Set surprise threshold (0-1) |
| `default_threshold()` | `f64` | 0.75 |
| `detect(fragments)` | `Vec<u64>` | IDs of anomalous fragments |
| `is_nightmare(fragments)` | `bool` | Any fragment exceeds threshold? |

### DreamCycleOrchestrator

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Self` | Start orchestrator |
| `tick(recent_events)` | `bool` | Advance cycle; true if dreaming |
| `state()` | `&DreamState` | Current phase and depth |
| `cycles_completed()` | `u32` | Total completed cycles |

## Mathematical/Technical Background

### Sleep Architecture

Mammalian sleep follows a ~90-minute ultradian rhythm:

```
NREM Stage 1 (Light) → NREM Stage 3 (Deep) → REM → repeat
     5% of cycle          50% of cycle        25%    20%
```

The library simplifies to three phases with depth progression:
- Depth increases by 25 per advance (0 → 25 → 50 → reset)
- REM phase triggers full depth reset on next cycle

### Memory Consolidation

Consolidation merges fragments with overlapping content by averaging salience and combining content:

```
merged_salience = (s₁ + s₂) / 2
merged_content = f"{content₁} + {content₂}"
```

This mirrors hippocampal replay, where the hippocampus replays the day's experiences to the neocortex for long-term storage — compressing detailed episodic memories into semantic knowledge.

### Surreal Narrative Generation

REM dreams chain memory fragments by association:

```
fragment₁ ──keyword_link──→ fragment₂ ──keyword_link──→ fragment₃
```

This is computational **dreamwork** — following Freud's associations but algorithmically. The resulting narrative chains are genuinely novel recombinations that can spark creative insights.

### Nightmare Detection

Fragments with salience above threshold θ are flagged:

```
nightmare(f) = f.salience > θ
```

This mirrors the amygdala's role in REM sleep: processing emotional/fearful memories. Computationally, high-salience fragments indicate experiences that don't fit existing patterns — anomalies worth investigating.

## Installation

```bash
cargo add dream-cycle
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
dream-cycle = "0.1"
```

## Related Crates

Part of the **SuperInstance Exocortex** ecosystem:

- **[free-energy](https://github.com/SuperInstance/free-energy)** — Variational free energy computation
- **[active-inference](https://github.com/SuperInstance/active-inference)** — Action as surprise minimization
- **[forgetting-curve](https://github.com/SuperInstance/forgetting-curve)** — Ebbinghaus forgetting and spaced repetition
- **[shadow-cathedral](https://github.com/SuperInstance/shadow-cathedral)** — 3-layer shadow rendering pipeline
- **[cortex-bus-protocol](https://github.com/SuperInstance/cortex-bus-protocol)** — CQRS event bus for agents
- **[persistent-agent](https://github.com/SuperInstance/persistent-agent)** — Topological fingerprints for agents

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project.
