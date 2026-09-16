use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

#[derive(Deserialize)]
struct Fact {
    id: String,
    statement: String,
    source_id: String,
    line: usize,
}

// Nesta etapa, carregamos apenas os campos usados na validação estrutural.
// Os demais campos do JSON são ignorados pelo Serde.
#[derive(Deserialize)]
struct Evidence {
    sources: Vec<Source>,
    facts: Vec<Fact>,
}

// Conferimos o conteúdo referenciado, não a correção do julgamento do modelo.
#[derive(Deserialize)]
struct Review {
    id: String,
    fact_id: String,
    original_statement: String,
    source: ReviewSource,
    evaluated_statement: String,
    verdict: String,
    justification: String,
}

#[derive(Deserialize)]
struct ReviewSource {
    id: String,
    line: usize,
    excerpt: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn review_example() -> (Review, Evidence) {
        let review = serde_json::from_str(include_str!(
            "../../ai/experimentos/02-mutex/parecer-f4.json"
        ))
        .unwrap();
        let evidence = serde_json::from_str(include_str!(
            "../../ai/experimentos/02-mutex/evidencias-afirmacao-incorreta.json"
        ))
        .unwrap();
        (review, evidence)
    }

    #[test]
    fn accepts_review_for_scoped_reformulation() {
        let (review, evidence) = review_example();
        assert_ne!(review.original_statement, review.evaluated_statement);
        assert_eq!(
            validate_review(&review, &evidence.facts, &evidence.sources),
            Ok(())
        );
    }

    #[test]
    fn rejects_review_for_missing_fact() {
        let (mut review, evidence) = review_example();
        review.fact_id = String::from("F99");
        assert_eq!(
            validate_review(&review, &evidence.facts, &evidence.sources),
            Err(String::from("P1: ficha \"F99\" não encontrada."))
        );
    }

    #[test]
    fn rejects_review_for_changed_statement() {
        let (review, mut evidence) = review_example();
        evidence.facts[3].statement = String::from("Outro texto.");
        assert_eq!(
            validate_review(&review, &evidence.facts, &evidence.sources),
            Err(String::from("P1: afirmação original difere da ficha F4."))
        );
    }

    #[test]
    fn rejects_review_for_different_reference() {
        let (mut review, evidence) = review_example();
        review.source.line = 4;
        assert_eq!(
            validate_review(&review, &evidence.facts, &evidence.sources),
            Err(String::from(
                "P1: referência do parecer difere da ficha F4."
            ))
        );
    }

    #[test]
    fn rejects_review_for_changed_excerpt() {
        let (review, mut evidence) = review_example();
        evidence.sources[0].lines[4] = String::from("Trecho alterado.");
        assert_eq!(
            validate_review(&review, &evidence.facts, &evidence.sources),
            Err(String::from(
                "P1: trecho copiado difere do trecho da fonte."
            ))
        );
    }

    #[test]
    fn loads_recorded_review() {
        let text = include_str!("../../ai/experimentos/02-mutex/parecer-f4.json");
        let review: Review = serde_json::from_str(text).expect("Parecer deve carregar");
        assert_eq!(review.id, "P1");
        assert_eq!(review.fact_id, "F4");
        assert_eq!(review.verdict, "CONTRADIZ");
        assert_eq!(
            review.evaluated_statement,
            "Na operação de escrita mostrada em S1:5, a tarefa B escreve no pixel 999."
        );
    }

    #[test]
    fn loads_experiment_and_validates_its_references() {
        let text = include_str!("../../ai/experimentos/02-mutex/evidencias.json");
        let evidence: Evidence = serde_json::from_str(text).expect("JSON deve carregar");
        assert_eq!(evidence.sources.len(), 1);
        assert_eq!(evidence.facts.len(), 5);
        assert_eq!(validate_source_ids(&evidence.sources), Ok(()));
        assert_eq!(validate_fact_ids(&evidence.facts), Ok(()));
        for fact in &evidence.facts {
            let source = find_source(&evidence.sources, &fact.source_id).unwrap();
            assert_eq!(validate_reference(fact, source), Ok(()));
        }
    }

    #[test]
    fn rejects_fact_without_source_reference() {
        let text = r#"{"sources": [], "facts": [{"id": "F1", "statement": "Texto", "line": 1}]}"#;
        assert!(serde_json::from_str::<Evidence>(text).is_err());
    }

    #[test]
    fn parsing_does_not_validate_line_bounds() {
        let text = r#"{"id": "F1", "statement": "Texto", "source_id": "S1", "line": 0}"#;
        let fact: Fact = serde_json::from_str(text).expect("Zero cabe em usize");
        let (_, source) = example();
        assert!(validate_reference(&fact, &source).is_err());
    }

