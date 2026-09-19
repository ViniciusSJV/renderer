use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::io::Write;

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

    fn version_example() -> Source {
        Source {
            id: String::from("SRC_TEST"),
            path: Some(String::from("exemplo.rs")),
            sha256: Some(format!("{:x}", Sha256::digest(b"linha original\n"))),
            lines: vec![String::from("linha original")],
        }
    }

    #[test]
    fn context_zero_and_large_radius_preserve_bounds() {
        let (mut fact, mut source) = example();
        source.lines = vec![String::from("a"), String::from("b"), String::from("c")];
        fact.line = 2;
        for (radius, expected) in [(0, vec!["b"]), (usize::MAX, vec!["a", "b", "c"])] {
            let value: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, radius).unwrap()).unwrap();
            assert_eq!(
                value["source"]["context"]["lines"],
                serde_json::json!(expected)
            );
            assert_eq!(value["source"]["context"]["requested_radius"], radius);
        }
    }

    #[test]
    fn same_selection_produces_identical_json() {
        let (fact, source) = example();
        assert_eq!(
            selection_json(&fact, &source, 4).unwrap(),
            selection_json(&fact, &source, 4).unwrap()
        );
    }

    #[test]
    fn exports_exact_statement_reference_and_excerpt() {
        let (fact, source) = example();
        let json = selection_json(&fact, &source, 3).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["fact_id"], fact.id);
        assert_eq!(value["statement"], fact.statement);
        assert_eq!(value["source"]["id"], source.id);
        assert_eq!(value["source"]["line"], fact.line);
        assert_eq!(value["source"]["excerpt"], source.lines[0]);
        assert!(value["source"]["sha256"].is_null());
    }

    #[test]
    fn exports_context_with_correct_line_numbers_and_bounds() {
        let (mut fact, mut source) = example();
        source.lines = (1..=10).map(|number| format!("linha {}", number)).collect();

        for (line, expected_start, expected_end) in [(1, 1, 4), (5, 2, 8), (10, 7, 10)] {
            fact.line = line;
            let value: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, 3).unwrap()).unwrap();
            let context = &value["source"]["context"];
            assert_eq!(context["start_line"], expected_start);
            assert_eq!(context["end_line"], expected_end);
            let expected: Vec<String> = (expected_start..=expected_end)
                .map(|number| format!("linha {}", number))
                .collect();
            assert_eq!(context["lines"], serde_json::json!(expected));
            assert_eq!(value["source"]["line"], line);
            assert_eq!(value["source"]["excerpt"], format!("linha {}", line));
        }
    }

    #[test]
    fn does_not_export_invalid_reference() {
        let (mut fact, source) = example();
        fact.line = 0;
        assert!(selection_json(&fact, &source, 3).is_err());
    }

    #[test]
    fn finds_requested_fact_after_another_fact() {
        let (first, _) = example();
        let (mut second, _) = example();
        second.id = String::from("F2");
        second.statement = String::from("Segunda ficha.");
        let facts = vec![first, second];
        assert_eq!(find_fact(&facts, "F2").unwrap().statement, "Segunda ficha.");
        assert!(find_fact(&facts, "F9").is_none());
        assert!(find_fact(&[], "F2").is_none());
    }

    #[test]
    fn accepts_matching_source_content() {
        assert_eq!(
            validate_source_content(&version_example(), b"linha original\n"),
            Ok(())
        );
    }

    #[test]
    fn rejects_changed_file_bytes() {
        assert!(validate_source_content(&version_example(), b"linha alterada\n").is_err());
    }

    #[test]
    fn rejects_tampered_snapshot_with_matching_hash() {
        let mut source = version_example();
        source.lines[0] = String::from("outro texto");
        assert!(validate_source_content(&source, b"linha original\n").is_err());
    }

    #[test]
    fn requires_complete_version_metadata() {
        let mut source = version_example();
        source.path = None;
        assert!(validate_source_version(&source).is_err());
    }

    #[test]
    fn preserves_sources_without_version_metadata() {
        let (_, source) = example();
        assert_eq!(validate_source_version(&source), Ok(()));
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
            path: None,
            sha256: None,
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
                path: None,
                sha256: None,
                id: String::from("S1"),
                lines: vec![],
            },
            Source {
                path: None,
                sha256: None,
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
                path: None,
                sha256: None,
                id: String::from("S1"),
                lines: vec![String::from("Primeira obra.")],
            },
            Source {
                path: None,
                sha256: None,
                id: String::from("S2"),
                lines: vec![],
            },
            Source {
                path: None,
                sha256: None,
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
                path: None,
                sha256: None,
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
    path: Option<String>,
    sha256: Option<String>,
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

fn find_fact<'a>(facts: &'a [Fact], fact_id: &str) -> Option<&'a Fact> {
    for fact in facts {
        if fact.id == fact_id {
            return Some(fact);
        }
    }
    None
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

fn validate_source_content(source: &Source, bytes: &[u8]) -> Result<(), String> {
    let expected = match &source.sha256 {
        Some(hash) => hash,
        None => return Err(format!("{}: SHA-256 ausente.", source.id)),
    };
    let actual = format!("{:x}", Sha256::digest(bytes));
    if actual != *expected {
        return Err(format!(
            "{}: o arquivo atual difere da edição registrada (SHA-256 diferente).",
            source.id
        ));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("{}: conteúdo não é UTF-8: {}", source.id, error))?;
    if !text.lines().eq(source.lines.iter().map(String::as_str)) {
        return Err(format!(
            "{}: as linhas do dossiê diferem das linhas do arquivo.",
            source.id
        ));
    }
    Ok(())
}

