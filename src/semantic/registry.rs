use crate::semantic::queries::*;
use tree_sitter::Language;

pub struct LanguageInfo {
    pub language: Language,
    pub query: &'static str,
}

pub fn get_language_info(extension: &str) -> Option<LanguageInfo> {
    match extension {
        "rs" => Some(LanguageInfo {
            language: tree_sitter_rust::language().into(),
            query: RUST_QUERY,
        }),
        "py" => Some(LanguageInfo {
            language: tree_sitter_python::LANGUAGE.into(),
            query: PYTHON_QUERY,
        }),
        "js" | "jsx" => Some(LanguageInfo {
            language: tree_sitter_javascript::LANGUAGE.into(),
            query: JAVASCRIPT_QUERY,
        }),
        "ts" => Some(LanguageInfo {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            query: TYPESCRIPT_QUERY,
        }),
        "tsx" => Some(LanguageInfo {
            language: tree_sitter_typescript::LANGUAGE_TSX.into(),
            query: TYPESCRIPT_QUERY,
        }),
        "go" => Some(LanguageInfo {
            language: tree_sitter_go::LANGUAGE.into(),
            query: GO_QUERY,
        }),
        "c" | "h" => Some(LanguageInfo {
            language: tree_sitter_c::LANGUAGE.into(),
            query: C_QUERY,
        }),
        "cpp" | "hpp" | "cc" | "cxx" => Some(LanguageInfo {
            language: tree_sitter_cpp::LANGUAGE.into(),
            query: CPP_QUERY,
        }),
        "java" => Some(LanguageInfo {
            language: tree_sitter_java::LANGUAGE.into(),
            query: JAVA_QUERY,
        }),
        "rb" => Some(LanguageInfo {
            language: tree_sitter_ruby::LANGUAGE.into(),
            query: RUBY_QUERY,
        }),
        "cs" => Some(LanguageInfo {
            language: tree_sitter_c_sharp::LANGUAGE.into(),
            query: C_SHARP_QUERY,
        }),
        "php" => Some(LanguageInfo {
            language: tree_sitter_php::LANGUAGE_PHP.into(),
            query: PHP_QUERY,
        }),
        "json" => Some(LanguageInfo {
            language: tree_sitter_json::LANGUAGE.into(),
            query: JSON_QUERY,
        }),
        "html" => Some(LanguageInfo {
            language: tree_sitter_html::LANGUAGE.into(),
            query: HTML_QUERY,
        }),
        "css" => Some(LanguageInfo {
            language: tree_sitter_css::LANGUAGE.into(),
            query: CSS_QUERY,
        }),
        "md" => Some(LanguageInfo {
            language: tree_sitter_md::LANGUAGE.into(),
            query: MARKDOWN_QUERY,
        }),
        _ => None,
    }
}