    fn example() -> (Fact, Source) {
        let source = Source {
            id: String::from("S1"),
            lines: vec![String::from("Um trecho da obra.")],
        };
        let fact = Fact {
            id: String::from("F1"),
            statement: String::from("Uma afirmação a conferir."),
            source_id: String::from("S1"),
            line: 1,
        };
        (fact, source)
    }

    #[test]
    fn accepts_equal_statements_with_different_fact_ids() {
        let (first, _) = example();
        let (mut second, _) = example();
        second.id = String::from("F2");
        assert_eq!(validate_fact_ids(&[first, second]), Ok(()));
    }

    #[test]
    fn rejects_duplicate_fact_ids_with_different_statements() {
        let (first, _) = example();
        let (mut second, _) = example();
        second.statement = String::from("Outra afirmação.");
        assert_eq!(
            validate_fact_ids(&[first, second]),
            Err(String::from("Fato duplicado: \"F1\"."))
        );
    }

    #[test]
    fn accepts_empty_fact_list() {
        assert_eq!(validate_fact_ids(&[]), Ok(()));
    }

    #[test]
    fn accepts_unique_source_ids() {
        let sources = vec![
            Source {
                id: String::from("S1"),
                lines: vec![],
            },
            Source {
                id: String::from("S2"),
                lines: vec![],
            },
        ];
        assert_eq!(validate_source_ids(&sources), Ok(()));
    }

    #[test]
    fn rejects_duplicate_source_ids_with_different_content() {
        let sources = vec![
            Source {
                id: String::from("S1"),
                lines: vec![String::from("Primeira obra.")],
            },
            Source {
                id: String::from("S2"),
                lines: vec![],
            },
            Source {
                id: String::from("S1"),
                lines: vec![String::from("Outra obra.")],
            },
        ];
        assert_eq!(
            validate_source_ids(&sources),
            Err(String::from("Fonte duplicada: \"S1\"."))
        );
    }

    #[test]
    fn accepts_empty_source_catalog() {
        assert_eq!(validate_source_ids(&[]), Ok(()));
    }

    #[test]
    fn finds_source_after_another_source() {
        let (_, source) = example();
        let sources = vec![
            Source {
                id: String::from("S2"),
                lines: vec![],
            },
            source,
        ];
        let found = find_source(&sources, "S1").expect("S1 deve ser encontrada");
        assert_eq!(found.id, "S1");
        assert_eq!(found.lines, vec![String::from("Um trecho da obra.")]);
        assert!(find_source(&sources, "S9").is_none());
    }

    #[test]
    fn finds_nothing_in_empty_collection() {
        assert!(find_source(&[], "S1").is_none());
    }

    #[test]
    fn accepts_existing_reference() {
        let (fact, source) = example();
        assert_eq!(validate_reference(&fact, &source), Ok(()));
    }

    #[test]
    fn rejects_missing_source() {
        let (mut fact, source) = example();
        fact.source_id = String::from("S9");
        assert_eq!(
            validate_reference(&fact, &source),
            Err(String::from("F1: fonte \"S9\" não encontrada."))
        );
    }

    #[test]
    fn rejects_line_zero() {
        let (mut fact, source) = example();
        fact.line = 0;
        assert_eq!(
            validate_reference(&fact, &source),
            Err(String::from(
                "F1: linha 0 inválida na fonte \"S1\"; a fonte contém 1 linhas."
            ))
        );
    }

    #[test]
    fn rejects_line_beyond_source() {
        let (mut fact, source) = example();
        fact.line = 2;
        assert_eq!(
            validate_reference(&fact, &source),
            Err(String::from(
                "F1: linha 2 inválida na fonte \"S1\"; a fonte contém 1 linhas."
            ))
        );
    }
}

#[derive(Deserialize)]
struct Source {
    id: String,
    lines: Vec<String>,
}

fn validate_source_ids(sources: &[Source]) -> Result<(), String> {
    let mut seen = HashSet::new();

    for source in sources {
        if !seen.insert(source.id.as_str()) {
            return Err(format!("Fonte duplicada: \"{}\".", source.id));
        }
    }

    Ok(())
}

fn validate_fact_ids(facts: &[Fact]) -> Result<(), String> {
    let mut seen = HashSet::new();

    for fact in facts {
        if !seen.insert(fact.id.as_str()) {
            return Err(format!("Fato duplicado: \"{}\".", fact.id));
        }
    }

    Ok(())
}

fn find_source<'a>(sources: &'a [Source], source_id: &str) -> Option<&'a Source> {
    for source in sources {
        if source.id == source_id {
            return Some(source);
        }
    }

    None
}

