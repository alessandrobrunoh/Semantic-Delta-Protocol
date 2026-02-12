use lasso::{Spur, ThreadedRodeo};
use std::sync::LazyLock;

/// Global string interner for efficient memory usage.
/// Used for common strings like symbol names, file paths, and language extensions.
pub static INTERNER: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

/// Interns a string and returns a stable `Spur` (ID).
pub fn intern(s: &str) -> Spur {
    INTERNER.get_or_intern(s)
}

/// Resolves an interned ID back to its original string.
pub fn resolve(spur: Spur) -> String {
    INTERNER.resolve(&spur).to_string()
}
