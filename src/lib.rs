//! # dream-cycle
//!
//! Agent dream consolidation engine with REM/NREM cycle simulation,
//! memory replay, surreal narrative generation, and nightmare detection.

use std::fmt;

/// Phase of a dream cycle, analogous to mammalian sleep stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DreamPhase {
    /// Light rest — shallow memory scanning.
    NremLight,
    /// Deep rest — slow-wave consolidation.
    NremDeep,
    /// Rapid replay — vivid memory recombination.
    Rem,
}

/// Current state of a dream cycle.
#[derive(Debug, Clone, PartialEq)]
pub struct DreamState {
    /// Current phase.
    pub phase: DreamPhase,
    /// Depth level 0-100, higher = deeper into the dream.
    pub depth: u8,
}

impl DreamState {
    /// Create a new dream state at light NREM with depth 0.
    pub fn new() -> Self {
        Self {
            phase: DreamPhase::NremLight,
            depth: 0,
        }
    }

    /// Transition to the next phase in the cycle.
    /// NremLight → NremDeep → Rem → NremLight (depth resets).
    pub fn advance(&mut self) {
        self.depth = self.depth.saturating_add(25).min(100);
        match self.phase {
            DreamPhase::NremLight => self.phase = DreamPhase::NremDeep,
            DreamPhase::NremDeep => self.phase = DreamPhase::Rem,
            DreamPhase::Rem => {
                self.phase = DreamPhase::NremLight;
                self.depth = 0;
            }
        }
    }

    /// Check whether the cortex is considered idle enough for dreaming.
    pub fn is_idle_threshold(recent_event_count: usize) -> bool {
        recent_event_count < 3
    }
}

impl Default for DreamState {
    fn default() -> Self {
        Self::new()
    }
}

/// A fragment of memory that can be replayed during a dream.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryFragment {
    /// Unique identifier.
    pub id: u64,
    /// Content tag (e.g. "conversation", "error", "insight").
    pub tag: String,
    /// Emotional weight 0.0–1.0.
    pub weight: f64,
}

/// Consolidates memories by replaying and compressing them during dream cycles.
#[derive(Debug, Clone)]
pub struct MemoryConsolidator {
    fragments: Vec<MemoryFragment>,
    compressed: Vec<MemoryFragment>,
}

impl MemoryConsolidator {
    /// Create an empty consolidator.
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
            compressed: Vec::new(),
        }
    }

    /// Load fragments for replay.
    pub fn load(&mut self, fragments: Vec<MemoryFragment>) {
        self.fragments = fragments;
    }

    /// Run one consolidation pass: compress fragments by merging same-tag
    /// pairs and summing their weights (capped at 1.0).
    pub fn consolidate(&mut self) -> usize {
        let mut tag_map: std::collections::HashMap<String, (u64, f64)> =
            std::collections::HashMap::new();
        let mut next_id: u64 = 1000;
        for f in self.fragments.drain(..) {
            let entry = tag_map.entry(f.tag.clone()).or_insert((next_id, 0.0));
            if entry.0 == next_id {
                next_id += 1;
            }
            entry.1 = (entry.1 + f.weight).min(1.0);
        }
        self.compressed = tag_map
            .into_iter()
            .map(|(tag, (id, weight))| MemoryFragment { id, tag, weight })
            .collect();
        self.compressed.len()
    }

    /// Retrieve the compressed fragments.
    pub fn compressed(&self) -> &[MemoryFragment] {
        &self.compressed
    }
}

impl Default for MemoryConsolidator {
    fn default() -> Self {
        Self::new()
    }
}

/// A surreal dream narrative built from memory fragments.
#[derive(Debug, Clone, PartialEq)]
pub struct SurrealNarrative {
    /// Ordered sequence of dream scenes.
    pub scenes: Vec<String>,
}

