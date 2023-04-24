use lsp_types::{Position, request::Request, Range};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum CompletionRequest {}

#[derive(Serialize, Deserialize)]
pub struct CompletionRequestParams {
    pub doc: Document,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub tab_size: usize,
    pub insert_spaces: bool,
    pub path: String,
    pub indent_size: usize,
    pub version: usize,
    pub relative_path: String,
    pub language_id: String,
    pub position: Position,
    pub source: String,
    pub uri: String,
}

impl Request for CompletionRequest {
    type Params = CompletionRequestParams;
    type Result = Option<CompletionResponse>;
    const METHOD: &'static str = "getCompletionsCycling";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    pub completions: Vec<Completion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    uuid: String,
    pub range: Range,
    display_text: String,
    position: Position,
    doc_version: Option<usize>,
    point: Option<usize>,
    region: Option<(usize, usize)>,
    pub text: String,
}
