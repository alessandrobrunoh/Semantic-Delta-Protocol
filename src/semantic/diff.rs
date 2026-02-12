use crate::models::{DeltaKind, SemanticDelta, SemanticSymbol};
use std::collections::HashMap;

pub struct SemanticDiffer;

impl SemanticDiffer {
    pub fn compare(
        prev_symbols: &[SemanticSymbol],
        curr_symbols: &[SemanticSymbol],
        from_snapshot_id: Option<i64>,
        to_snapshot_id: i64,
    ) -> Vec<SemanticDelta> {
        let mut deltas = Vec::new();

        // Maps for efficient lookups
        let prev_map: HashMap<String, &SemanticSymbol> =
            prev_symbols.iter().map(|s| (s.name.clone(), s)).collect();

        // 1. Identify Modified and Added
        let mut matched_prev = std::collections::HashSet::new();

        for curr in curr_symbols {
            if let Some(prev) = prev_map.get(&curr.name) {
                matched_prev.insert(curr.name.clone());
                if prev.structural_hash != curr.structural_hash {
                    deltas.push(SemanticDelta {
                        id: 0,
                        project_id: None,
                        from_snapshot_id,
                        to_snapshot_id,
                        symbol_name: curr.name.clone(),
                        new_name: None,
                        kind: DeltaKind::Modified,
                        structural_hash: curr.structural_hash.clone(),
                    });
                }
            } else {
                // Potential rename or purely added
                deltas.push(SemanticDelta {
                    id: 0,
                    project_id: None,
                    from_snapshot_id,
                    to_snapshot_id,
                    symbol_name: curr.name.clone(),
                    new_name: None,
                    kind: DeltaKind::Added,
                    structural_hash: curr.structural_hash.clone(),
                });
            }
        }

        // 2. Identify Deleted and Renamed
        for prev in prev_symbols {
            if !matched_prev.contains(&prev.name) {
                // Check if this structural_hash exists in the Added deltas (Rename detection)
                let mut found_rename = false;

                // Sort deltas to ensure deterministic rename matching if multiple symbols have same hash
                for delta in deltas.iter_mut() {
                    if matches!(delta.kind, DeltaKind::Added)
                        && delta.structural_hash == prev.structural_hash
                    {
                        delta.kind = DeltaKind::Renamed;
                        delta.new_name = Some(delta.symbol_name.clone());
                        delta.symbol_name = prev.name.clone();
                        found_rename = true;
                        break;
                    }
                }

                if !found_rename {
                    deltas.push(SemanticDelta {
                        id: 0,
                        project_id: None,
                        from_snapshot_id,
                        to_snapshot_id,
                        symbol_name: prev.name.clone(),
                        new_name: None,
                        kind: DeltaKind::Deleted,
                        structural_hash: prev.structural_hash.clone(),
                    });
                }
            }
        }

        deltas
    }
}
