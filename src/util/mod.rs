use std::path::Path;

pub fn display_path(p: &Path) -> String {
    return format!("{}", p.display());
}