fn validate_source_version(source: &Source) -> Result<(), String> {
    match (&source.path, &source.sha256) {
        (None, None) => Ok(()),
        (Some(path), Some(_)) => {
            let bytes = fs::read(path).map_err(|error| {
                format!("{}: não foi possível ler {}: {}", source.id, path, error)
            })?;
            validate_source_content(source, &bytes)
        }
        _ => Err(format!(
            "{}: path e sha256 devem ser informados juntos.",
            source.id
        )),
    }
}

fn selection_json(fact: &Fact, source: &Source, radius: usize) -> Result<String, String> {
    validate_reference(fact, source)?;
    let index = fact.line - 1;
    let start = index.saturating_sub(radius);
    let end = fact.line.saturating_add(radius).min(source.lines.len());
    let selection = serde_json::json!({
        "fact_id": fact.id,
        "statement": fact.statement,
        "source": {
            "id": source.id,
            "path": source.path,
            "sha256": source.sha256,
            "line": fact.line,
            "excerpt": source.lines[index],
            "context": {
                "requested_radius": radius,
                "start_line": start + 1,
                "end_line": end,
                "lines": &source.lines[start..end]
            }
        },
        "scope": "Uma ficha, sua linha de referência e linhas vizinhas conforme requested_radius. Janela textual que pode cortar funções; significado da afirmação não validado."
    });
    serde_json::to_string_pretty(&selection).map_err(|error| error.to_string())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let mut radius = 3;
    let mut output_path: Option<&str> = None;
    let selected_id = if arguments.get(2).map(String::as_str) == Some("--fact") {
        let id = arguments
            .get(3)
            .filter(|id| !id.starts_with("--"))
            .unwrap_or_else(|| {
                eprintln!("Informe o ID após --fact.");
                std::process::exit(1);
            });
        let mut position = 4;
        let mut context_seen = false;
        while position < arguments.len() {
            let value = arguments
                .get(position + 1)
                .filter(|v| !v.starts_with("--"))
                .unwrap_or_else(|| {
                    eprintln!("Falta um valor após {}.", arguments[position]);
                    std::process::exit(1);
                });
            match arguments[position].as_str() {
                "--context" if !context_seen => {
                    radius = value.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!(
                            "--context exige um inteiro não negativo representável como usize."
                        );
                        std::process::exit(1);
                    });
                    context_seen = true;
                }
                "--output" if output_path.is_none() => output_path = Some(value.as_str()),
                _ => {
                    eprintln!("Opção desconhecida ou repetida: {}", arguments[position]);
                    std::process::exit(1);
                }
            }
            position += 2;
        }
        Some(id.as_str())
    } else {
        if arguments.len() > 3 || arguments.get(2).is_some_and(|v| v.starts_with("--")) {
            eprintln!("Uso: validate_evidence [DOSSIÊ] [PARECER] ou DOSSIÊ --fact ID [--context N] [--output ARQUIVO]");
            std::process::exit(1);
        }
        None
    };
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

    for source in &sources {
        match validate_source_version(source) {
            Ok(()) if source.path.is_some() => println!(
                "{}: arquivo atual e linhas correspondem à edição registrada.",
                source.id
            ),
            Ok(()) => println!(
                "{}: sem metadados de versão; arquivo externo não conferido.",
                source.id
            ),
            Err(message) => {
                eprintln!("{}", message);
                std::process::exit(1);
            }
        }
    }

    let mut invalid_references = 0;

    for fact in &facts {
        if selected_id.is_none() {
            println!("{}: {}", fact.id, fact.statement);
            println!("Origem: {}, linha {}", fact.source_id, fact.line);
        }

        match find_source(&sources, &fact.source_id) {
            Some(source) => match validate_reference(fact, source) {
                Ok(()) => {
                    let index = fact.line - 1;
                    if selected_id.is_none() {
                        println!("Trecho encontrado: {}", source.lines[index]);
                    }
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

    if let Some(id) = selected_id {
        let fact = match find_fact(&facts, id) {
            Some(fact) => fact,
            None => {
                eprintln!("Ficha \"{}\" não encontrada.", id);
                std::process::exit(1);
            }
        };
        // As referências e versões foram conferidas antes da seleção.
        let source = find_source(&sources, &fact.source_id).expect("A fonte da ficha foi validada");
        println!("Ficha selecionada: {}", fact.id);
        println!("Afirmação: {}", fact.statement);
        println!("Referência: {}, linha {}", source.id, fact.line);
        if let Some(path) = &source.path {
            println!("Arquivo: {}", path);
        }
        if let Some(hash) = &source.sha256 {
            println!("SHA-256: {}", hash);
        }
        println!("Trecho: {}", source.lines[fact.line - 1]);
        if let Some(output_path) = output_path {
            let result = selection_json(fact, source, radius).and_then(|json| {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output_path)
                    .map_err(|error| error.to_string())?;
                writeln!(file, "{}", json).map_err(|error| error.to_string())
            });
            match result {
                Ok(()) => println!("Seleção salva em {}", output_path),
                Err(error) => {
                    eprintln!("Não foi possível exportar a seleção: {}", error);
                    std::process::exit(1);
                }
            }
        }
        return;
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