fn validate_reference(fact: &Fact, source: &Source) -> Result<(), String> {
    if fact.source_id != source.id {
        return Err(format!(
            "{}: fonte \"{}\" não encontrada.",
            fact.id, fact.source_id
        ));
    }

    if fact.line == 0 || fact.line > source.lines.len() {
        return Err(format!(
            "{}: linha {} inválida na fonte \"{}\"; a fonte contém {} linhas.",
            fact.id,
            fact.line,
            source.id,
            source.lines.len()
        ));
    }

    Ok(())
}

fn validate_review(review: &Review, facts: &[Fact], sources: &[Source]) -> Result<(), String> {
    validate_fact_ids(facts)?;
    validate_source_ids(sources)?;

    let fact = match facts.iter().find(|fact| fact.id == review.fact_id) {
        Some(fact) => fact,
        None => {
            return Err(format!(
                "{}: ficha \"{}\" não encontrada.",
                review.id, review.fact_id
            ))
        }
    };

    if review.original_statement != fact.statement {
        return Err(format!(
            "{}: afirmação original difere da ficha {}.",
            review.id, fact.id
        ));
    }
    if review.source.id != fact.source_id || review.source.line != fact.line {
        return Err(format!(
            "{}: referência do parecer difere da ficha {}.",
            review.id, fact.id
        ));
    }

    let source = match find_source(sources, &fact.source_id) {
        Some(source) => source,
        None => {
            return Err(format!(
                "{}: fonte \"{}\" não encontrada.",
                review.id, fact.source_id
            ))
        }
    };
    validate_reference(fact, source)?;

    if review.source.excerpt != source.lines[fact.line - 1] {
        return Err(format!(
            "{}: trecho copiado difere do trecho da fonte.",
            review.id
        ));
    }
    Ok(())
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(argument) => argument,
        None => String::from("ai/experimentos/02-mutex/evidencias.json"),
    };
    let content = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("Não foi possível ler \"{}\": {}", path, error);
            std::process::exit(1);
        }
    };
    println!("Documento lido: {} ({} bytes).", path, content.len());
    let evidence: Evidence = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Não foi possível interpretar o JSON: {}", error);
            std::process::exit(1);
        }
    };
    let sources = evidence.sources;
    let facts = evidence.facts;

    println!("Obras no acervo: {}", sources.len());
    println!("Fichas no catálogo: {}", facts.len());

    match validate_source_ids(&sources) {
        Ok(()) => println!("Acervo sem IDs de fontes duplicados."),
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }

    match validate_fact_ids(&facts) {
        Ok(()) => println!("Catálogo sem IDs de fatos duplicados."),
        Err(message) => {
            eprintln!("{}", message);
            std::process::exit(1);
        }
    }

    let mut invalid_references = 0;

    for fact in &facts {
        println!("{}: {}", fact.id, fact.statement);
        println!("Origem: {}, linha {}", fact.source_id, fact.line);

        match find_source(&sources, &fact.source_id) {
            Some(source) => match validate_reference(fact, source) {
                Ok(()) => {
                    let index = fact.line - 1;
                    println!("Trecho encontrado: {}", source.lines[index]);
                }
                Err(message) => {
                    eprintln!("{}", message);
                    invalid_references += 1;
                }
            },
            None => {
                eprintln!("{}: fonte \"{}\" não encontrada.", fact.id, fact.source_id);
                invalid_references += 1;
            }
        }
    }

    println!(
        "Fichas verificadas: {}. Referências inválidas: {}.",
        facts.len(),
        invalid_references
    );

    if invalid_references > 0 {
        std::process::exit(1);
    }

    if let Some(review_path) = std::env::args().nth(2) {
        let text = match fs::read_to_string(&review_path) {
            Ok(text) => text,
            Err(error) => {
                eprintln!(
                    "Não foi possível ler o parecer \"{}\": {}",
                    review_path, error
                );
                std::process::exit(1);
            }
        };
        let review: Review = match serde_json::from_str(&text) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Não foi possível interpretar o parecer: {}", error);
                std::process::exit(1);
            }
        };
        println!("Parecer {}: ficha indicada {}.", review.id, review.fact_id);
        println!("Afirmação avaliada: {}", review.evaluated_statement);
        println!("Julgamento registrado: {}", review.verdict);
        println!("Justificativa: {}", review.justification);
        match validate_review(&review, &facts, &sources) {
            Ok(()) => println!(
                "Parecer ligado à ficha e ao trecho conferidos. Julgamento semântico não validado."
            ),
            Err(message) => {
                eprintln!("{}", message);
                std::process::exit(1);
            }
        }
    }
}