impl SurrealNarrative {
    /// Generate a dream-like narrative from memory fragments.
    ///
    /// Each fragment becomes a surreal scene string that blends its tag
    /// and emotional weight into a metaphor.
    pub fn generate(fragments: &[MemoryFragment]) -> Self {
        let scenes = fragments
            .iter()
            .map(|f| {
                let intensity = if f.weight > 0.7 {
                    "vividly"
                } else if f.weight > 0.4 {
                    "softly"
                } else {
                    "faintly"
                };
                format!(
                    "A {} shimmering vision of {} echoes through the dreamscape",
                    intensity, f.tag
                )
            })
            .collect();
        Self { scenes }
    }

    /// Return the number of scenes.
    pub fn len(&self) -> usize {
        self.scenes.len()
    }

    /// Check if the narrative is empty.
    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }
}

impl fmt::Display for SurrealNarrative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, scene) in self.scenes.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "Scene {}: {}", i + 1, scene)?;
        }
        Ok(())
    }
}

/// Orchestrates dream cycles, scheduling them when the cortex is idle.
#[derive(Debug)]
pub struct DreamCycleOrchestrator {
    state: DreamState,
    idle_count: usize,
    cycles_completed: u32,
}

impl DreamCycleOrchestrator {
    /// Create a new orchestrator.
    pub fn new() -> Self {
        Self {
            state: DreamState::new(),
            idle_count: 0,
            cycles_completed: 0,
        }
    }

    /// Tick: report the number of recent external events.
    /// Returns `true` when a full cycle (NremLight → Rem) completes.
    pub fn tick(&mut self, recent_events: usize) -> bool {
        if DreamState::is_idle_threshold(recent_events) {
            self.idle_count += 1;
        } else {
            self.idle_count = 0;
        }
        // Need at least 2 idle ticks to start dreaming.
        if self.idle_count >= 2 {
            self.state.advance();
            if self.state.phase == DreamPhase::NremLight && self.state.depth == 0 {
                self.cycles_completed += 1;
                return true;
            }
        }
        false
    }

    /// Current dream state.
    pub fn state(&self) -> &DreamState {
        &self.state
    }

    /// Number of full cycles completed.
    pub fn cycles_completed(&self) -> u32 {
        self.cycles_completed
    }
}

impl Default for DreamCycleOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects anomalies (nightmares) in dream content based on emotional weight.
#[derive(Debug, Clone)]
pub struct NightmareDetector {
    /// Threshold above which a memory fragment is flagged as nightmarish.
    pub threshold: f64,
}

impl NightmareDetector {
    /// Create a detector with the given weight threshold.
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Default threshold of 0.85.
    pub fn default_threshold() -> f64 {
        0.85
    }

    /// Analyse a slice of fragments; returns IDs of fragments that trigger
    /// a nightmare alert.
    pub fn detect(&self, fragments: &[MemoryFragment]) -> Vec<u64> {
        fragments
            .iter()
            .filter(|f| f.weight >= self.threshold)
            .map(|f| f.id)
            .collect()
    }

