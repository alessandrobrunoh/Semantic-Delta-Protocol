use crate::models::{RecordKind, SemanticRecord, SemanticSymbol};
use std::collections::HashMap;

pub struct SemanticDiffer;

impl SemanticDiffer {
    pub fn compare(
        prev_symbols: &[SemanticSymbol],
        curr_symbols: &[SemanticSymbol],
        from_snapshot_id: Option<i64>,
        to_snapshot_id: i64,
    ) -> Vec<SemanticRecord> {
        let mut records = Vec::new();

        // Maps for efficient lookups
        let prev_map: HashMap<String, &SemanticSymbol> =
            prev_symbols.iter().map(|s| (s.name.clone(), s)).collect();

        // 1. Identify Modified and Added
        let mut matched_prev = std::collections::HashSet::new();

        for curr in curr_symbols {
            if let Some(prev) = prev_map.get(&curr.name) {
                matched_prev.insert(curr.name.clone());
                if prev.structural_hash != curr.structural_hash {
                    records.push(SemanticRecord {
                        id: 0,
                        project_id: None,
                        from_snapshot_id,
                        to_snapshot_id,
                        symbol_name: curr.name.clone(),
                        new_name: None,
                        kind: RecordKind::Modified,
                        structural_hash: curr.structural_hash.clone(),
                    });
                }
            } else {
                // Potential rename or purely added
                records.push(SemanticRecord {
                    id: 0,
                    project_id: None,
                    from_snapshot_id,
                    to_snapshot_id,
                    symbol_name: curr.name.clone(),
                    new_name: None,
                    kind: RecordKind::Added,
                    structural_hash: curr.structural_hash.clone(),
                });
            }
        }

        // 2. Identify Deleted and Renamed
        for prev in prev_symbols {
            if !matched_prev.contains(&prev.name) {
                // Check if this structural_hash exists in the Added records (Rename detection)
                let mut found_rename = false;

                // Sort records to ensure deterministic rename matching if multiple symbols have same hash
                for record in records.iter_mut() {
                    if matches!(record.kind, RecordKind::Added)
                        && record.structural_hash == prev.structural_hash
                    {
                        record.kind = RecordKind::Renamed;
                        record.new_name = Some(record.symbol_name.clone());
                        record.symbol_name = prev.name.clone();
                        found_rename = true;
                        break;
                    }
                }

                if !found_rename {
                    records.push(SemanticRecord {
                        id: 0,
                        project_id: None,
                        from_snapshot_id,
                        to_snapshot_id,
                        symbol_name: prev.name.clone(),
                        new_name: None,
                        kind: RecordKind::Deleted,
                        structural_hash: prev.structural_hash.clone(),
                    });
                }
            }
        }

        records
    }
}
