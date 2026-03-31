use std::path::Path;
use std::sync::Arc;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

const MAX_LINE_LENGTH: usize = 5000;
const MINIFIED_CHECK_BYTES: usize = 32768;

pub fn build_gitignore(workspace_root: &Path) -> Option<Arc<Gitignore>> {
    let gitignore_path = workspace_root.join(".gitignore");
    if !gitignore_path.exists() {
        return None;
    }

    let mut builder = GitignoreBuilder::new(workspace_root);
    builder.add(&gitignore_path);
    builder.build().ok().map(Arc::new)
}

pub fn is_gitignored(gitignore: Option<&Gitignore>, path: &Path, is_dir: bool) -> bool {
    match gitignore {
        Some(gi) => gi.matched(path, is_dir).is_ignore(),
        None => false,
    }
}

pub fn is_minified(path: &Path) -> bool {
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mmap = match unsafe { memmap2::Mmap::map(&file) } {
        Ok(m) => m,
        Err(_) => return false,
    };

    let check_len = mmap.len().min(MINIFIED_CHECK_BYTES);
    let buf = &mmap[..check_len];

    let mut line_start = 0;
    for (i, &byte) in buf.iter().enumerate() {
        if byte == b'\n' {
            let line_len = i - line_start;
            if line_len > MAX_LINE_LENGTH {
                return true;
            }
            line_start = i + 1;
        }
    }

    // If we read the full check buffer without finding a newline, the line is longer than the buffer
    if check_len == MINIFIED_CHECK_BYTES && line_start == 0 {
        return true;
    }

    // Check last line (from last newline to end of buffer)
    let last_line_len = check_len - line_start;
    if last_line_len > MAX_LINE_LENGTH && check_len == mmap.len() {
        return true;
    }

    false
}
