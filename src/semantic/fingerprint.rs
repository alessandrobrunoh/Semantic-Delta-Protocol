use blake3::Hasher;
use tree_sitter::{Node, TreeCursor};

/// Generates a structural hash of a code block, ignoring variable names and literals
/// to identify logically equivalent code despite cosmetic changes.
pub struct StructuralFingerprinter;

impl StructuralFingerprinter {
    /// Computes a structural hash for the given Tree-Sitter node.
    pub fn compute(node: Node, _source: &[u8]) -> String {
        let mut hasher = Hasher::new();
        let mut cursor = node.walk();

        Self::traverse(&mut cursor, &mut hasher);

        hasher.finalize().to_hex().to_string()
    }

    fn traverse(cursor: &mut TreeCursor, hasher: &mut Hasher) {
        let mut reached_root = false;
        while !reached_root {
            let node = cursor.node();
            let kind = node.kind();

            if !node.is_named() {
                // Anonymous nodes (keywords, operators, punctuation) define structure
                hasher.update(kind.as_bytes());
            } else {
                // Named nodes are normalized to ignore variable/literal identity
                match kind {
                    "identifier" | "field_identifier" | "type_identifier" => {
                        hasher.update(b"|ID|");
                    }
                    "string_literal" | "integer_literal" | "float_literal" | "boolean_literal" => {
                        hasher.update(b"|LIT|");
                    }
                    "comment" => {
                        // Skip comments entirely for structural comparison
                    }
                    _ => {
                        // For structural nodes (blocks, loops, etc), hash the type
                        hasher.update(kind.as_bytes());
                    }
                }
            }

            // Depth-first traversal
            if cursor.goto_first_child() {
                continue;
            }

            if cursor.goto_next_sibling() {
                continue;
            }

            // Backtrack
            loop {
                if !cursor.goto_parent() {
                    reached_root = true;
                    break;
                }
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}

pub type Fingerprinter = StructuralFingerprinter;