    /// Returns `true` if any fragment exceeds the threshold.
    pub fn is_nightmare(&self, fragments: &[MemoryFragment]) -> bool {
        fragments.iter().any(|f| f.weight >= self.threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dream_state_starts_nrem_light() {
        let ds = DreamState::new();
        assert_eq!(ds.phase, DreamPhase::NremLight);
        assert_eq!(ds.depth, 0);
    }

    #[test]
    fn dream_state_advances_through_phases() {
        let mut ds = DreamState::new();
        ds.advance();
        assert_eq!(ds.phase, DreamPhase::NremDeep);
        assert_eq!(ds.depth, 25);
        ds.advance();
        assert_eq!(ds.phase, DreamPhase::Rem);
        assert_eq!(ds.depth, 50);
        ds.advance();
        assert_eq!(ds.phase, DreamPhase::NremLight);
        assert_eq!(ds.depth, 0);
    }

    #[test]
    fn dream_state_depth_caps_at_100() {
        let mut ds = DreamState::new();
        ds.depth = 90;
        ds.advance();
        assert_eq!(ds.depth, 100);
    }

    #[test]
    fn idle_threshold() {
        assert!(DreamState::is_idle_threshold(0));
        assert!(DreamState::is_idle_threshold(2));
        assert!(!DreamState::is_idle_threshold(5));
    }

    #[test]
    fn consolidator_merges_same_tag() {
        let mut c = MemoryConsolidator::new();
        c.load(vec![
            MemoryFragment { id: 1, tag: "error".into(), weight: 0.3 },
            MemoryFragment { id: 2, tag: "error".into(), weight: 0.4 },
            MemoryFragment { id: 3, tag: "insight".into(), weight: 0.6 },
        ]);
        let count = c.consolidate();
        assert_eq!(count, 2);
        let err = c.compressed().iter().find(|f| f.tag == "error").unwrap();
        assert!((err.weight - 0.7).abs() < 1e-9);
    }

    #[test]
    fn consolidator_weight_caps_at_one() {
        let mut c = MemoryConsolidator::new();
        c.load(vec![
            MemoryFragment { id: 1, tag: "a".into(), weight: 0.8 },
            MemoryFragment { id: 2, tag: "a".into(), weight: 0.6 },
        ]);
        c.consolidate();
        let a = c.compressed().iter().find(|f| f.tag == "a").unwrap();
        assert!((a.weight - 1.0).abs() < 1e-9);
    }

    #[test]
    fn consolidator_empty() {
        let mut c = MemoryConsolidator::new();
        assert_eq!(c.consolidate(), 0);
    }

    #[test]
    fn surreal_narrative_generates_scenes() {
        let frags = vec![MemoryFragment { id: 1, tag: "ocean".into(), weight: 0.8 }];
        let narr = SurrealNarrative::generate(&frags);
        assert_eq!(narr.len(), 1);
        assert!(narr.scenes[0].contains("vividly"));
        assert!(narr.scenes[0].contains("ocean"));
    }

    #[test]
    fn surreal_narrative_intensity_levels() {
        let low = SurrealNarrative::generate(&[MemoryFragment { id: 1, tag: "x".into(), weight: 0.1 }]);
        assert!(low.scenes[0].contains("faintly"));
        let mid = SurrealNarrative::generate(&[MemoryFragment { id: 2, tag: "x".into(), weight: 0.5 }]);
        assert!(mid.scenes[0].contains("softly"));
    }

    #[test]
    fn surreal_narrative_display() {
        let narr = SurrealNarrative::generate(&[MemoryFragment { id: 1, tag: "sky".into(), weight: 0.9 }]);
        let s = format!("{}", narr);
        assert!(s.starts_with("Scene 1:"));
    }

    #[test]
    fn orchestrator_completes_cycle() {
        let mut orch = DreamCycleOrchestrator::new();
        // 6 idle ticks: advance through 3 phases (NremLight→NremDeep→Rem→NremLight)
        let mut completed = false;
        for _ in 0..6 {
            if orch.tick(0) {
                completed = true;
            }
        }
        assert!(completed);
        assert_eq!(orch.cycles_completed(), 1);
    }

    #[test]
    fn orchestrator_no_cycle_when_busy() {
        let mut orch = DreamCycleOrchestrator::new();
        for _ in 0..10 {
            orch.tick(10); // always busy
        }
        assert_eq!(orch.cycles_completed(), 0);
    }

    #[test]
    fn nightmare_detector_flags_high_weight() {
        let det = NightmareDetector::new(0.85);
        let frags = vec![
            MemoryFragment { id: 1, tag: "a".into(), weight: 0.9 },
            MemoryFragment { id: 2, tag: "b".into(), weight: 0.5 },
        ];
        let ids = det.detect(&frags);
        assert_eq!(ids, vec![1]);
        assert!(det.is_nightmare(&frags));
    }

    #[test]
    fn nightmare_detector_no_false_positive() {
        let det = NightmareDetector::new(0.85);
        let frags = vec![MemoryFragment { id: 1, tag: "a".into(), weight: 0.3 }];
        assert!(!det.is_nightmare(&frags));
    }
}
