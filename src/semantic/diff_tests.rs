#[cfg(test)]
mod tests {
    use crate::models::{DeltaKind, SemanticSymbol};
    use crate::semantic::diff::SemanticDiffer;

    fn create_mock_symbol(name: &str, hash: &str) -> SemanticSymbol {
        SemanticSymbol {
            id: 0,
            name: name.to_string(),
            kind: "function".to_string(),
            scope: None,
            snapshot_id: 0,
            chunk_hash: "".to_string(),
            structural_hash: hash.to_string(),
            start_line: 0,
            end_line: 0,
            start_byte: 0,
            end_byte: 0,
            parent_id: None,
        }
    }

    #[test]
    fn test_diff_modified() {
        let prev = vec![create_mock_symbol("func1", "hash1")];
        let curr = vec![create_mock_symbol("func1", "hash2")];

        let deltas = SemanticDiffer::compare(&prev, &curr, Some(1), 2);

        assert_eq!(deltas.len(), 1);
        assert!(matches!(deltas[0].kind, DeltaKind::Modified));
        assert_eq!(deltas[0].symbol_name, "func1");
    }

    #[test]
    fn test_diff_renamed() {
        // Same hash, different name
        let prev = vec![create_mock_symbol("old_name", "same_hash")];
        let curr = vec![create_mock_symbol("new_name", "same_hash")];

        let deltas = SemanticDiffer::compare(&prev, &curr, Some(1), 2);

        assert_eq!(deltas.len(), 1);
        assert!(matches!(deltas[0].kind, DeltaKind::Renamed));
        assert_eq!(deltas[0].symbol_name, "old_name");
        assert_eq!(deltas[0].new_name, Some("new_name".to_string()));
    }

    #[test]
    fn test_diff_added_deleted() {
        let prev = vec![create_mock_symbol("deleted_func", "h1")];
        let curr = vec![create_mock_symbol("added_func", "h2")];

        let deltas = SemanticDiffer::compare(&prev, &curr, Some(1), 2);

        assert_eq!(deltas.len(), 2);
        let has_added = deltas
            .iter()
            .any(|d| matches!(d.kind, DeltaKind::Added) && d.symbol_name == "added_func");
        let has_deleted = deltas
            .iter()
            .any(|d| matches!(d.kind, DeltaKind::Deleted) && d.symbol_name == "deleted_func");

        assert!(has_added);
        assert!(has_deleted);
    }
}
