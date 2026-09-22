#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bibliography_fact_can_be_adapted_from_renderer_artifact() {
        let artifact = crate::adapters::renderer::RendererArtifact::new(
            "F-1",
            "ray hits sphere",
            "source-1",
            12,
        );
        let fact = BibliotecarioFact::from_renderer_artifact(&artifact);
        assert_eq!(fact.id, "F-1");
        assert_eq!(fact.statement, "ray hits sphere");
        assert_eq!(fact.source_id, "source-1");
        assert_eq!(fact.line, 12);
    }

    #[test]
    fn selection_uses_requested_order_and_context() {
        let selection = Selection::new(vec!["F-2", "F-1"], 3);
        assert_eq!(selection.fact_ids, vec!["F-2", "F-1"]);
        assert_eq!(selection.context_lines, 3);
    }

    #[test]
    fn bibliography_fact_can_be_adapted_from_chess_artifact() {
        let artifact = crate::adapters::chess::ChessArtifact::new(
            "C-1",
            "white queen moves from d1 to h5",
            "game-01",
            18,
        );
        let fact = BibliotecarioFact::from_chess_artifact(&artifact);
        assert_eq!(fact.id, "C-1");
        assert_eq!(fact.statement, "white queen moves from d1 to h5");
        assert_eq!(fact.source_id, "game-01");
        assert_eq!(fact.line, 18);
    }

    #[test]
    fn evidence_bundle_keeps_source_context_and_selection_order() {
        let source = SourceRef {
            id: "source-1".into(),
            path: "src/renderer.rs".into(),
            lines: vec!["a".into(), "b".into(), "c".into()],
        };
        let facts = vec![
            BibliotecarioFact::new("F-2", "second fact", "source-1", 2),
            BibliotecarioFact::new("F-1", "first fact", "source-1", 1),
        ];
        let selection = Selection::new(vec!["F-2", "F-1"], 1);
        let bundle = EvidenceBundle::new(source, facts.clone(), selection.clone());
        assert_eq!(bundle.selection.fact_ids, vec!["F-2", "F-1"]);
        assert_eq!(bundle.source.id, "source-1");
        assert_eq!(bundle.facts[0].id, "F-2");
        assert_eq!(bundle.facts.len(), 2);
        let exported = bundle.to_query("Explain this fact");
        assert_eq!(exported.question, "Explain this fact");
        assert_eq!(exported.evidence.facts[1].id, "F-1");
    }

    #[test]
    fn validation_rejects_empty_fact_and_empty_selection() {
        let valid = BibliotecarioFact::new("F-1", "statement", "source-1", 1);
        assert!(valid.validate().ok);

        let invalid = BibliotecarioFact::new("", "statement", "source-1", 1);
        assert!(!invalid.validate().ok);

        let empty_selection = Selection::new(vec![], 0);
        assert!(!empty_selection.validate().ok);

        let valid_bundle = EvidenceBundle::new(
            SourceRef {
                id: "source-1".into(),
                path: "src/example.rs".into(),
                lines: vec!["line 1".into()],
            },
            vec![valid.clone()],
            Selection::new(vec!["F-1"], 1),
        );
        assert!(valid_bundle.validate().ok);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct SourceRef {
    pub id: String,
    pub path: String,
    pub lines: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct BibliotecarioFact {
    pub id: String,
    pub statement: String,
    pub source_id: String,
    pub line: usize,
}

impl BibliotecarioFact {
    pub fn new(id: &str, statement: &str, source_id: &str, line: usize) -> Self {
        Self {
            id: id.to_owned(),
            statement: statement.to_owned(),
            source_id: source_id.to_owned(),
            line,
        }
    }

    pub fn validate(&self) -> ValidationOutcome {
        if self.id.trim().is_empty() {
            return ValidationOutcome::fail("fact.id must not be empty");
        }
        if self.statement.trim().is_empty() {
            return ValidationOutcome::fail("fact.statement must not be empty");
        }
        if self.source_id.trim().is_empty() {
            return ValidationOutcome::fail("fact.source_id must not be empty");
        }
        if self.line == 0 {
            return ValidationOutcome::fail("fact.line must be greater than zero");
        }
        ValidationOutcome::ok()
    }

    pub fn from_renderer_artifact(artifact: &crate::adapters::renderer::RendererArtifact) -> Self {
        Self {
            id: artifact.id.clone(),
            statement: artifact.statement.clone(),
            source_id: artifact.source_id.clone(),
            line: artifact.line,
        }
    }

    pub fn from_chess_artifact(artifact: &crate::adapters::chess::ChessArtifact) -> Self {
        Self {
            id: artifact.id.clone(),
            statement: artifact.statement.clone(),
            source_id: artifact.source_id.clone(),
            line: artifact.line,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub fact_ids: Vec<String>,
    pub context_lines: usize,
}

impl Selection {
    pub fn new(fact_ids: Vec<&str>, context_lines: usize) -> Self {
        Self {
            fact_ids: fact_ids.into_iter().map(str::to_owned).collect(),
            context_lines,
        }
    }

    pub fn validate(&self) -> ValidationOutcome {
        if self.fact_ids.is_empty() {
            return ValidationOutcome::fail("selection.fact_ids must not be empty");
        }
        if self.context_lines == 0 {
            return ValidationOutcome::fail("selection.context_lines must be greater than zero");
        }
        for fact_id in &self.fact_ids {
            if fact_id.trim().is_empty() {
                return ValidationOutcome::fail("selection.fact_ids must not contain empty ids");
            }
        }
        ValidationOutcome::ok()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ValidationOutcome {
    pub ok: bool,
    pub reason: Option<String>,
}

impl ValidationOutcome {
    pub fn ok() -> Self {
        Self {
            ok: true,
            reason: None,
        }
    }

    pub fn fail(reason: impl Into<String>) -> Self {
        Self {
            ok: false,
            reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct EvidenceBundle {
    pub source: SourceRef,
    pub facts: Vec<BibliotecarioFact>,
    pub selection: Selection,
}

impl EvidenceBundle {
    pub fn new(source: SourceRef, facts: Vec<BibliotecarioFact>, selection: Selection) -> Self {
        Self {
            source,
            facts,
            selection,
        }
    }

    pub fn validate(&self) -> ValidationOutcome {
        if self.source.path.trim().is_empty() {
            return ValidationOutcome::fail("bundle.source.path must not be empty");
        }
        if self.facts.is_empty() {
            return ValidationOutcome::fail("bundle.facts must not be empty");
        }
        for fact in &self.facts {
            let outcome = fact.validate();
            if !outcome.ok {
                return ValidationOutcome::fail(format!("bundle fact invalid: {}", outcome.reason.unwrap_or_default()));
            }
        }
        if !self.selection.validate().ok {
            return self.selection.validate();
        }
        ValidationOutcome::ok()
    }

    pub fn to_query(&self, question: &str) -> QueryExport {
        QueryExport {
            question: question.to_owned(),
            evidence: self.clone(),
            instructions: vec!["Cite a origem das evidências".to_owned()],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct QueryExport {
    pub question: String,
    pub evidence: EvidenceBundle,
    pub instructions: Vec<String>,
}
