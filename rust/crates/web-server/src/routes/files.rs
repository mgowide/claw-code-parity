use std::path::{Component, Path};
use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct PathParam {
    path: Option<String>,
}

#[derive(Deserialize)]
pub struct ReadParams {
    path: String,
    start: Option<u64>,
    end: Option<u64>,
}

/// Reject any path that escapes the workspace root (path traversal guard).
fn safe_path(root: &Path, rel: &str) -> Option<std::path::PathBuf> {
    let joined = root.join(rel);
    // Canonicalise without resolving symlinks (file may not exist yet)
    let mut normalized = std::path::PathBuf::new();
    for component in joined.components() {
        match component {
            Component::ParentDir => { normalized.pop(); }
            Component::CurDir => {}
            c => normalized.push(c),
        }
    }
    // Must stay within root
    if normalized.starts_with(root) {
        Some(normalized)
    } else {
        None
    }
}

fn lang_from_ext(name: &str) -> &'static str {
    let ext = name.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "rust", "ts" | "tsx" => "typescript", "js" | "jsx" => "javascript",
        "vue" => "vue", "py" => "python", "go" => "go", "java" => "java",
        "c" | "h" => "c", "cpp" | "hpp" | "cc" => "cpp", "cs" => "csharp",
        "json" => "json", "toml" => "toml", "yaml" | "yml" => "yaml",
        "md" => "markdown", "html" => "html", "css" => "css", "sh" | "bash" => "bash",
        "sql" => "sql", "xml" => "xml", "txt" => "plaintext",
        _ => "plaintext",
    }
}

/// `GET /api/files?path=.` — list directory contents (one level).
pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PathParam>,
) -> impl IntoResponse {
    let rel = params.path.as_deref().unwrap_or(".");
    let Some(dir) = safe_path(&state.workspace_root, rel) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "invalid path" }))).into_response();
    };
    if !dir.is_dir() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "not a directory" }))).into_response();
    }

    let mut entries = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let mut items: Vec<_> = rd.flatten().collect();
        items.sort_by_key(|e| (!e.path().is_dir(), e.file_name()));
        for entry in items {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            // Skip hidden files and common noise
            if name.starts_with('.') || name == "node_modules" || name == "target" || name == "dist" {
                continue;
            }
            if p.is_dir() {
                let count = std::fs::read_dir(&p).map(|rd| rd.count()).unwrap_or(0);
                entries.push(serde_json::json!({ "name": name, "type": "directory", "children_count": count }));
            } else {
                let size = p.metadata().map(|m| m.len()).unwrap_or(0);
                entries.push(serde_json::json!({ "name": name, "type": "file", "size": size }));
            }
        }
    }

    Json(serde_json::json!({ "path": rel, "entries": entries })).into_response()
}

/// `GET /api/files/read?path=src/main.rs&start=1&end=100`
pub async fn read_file(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ReadParams>,
) -> impl IntoResponse {
    let Some(path) = safe_path(&state.workspace_root, &params.path) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "invalid path" }))).into_response();
    };
    if !path.is_file() {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "file not found" }))).into_response();
    }
    // Size guard: 1 MB
    let size = path.metadata().map(|m| m.len()).unwrap_or(0);
    if size > 1_048_576 {
        return (StatusCode::PAYLOAD_TOO_LARGE, Json(serde_json::json!({ "error": "file too large" }))).into_response();
    }
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::UNSUPPORTED_MEDIA_TYPE, Json(serde_json::json!({ "error": "binary file" }))).into_response();
    };
    let all_lines: Vec<&str> = raw.lines().collect();
    let total_lines = all_lines.len() as u64;
    let start = params.start.unwrap_or(1).max(1);
    let end = params.end.unwrap_or(total_lines).min(total_lines);
    let content = all_lines
        .iter()
        .skip((start - 1) as usize)
        .take((end - start + 1) as usize)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");

    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let language = lang_from_ext(&name);

    Json(serde_json::json!({
        "path": params.path,
        "content": content,
        "language": language,
        "total_lines": total_lines,
        "start_line": start,
        "end_line": end,
    })).into_response()
}
