pub mod chunker;
pub mod diff;
#[cfg(test)]
mod diff_tests;
pub mod fingerprint;

pub mod queries;
pub mod registry;

use self::fingerprint::Fingerprinter;
use crate::SdpResult;
use crate::models::{SemanticSymbol, SymbolReference};
use bytes::Bytes;
use std::collections::HashMap;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Parser, Query, QueryCursor, Tree};

pub struct SemanticParser {
    parser: Parser,
    query_cache: HashMap<String, Query>,
    tree_cache: HashMap<String, Tree>,
}

impl SemanticParser {
    pub fn new() -> SdpResult<Self> {
        Ok(Self {
            parser: Parser::new(),
            query_cache: HashMap::new(),
            tree_cache: HashMap::new(),
        })
    }

    /// Clears the tree cache. Useful when switching projects or during heavy maintenance.
    pub fn clear_cache(&mut self) {
        self.tree_cache.clear();
    }

    fn get_scope(node: Node, content: &[u8]) -> Option<String> {
        let mut current = node.parent();
        let mut parts = Vec::new();

        while let Some(parent) = current {
            let kind = parent.kind();
            if matches!(
                kind,
                "impl_item" | "struct_item" | "class_definition" | "mod_item"
            ) {
                // Try to find a name node within this parent
                for i in 0..parent.child_count() {
                    let child = parent.child(i).expect("child index is in range");
                    if child.kind().contains("identifier") {
                        if let Ok(name) = child.utf8_text(content) {
                            parts.push(name.to_string());
                            break;
                        }
                    }
                }
            }
            current = parent.parent();
        }

        if parts.is_empty() {
            None
        } else {
            parts.reverse();
            Some(parts.join("::"))
        }
    }

    pub fn parse_semantic_data(
        &mut self,
        content: &Bytes,
        extension: &str,
        snapshot_id: i64,
        file_path: Option<&str>,
    ) -> SdpResult<(Vec<SemanticSymbol>, Vec<SymbolReference>)> {
        let lang_info = match registry::get_language_info(extension) {
            Some(info) => info,
            None => return Ok((vec![], vec![])),
        };

        self.parser.set_language(&lang_info.language).map_err(|e| {
            crate::error::SdpError::Internal(format!("Error setting language: {}", e))
        })?;

        if !self.query_cache.contains_key(extension) {
            let q = Query::new(&lang_info.language, lang_info.query).map_err(|e| {
                crate::error::SdpError::Internal(format!("Query error for {}: {}", extension, e))
            })?;
            self.query_cache.insert(extension.to_string(), q);
        }
        let query = self
            .query_cache
            .get(extension)
            .expect("query was just cached");

        // Incremental parsing: check if we have a previous tree for this file
        let old_tree = file_path.and_then(|p| self.tree_cache.get(p));

        let tree = match self.parser.parse(content, old_tree) {
            Some(t) => t,
            None => return Ok((vec![], vec![])),
        };

        // Cache the new tree
        if let Some(path) = file_path {
            self.tree_cache.insert(path.to_string(), tree.clone());
        }

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(query, tree.root_node(), content.as_ref());

        let mut symbols = Vec::new();
        let mut references = Vec::new();

        while let Some(mat) = matches.next() {
            let mut name_override: Option<String> = None;

            for cap in mat.captures {
                let node = cap.node;
                let capture_name = &query.capture_names()[cap.index as usize];

                if capture_name.ends_with(".name") {
                    if let Ok(name) = node.utf8_text(content) {
                        name_override = Some(name.to_string());
                    }
                    continue;
                }

                if capture_name.starts_with("call") {
                    if let Ok(name) = node.utf8_text(content) {
                        references.push(SymbolReference {
                            symbol_name: name.to_string(),
                            snapshot_id,
                            start_line: node.start_position().row,
                            start_byte: node.start_byte(),
                        });
                    }
                    continue;
                }

                let kind = capture_name.to_string();
                let mut name = name_override.take().unwrap_or_else(|| "anonymous".into());

                if name == "anonymous" {
                    for i in 0..node.child_count() {
                        let child = node.child(i).expect("child index is in range");
                        if child.kind().contains("identifier") {
                            if let Ok(n) = child.utf8_text(content) {
                                name = n.to_string();
                                break;
                            }
                        }
                    }
                }

                let structural_hash = Fingerprinter::compute(node, content);
                let scope = Self::get_scope(node, content);

                symbols.push(SemanticSymbol {
                    id: 0,
                    name,
                    kind,
                    scope,
                    snapshot_id,
                    chunk_hash: "".to_string(),
                    structural_hash,
                    start_line: node.start_position().row,
                    end_line: node.end_position().row,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    parent_id: None,
                });
            }
        }

        symbols.sort_by(|a, b| {
            a.start_byte
                .cmp(&b.start_byte)
                .then(b.end_byte.cmp(&a.end_byte))
        });

        Ok((symbols, references))
    }

    pub fn parse_symbols(
        &mut self,
        content: &Bytes,
        extension: &str,
        snapshot_id: i64,
        file_path: Option<&str>,
    ) -> SdpResult<Vec<SemanticSymbol>> {
        let (symbols, _) = self.parse_semantic_data(content, extension, snapshot_id, file_path)?;
        Ok(symbols)
    }
}
