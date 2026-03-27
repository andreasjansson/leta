use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub path: String,
    pub line: u32,
    #[serde(default)]
    pub column: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_lines: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl LocationInfo {
    pub fn new(path: String, line: u32) -> Self {
        Self {
            path,
            line,
            column: 0,
            context_lines: None,
            context_start: None,
            name: None,
            kind: None,
            detail: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub lines: u32,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub current_bytes: u64,
    pub max_bytes: u64,
    pub entries: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub root: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_pid: Option<u32>,
    pub open_documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,
    #[serde(default)]
    pub in_workspace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calls: Option<Vec<CallNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub called_by: Option<Vec<CallNode>>,
}

impl CallNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            kind: None,
            detail: None,
            path: None,
            line: None,
            column: None,
            in_workspace: true,
            calls: None,
            called_by: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CallGraphSymbol {
    pub name: String,
    pub kind: String,
    pub path: String,
    pub line: u32,
    pub column: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphEdge {
    pub caller: CallGraphSymbol,
    pub callee: CallGraphSymbol,
    pub in_workspace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_site_line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphFileEdges {
    pub edges: Vec<CallGraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResult {
    pub nodes: Vec<CallGraphSymbol>,
    pub edges: Vec<CallGraphEdge>,
    pub indexing_time_ms: Option<u64>,
}
