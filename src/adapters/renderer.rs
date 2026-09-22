#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RendererArtifact {
    pub id: String,
    pub statement: String,
    pub source_id: String,
    pub line: usize,
}

impl RendererArtifact {
    pub fn new(id: &str, statement: &str, source_id: &str, line: usize) -> Self {
        Self {
            id: id.to_owned(),
            statement: statement.to_owned(),
            source_id: source_id.to_owned(),
            line,
        }
    }
}
