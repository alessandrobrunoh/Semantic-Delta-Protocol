use crate::semantic::registry::get_language_info;
use bytes::Bytes;
use fastcdc::v2020::FastCDC;
use tree_sitter::{Node, Parser};

// Configuration constants for chunk sizes (aligned with FastCDC defaults)
const MIN_CHUNK_SIZE: usize = 4096;
const AVG_CHUNK_SIZE: usize = 16384;
const MAX_CHUNK_SIZE: usize = 65536;

pub struct SemanticChunker;

pub struct Chunk {
    pub offset: usize,
    pub length: usize,
    pub data: Bytes,
}

impl SemanticChunker {
    /// chunks content using a hybrid approach:
    /// 1. Parse with TreeSitter to find "natural" boundaries (top-level nodes).
    /// 2. If a node is too large, sub-chunk it with FastCDC.
    /// 3. If nodes are too small, merge them until MIN_CHUNK_SIZE.
    /// 4. Fallback to pure FastCDC if parsing fails or language unsupported.
    pub fn chunk(content: Bytes, extension: &str) -> Vec<Chunk> {
        // Try semantic parsing first
        if let Some(mut parser) = Self::get_parser(extension) {
            if let Some(tree) = parser.parse(&content, None) {
                return Self::chunk_semantic(content, tree.root_node());
            }
        }

        // Fallback to pure FastCDC
        Self::chunk_fastcdc(content)
    }

    fn get_parser(extension: &str) -> Option<Parser> {
        let info = get_language_info(extension)?;
        let mut parser = Parser::new();
        parser.set_language(&info.language).ok()?;
        Some(parser)
    }

    fn chunk_semantic(content: Bytes, root: Node) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut chunk_start = 0;

        let mut cursor = root.walk();
        let children: Vec<Node> = root.children(&mut cursor).collect();

        if children.is_empty() {
            return Self::chunk_range_smart(content.clone(), 0, content.len());
        }

        for node in children {
            let node_start = node.start_byte();
            let node_end = node.end_byte();

            // Calculate potential size if we include this node in current chunk
            let current_len_with_node = node_end - chunk_start;

            if current_len_with_node > MAX_CHUNK_SIZE {
                // Determine strategy:
                // 1. Can we just cut BEFORE this node?
                let pre_node_len = node_start - chunk_start;

                if pre_node_len >= MIN_CHUNK_SIZE {
                    // Yes, flush up to node start
                    chunks.push(Chunk {
                        offset: chunk_start,
                        length: pre_node_len,
                        data: content.slice(chunk_start..node_start),
                    });
                    chunk_start = node_start;
                }

                // Now check node itself
                let node_len = node_end - chunk_start; // Re-calc from new start

                if node_len > MAX_CHUNK_SIZE {
                    // Node is huge (bigger than MAX, even after flushing pre-gap).
                    // We MUST split this node using FastCDC.

                    let huge_block = content.slice(chunk_start..node_end);
                    let sub_chunks = Self::chunk_fastcdc_offset(huge_block, chunk_start);
                    chunks.extend(sub_chunks);

                    chunk_start = node_end;
                }
            }

            // Preference: Flush if we are at a clean boundary (node end) and size is good
            let current_len = node_end - chunk_start;
            if current_len >= AVG_CHUNK_SIZE {
                chunks.push(Chunk {
                    offset: chunk_start,
                    length: current_len,
                    data: content.slice(chunk_start..node_end),
                });
                chunk_start = node_end;
            }
        }

        // Handle tail
        if chunk_start < content.len() {
            let tail_len = content.len() - chunk_start;
            let block = content.slice(chunk_start..);
            if tail_len > MAX_CHUNK_SIZE {
                let sub_chunks = Self::chunk_fastcdc_offset(block, chunk_start);
                chunks.extend(sub_chunks);
            } else {
                chunks.push(Chunk {
                    offset: chunk_start,
                    length: tail_len,
                    data: block,
                });
            }
        }

        // Post-pass merge for tiny chunks
        Self::merge_tiny_chunks(chunks)
    }

    fn chunk_range_smart(content: Bytes, start: usize, len: usize) -> Vec<Chunk> {
        let block = content.slice(start..start + len);
        if len > MAX_CHUNK_SIZE {
            Self::chunk_fastcdc_offset(block, start)
        } else {
            vec![Chunk {
                offset: start,
                length: len,
                data: block,
            }]
        }
    }

    fn chunk_fastcdc(content: Bytes) -> Vec<Chunk> {
        Self::chunk_fastcdc_offset(content, 0)
    }

    fn chunk_fastcdc_offset(content: Bytes, global_offset: usize) -> Vec<Chunk> {
        let chunker = FastCDC::new(
            &content,
            MIN_CHUNK_SIZE as u32,
            AVG_CHUNK_SIZE as u32,
            MAX_CHUNK_SIZE as u32,
        );
        chunker
            .map(|chunk| Chunk {
                offset: global_offset + chunk.offset,
                length: chunk.length,
                data: content.slice(chunk.offset..chunk.offset + chunk.length),
            })
            .collect()
    }

    fn merge_tiny_chunks(input: Vec<Chunk>) -> Vec<Chunk> {
        if input.is_empty() {
            return input;
        }

        let mut output = Vec::with_capacity(input.len());
        let mut iter = input.into_iter();
        let mut current = iter.next().expect("input was checked to be non-empty");

        for next in iter {
            if current.length < MIN_CHUNK_SIZE {
                // Merge current into next (actually append next to current)
                // This is the only place where we MIGHT copy if we want to keep them contiguous,
                // but since these are small chunks, it's acceptable, OR we could use a Chain.
                // However, for simplicity and because we need contiguous bytes for storage/hashing,
                // we'll do a small copy here.
                let mut merged = Vec::with_capacity(current.length + next.length);
                merged.extend_from_slice(&current.data);
                merged.extend_from_slice(&next.data);

                current.length += next.length;
                current.data = Bytes::from(merged);
            } else {
                output.push(current);
                current = next;
            }
        }
        output.push(current);

        // Final check for last chunk
        if output.len() > 1 {
            let last_idx = output.len() - 1;
            if output[last_idx].length < MIN_CHUNK_SIZE {
                let last = output.pop().unwrap();
                if let Some(prev) = output.last_mut() {
                    if prev.length + last.length <= MAX_CHUNK_SIZE + MIN_CHUNK_SIZE {
                        let mut merged = Vec::with_capacity(prev.length + last.length);
                        merged.extend_from_slice(&prev.data);
                        merged.extend_from_slice(&last.data);

                        prev.length += last.length;
                        prev.data = Bytes::from(merged);
                    } else {
                        output.push(last);
                    }
                }
            }
        }

        output
    }
}
