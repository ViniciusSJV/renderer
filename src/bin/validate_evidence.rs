use librarian_graph_engine::filesystem::SourceChecks;
#[cfg(test)]
use librarian_graph_engine::filesystem::{
    validate_capture_link, validate_capture_link_cached, validate_source_content,
    validate_source_version, CaptureCache,
};
use librarian_graph_engine::{
    capture, find_fact, find_source, metrics, prepare_query, validate_dossier, validate_review,
    Review,
};
#[cfg(test)]
use librarian_graph_engine::{
    merge_ranges, query_json, select_facts, selection_with_capture, selections_json_with_capture,
    validate_execution_record, validate_fact_ids, validate_reference, validate_source_ids,
    Evidence, Fact, Source,
};
#[cfg(test)]
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;

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

    struct CaptureFixture {
        evidence: Evidence,
        dir: std::path::PathBuf,
    }
    impl std::ops::Deref for CaptureFixture {
        type Target = Evidence;
        fn deref(&self) -> &Evidence {
            &self.evidence
        }
    }
    impl std::ops::DerefMut for CaptureFixture {
        fn deref_mut(&mut self) -> &mut Evidence {
            &mut self.evidence
        }
    }
    impl Drop for CaptureFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn capture_dossier() -> CaptureFixture {
        use serde_json::json;
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "capture-dossier-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        // Synthetic fixture only: no historical evidence is modified or re-certified.
        let mut evidence: Evidence = serde_json::from_str(include_str!(
            "../../ai/experimentos/07-dossie-captura/evidencias.json"
        ))
        .unwrap();
        let output = include_bytes!("../../ai/experimentos/05-associacao/fronteira/saida.bin");
        fs::write(dir.join("saida.bin"), output).unwrap();
        fs::write(dir.join("source"), b"synthetic source").unwrap();
        let hash = format!("{:x}", Sha256::digest(b"synthetic source"));
        let record = json!({"schema_version":2,"run_id":"RUN_EQUIVALENCE_BOUNDARY_2",
            "argv":["synthetic-fixture"],"cwd":dir,"started_at":"fixture","finished_at":"fixture",
            "environment":{"fixture":true},"result":{"status":"exited","exit_code":0},
            "output":{"path":"saida.bin","streams":"stdout+stderr","bytes":output.len(),"sha256":format!("{:x}",Sha256::digest(output))},
            "sources":[{"path":"source","resolved_path":dir.join("source"),"before_sha256":hash,"after_sha256":hash,"comparison":"equal"}]});
        let bytes = serde_json::to_vec(&record).unwrap();
        fs::write(dir.join("execucao.json"), &bytes).unwrap();
        evidence.sources[0].path = Some(dir.join("saida.bin").to_string_lossy().into_owned());
        let link = evidence.sources[0].capture.as_mut().unwrap();
        link.path = dir.join("execucao.json").to_string_lossy().into_owned();
        link.sha256 = format!("{:x}", Sha256::digest(&bytes));
        CaptureFixture { evidence, dir }
    }
    #[test]
    fn capture_fixture_rejects_changed_current_source() {
        let e = capture_dossier();
        validate_capture_link(&e.sources[0]).unwrap();
        fs::write(e.dir.join("source"), b"changed").unwrap();
        assert!(validate_capture_link(&e.sources[0])
            .unwrap_err()
            .contains("diverge do hash posterior"));
    }
    #[test]
    fn capture_export_preserves_source_and_run_id_with_limits() {
        let e = capture_dossier();
        let text = selection_json(&e.facts[0], &e.sources[0], 0, e.id.as_deref(), None).unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["source"]["id"], "TEST_BOUNDARY_2");
        assert_eq!(
            v["source"]["capture_validation"]["record"]["run_id"],
            "RUN_EQUIVALENCE_BOUNDARY_2"
        );
        assert!(v["source"]["capture_validation"]["scope"]
            .as_str()
            .unwrap()
            .contains("Não autentica"));
        assert!(v["source"].get("execution_validation").is_none());
        assert!(v["source"].get("executed").is_none());
        let many = selections_json(&[&e.facts[0], &e.facts[1]], &e.sources, 0, None, None).unwrap();
        assert_eq!(many.matches("capture_and_source_match").count(), 2);
    }
    #[test]
    fn capture_link_rejects_wrong_run_or_record_hash() {
        let mut e = capture_dossier();
        e.sources[0].capture.as_mut().unwrap().run_id = "TEST_BOUNDARY_2".into();
        assert!(validate_capture_link(&e.sources[0])
            .unwrap_err()
            .contains("ID de execução"));
        e.sources[0].capture.as_mut().unwrap().sha256 = "0".repeat(64);
        assert!(validate_capture_link(&e.sources[0])
            .unwrap_err()
            .contains("Hash do registro"));
    }
    #[test]
    fn capture_link_rejects_wrong_source_and_tampered_lines() {
        let mut e = capture_dossier();
        e.sources[0].lines[0] = "alterada".into();
        assert!(validate_capture_link(&e.sources[0]).is_err());
        let mut e = capture_dossier();
        let bytes = fs::read("Cargo.toml").unwrap();
        e.sources[0].path = Some("Cargo.toml".into());
        e.sources[0].sha256 = Some(format!("{:x}", Sha256::digest(&bytes)));
        e.sources[0].lines = String::from_utf8(bytes)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect();
        assert!(validate_capture_link(&e.sources[0])
            .unwrap_err()
            .contains("não corresponde"));
    }
    #[test]
    fn capture_link_rejects_code_kind_and_legacy_execution() {
        let mut e = capture_dossier();
        e.sources[0].kind = Some("rust_source".into());
        assert!(validate_capture_link(&e.sources[0]).is_err());
        e.sources[0].kind = Some("test_run".into());
        e.sources[0].execution = recorded_execution_source().execution;
        assert!(validate_capture_link(&e.sources[0]).is_err());
    }

    #[test]
    fn capture_reuse_does_not_cross_exports_or_sources() {
        let mut e = capture_dossier();
        selections_json(&[&e.facts[0], &e.facts[1]], &e.sources, 1, None, None).unwrap();
        e.sources[0].capture.as_mut().unwrap().sha256 = "0".repeat(64);
        assert!(selections_json(&[&e.facts[0], &e.facts[1]], &e.sources, 1, None, None).is_err());
        let mut e = capture_dossier();
        let mut other_fixture = capture_dossier();
        let mut other = other_fixture.sources.remove(0);
        other.id = "OTHER".into();
        other.capture.as_mut().unwrap().run_id = "WRONG".into();
        e.sources.push(other);
        e.facts[1].source_id = "OTHER".into();
        assert!(selections_json(&[&e.facts[0], &e.facts[1]], &e.sources, 1, None, None).is_err());
    }

    #[test]
    fn shared_capture_still_checks_each_source() {
        let mut e = capture_dossier();
        let mut cache = CaptureCache::new();
        validate_capture_link_cached(&e.sources[0], &mut cache).unwrap();
        e.sources[0].id = "OTHER".into();
        validate_capture_link_cached(&e.sources[0], &mut cache).unwrap();
        assert_eq!(cache.len(), 1);
        e.sources[0].lines[0] = "alterada".into();
        assert!(validate_capture_link_cached(&e.sources[0], &mut cache).is_err());
        let bytes = fs::read("Cargo.toml").unwrap();
        e.sources[0].path = Some("Cargo.toml".into());
        e.sources[0].sha256 = Some(format!("{:x}", Sha256::digest(&bytes)));
        e.sources[0].lines = String::from_utf8(bytes)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect();
        assert!(validate_capture_link_cached(&e.sources[0], &mut cache)
            .unwrap_err()
            .contains("não corresponde"));
    }

    #[test]
    fn cache_key_separates_hash_run_and_location_and_does_not_save_failures() {
        let mut e = capture_dossier();
        let mut cache = CaptureCache::new();
        validate_capture_link_cached(&e.sources[0], &mut cache).unwrap();
        let link = e.sources[0].capture.as_mut().unwrap();
        let original_hash = link.sha256.clone();
        let original_run = link.run_id.clone();
        let original_path = link.path.clone();
        link.sha256 = "0".repeat(64);
        assert!(validate_capture_link_cached(&e.sources[0], &mut cache).is_err());
        let link = e.sources[0].capture.as_mut().unwrap();
        link.sha256 = original_hash;
        link.run_id = "WRONG".into();
        assert!(validate_capture_link_cached(&e.sources[0], &mut cache).is_err());
        let link = e.sources[0].capture.as_mut().unwrap();
        link.run_id = original_run;
        link.path = format!("{original_path}/missing");
        assert!(validate_capture_link_cached(&e.sources[0], &mut cache).is_err());
        assert_eq!(cache.len(), 1);
    }

    fn version_example() -> Source {
        Source {
            capture: None,
            execution: None,
            executed: None,
            git_commit: None,
            kind: None,
            id: String::from("SRC_TEST"),
            path: Some(String::from("exemplo.rs")),
            sha256: Some(format!("{:x}", Sha256::digest(b"linha original\n"))),
            lines: vec![String::from("linha original")],
        }
    }

    fn recorded_execution_source() -> Source {
        let evidence: Evidence = serde_json::from_str(include_str!(
            "../../ai/experimentos/03-tuplas/evidencias-execucao-identificada.json"
        ))
        .unwrap();
        evidence
            .sources
            .into_iter()
            .find(|source| source.id == "TEST_VECTOR_1")
            .unwrap()
    }

    #[test]
    fn accepts_execution_transcribed_from_report() {
        assert_eq!(
            validate_execution_record(&recorded_execution_source()),
            Ok(())
        );
    }

    #[test]
    fn rejects_each_execution_field_when_it_disagrees_with_report() {
        for field in ["command", "started_at", "finished_at", "exit_code"] {
            let mut source = recorded_execution_source();
            let record = source.execution.as_mut().unwrap();
            match field {
                "command" => record.command.push_str(" --nocapture"),
                "started_at" => record.started_at.push('Z'),
                "finished_at" => record.finished_at.push('Z'),
                _ => record.exit_code = 1,
            }
            let error = validate_execution_record(&source).unwrap_err();
            assert!(error.contains(&format!("execution.{} difere", field)));
        }
    }

    #[test]
    fn rejects_missing_or_duplicate_report_fields() {
        for prefix in [
            "Comando: ",
            "Início UTC: ",
            "Fim UTC: ",
            "Código de término: ",
        ] {
            let mut source = recorded_execution_source();
            let index = source
                .lines
                .iter()
                .position(|line| line.starts_with(prefix))
                .unwrap();
            let line = source.lines.remove(index);
            // Uma cópia na saída do comando não substitui o cabeçalho ausente.
            source.lines.push(line.clone());
            assert!(validate_execution_record(&source)
                .unwrap_err()
                .contains("sem campo"));
            source.lines.insert(index, line.clone());
            source.lines.insert(index, line);
            assert!(validate_execution_record(&source)
                .unwrap_err()
                .contains("repetido"));
        }
    }

    #[test]
    fn accepts_matching_nonzero_exit_code_without_claiming_success() {
        let mut source = recorded_execution_source();
        source.execution.as_mut().unwrap().exit_code = 1;
        let line = source
            .lines
            .iter_mut()
            .find(|line| line.starts_with("Código de término: "))
            .unwrap();
        *line = String::from("Código de término: 1");
        assert_eq!(validate_execution_record(&source), Ok(()));
    }

    #[test]
    fn execution_record_requires_test_run_but_legacy_sources_remain_accepted() {
        let mut source = recorded_execution_source();
        for kind in [Some("rust_source"), None] {
            source.kind = kind.map(String::from);
            assert!(validate_execution_record(&source).is_err());
        }
        source.execution = None;
        assert_eq!(validate_execution_record(&source), Ok(()));
    }

    #[test]
    fn exports_specific_execution_instead_of_legacy_flag() {
        let (fact, _) = example();
        for exit_code in [0, 1] {
            let record = serde_json::json!({
                "id":"RUN_1", "command":"cargo test example",
                "started_at":"2026-09-17T02:00:00Z",
                "finished_at":"2026-09-17T02:00:01Z", "exit_code":exit_code
            });
            let source: Source = serde_json::from_value(serde_json::json!({
                "id":"S1", "kind":"test_run", "executed":true,
                "execution":record, "lines":[
                    "Comando: cargo test example",
                    "Início UTC: 2026-09-17T02:00:00Z",
                    "Fim UTC: 2026-09-17T02:00:01Z",
                    format!("Código de término: {}", exit_code)
                ]
            }))
            .unwrap();
            let value: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap())
                    .unwrap();
            assert_eq!(value["source"]["execution"], record);
            assert!(value["source"].get("executed").is_none());
        }
    }

    #[test]
    fn export_reports_only_execution_fields_actually_compared() {
        let mut source = recorded_execution_source();
        let (mut fact, _) = example();
        fact.source_id = source.id.clone();
        // O ID é atribuído no catálogo, sem correspondente no relatório.
        source.execution.as_mut().unwrap().id = String::from("OUTRO_ID");
        let output = selection_json(&fact, &source, 0, None, None).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        let validation = &value["source"]["execution_validation"];
        assert_eq!(validation["status"], "matches_report_header");
        assert_eq!(
            validation["compared_fields"],
            serde_json::json!(["command", "started_at", "finished_at", "exit_code"])
        );
        source.execution.as_mut().unwrap().exit_code = 1;
        assert!(selection_json(&fact, &source, 0, None, None).is_err());
    }

    #[test]
    fn rejects_incomplete_execution_record() {
        let input = serde_json::json!({
            "id":"S1", "kind":"test_run", "lines":[], "execution":{"id":"RUN_1"}
        });
        assert!(serde_json::from_value::<Source>(input).is_err());
    }

    #[test]
    fn rust_source_does_not_export_legacy_execution_flag() {
        let (fact, _) = example();
        for executed in [Some(false), Some(true), None] {
            let source: Source = serde_json::from_value(serde_json::json!({
                "id":"S1", "kind":"rust_source", "executed":executed, "lines":["trecho"]
            }))
            .unwrap();
            let value: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap())
                    .unwrap();
            assert!(value["source"].get("executed").is_none());
            assert!(value["source"]["execution_scope"].is_string());
            assert!(value["source"].get("execution_validation").is_none());
            assert_eq!(source.executed, executed);
        }
    }

    #[test]
    fn test_run_still_preserves_declared_execution_flag() {
        let (fact, _) = example();
        let source: Source = serde_json::from_value(serde_json::json!({
            "id":"S1", "kind":"test_run", "executed":true, "lines":["registro"]
        }))
        .unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap()).unwrap();
        assert_eq!(value["source"]["executed"], true);
        assert!(value["source"]["execution"].is_null());
        assert_eq!(
            value["source"]["execution_validation"]["status"],
            "no_execution_record"
        );
        assert_eq!(
            value["source"]["execution_validation"]["compared_fields"],
            serde_json::json!([])
        );
    }

    #[test]
    fn selects_multiple_facts_in_requested_order_and_rejects_invalid_ids() {
        let (first, _) = example();
        let (mut second, _) = example();
        second.id = String::from("F2");
        let facts = vec![first, second];
        let selected = select_facts(&facts, &["F2", "F1"]).unwrap();
        assert_eq!(
            selected
                .iter()
                .map(|fact| fact.id.as_str())
                .collect::<Vec<_>>(),
            vec!["F2", "F1"]
        );
        for ids in [vec![], vec!["F1", "F1"], vec!["F1", "MISSING"]] {
            assert!(select_facts(&facts, &ids).is_err());
        }
    }

    #[test]
    fn single_selection_keeps_existing_json_format() {
        let (fact, source) = example();
        let expected = selection_json(&fact, &source, 4, Some("D1"), None).unwrap();
        assert_eq!(
            selections_json(&[&fact], &[source], 4, Some("D1"), None).unwrap(),
            expected
        );
    }

    #[test]
    fn multiple_selections_preserve_code_and_execution_roles() {
        let evidence: Evidence = serde_json::from_str(include_str!(
            "../../ai/experimentos/03-tuplas/evidencias-execucao-identificada.json"
        ))
        .unwrap();
        let selected = select_facts(
            &evidence.facts,
            &["F_VECTOR_TEST_X", "F_VECTOR_TEST_PASSED"],
        )
        .unwrap();
        let output = selections_json(
            &selected,
            &evidence.sources,
            4,
            evidence.id.as_deref(),
            evidence.unknowns.as_deref(),
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        for (item, fact) in value["selections"]
            .as_array()
            .unwrap()
            .iter()
            .zip(&selected)
        {
            let source = find_source(&evidence.sources, &fact.source_id).unwrap();
            let expected: serde_json::Value = serde_json::from_str(
                &selection_json(
                    fact,
                    source,
                    4,
                    evidence.id.as_deref(),
                    evidence.unknowns.as_deref(),
                )
                .unwrap(),
            )
            .unwrap();
            let context_id = item["source"]["context"]["context_id"].as_str().unwrap();
            let block = value["contexts"]
                .as_array()
                .unwrap()
                .iter()
                .find(|block| block["id"] == context_id)
                .unwrap();
            assert_eq!(block["source_id"], source.id);
            let mut reconstructed = item.clone();
            let context = reconstructed["source"]["context"].as_object_mut().unwrap();
            context.remove("context_id");
            context.insert("lines".into(), block["lines"].clone());
            assert_eq!(reconstructed, expected);
        }
        assert_eq!(value["selections"].as_array().unwrap().len(), 2);
        assert_eq!(value["selections"][0]["source"]["kind"], "rust_source");
        assert_eq!(
            value["selections"][1]["source"]["execution_validation"]["status"],
            "matches_report_header"
        );
        let query: serde_json::Value =
            serde_json::from_str(&query_json("O que cada fonte informa?", &output).unwrap())
                .unwrap();
        assert_eq!(query["evidence"], value);
    }

    #[test]
    fn merges_overlapping_adjacent_and_nested_ranges_but_preserves_gaps() {
        assert_eq!(
            merge_ranges(vec![(8, 10), (2, 5), (0, 3), (1, 2), (5, 6)]),
            vec![(0, 6), (8, 10)]
        );
        assert!(merge_ranges(vec![]).is_empty());
    }

    #[test]
    fn shared_context_preserves_each_requested_window_and_fact_order() {
        let (_, mut source) = example();
        source.lines = (1..=12).map(|n| format!("linha {}", n)).collect();
        let facts: Vec<Fact> = [5, 3, 11]
            .into_iter()
            .enumerate()
            .map(|(i, line)| Fact {
                id: format!("F{}", i),
                statement: format!("Ficha {}", i),
                authorship: None,
                source_id: source.id.clone(),
                line,
            })
            .collect();
        let refs: Vec<&Fact> = facts.iter().collect();
        let sources = [source];
        for radius in [0, 1, usize::MAX] {
            let output = selections_json(&refs, &sources, radius, None, None).unwrap();
            assert_eq!(
                output,
                selections_json(&refs, &sources, radius, None, None).unwrap()
            );
            let value: serde_json::Value = serde_json::from_str(&output).unwrap();
            let blocks = value["contexts"].as_array().unwrap();
            assert_eq!(
                blocks.len(),
                if radius == usize::MAX {
                    1
                } else if radius == 1 {
                    2
                } else {
                    3
                }
            );
            for (item, fact) in value["selections"].as_array().unwrap().iter().zip(&facts) {
                assert_eq!(item["fact_id"], fact.id);
                assert_eq!(item["source"]["line"], fact.line);
                assert_eq!(item["source"]["excerpt"], sources[0].lines[fact.line - 1]);
                let window = &item["source"]["context"];
                assert!(window.get("lines").is_none());
                let block = blocks
                    .iter()
                    .find(|block| block["id"] == window["context_id"])
                    .unwrap();
                let start = window["start_line"].as_u64().unwrap() as usize;
                let end = window["end_line"].as_u64().unwrap() as usize;
                let block_start = block["start_line"].as_u64().unwrap() as usize;
                assert!(
                    start >= block_start && end <= block["end_line"].as_u64().unwrap() as usize
                );
                let lines = block["lines"].as_array().unwrap();
                assert_eq!(
                    serde_json::json!(&lines[start - block_start..=end - block_start]),
                    serde_json::json!(&sources[0].lines[start - 1..end])
                );
            }
            for block in blocks {
                let start = block["start_line"].as_u64().unwrap() as usize;
                let end = block["end_line"].as_u64().unwrap() as usize;
                assert_eq!(
                    block["lines"],
                    serde_json::json!(&sources[0].lines[start - 1..end])
                );
                for line in start..=end {
                    assert!(facts
                        .iter()
                        .any(|fact| line >= fact.line.saturating_sub(radius).max(1)
                            && line <= fact.line.saturating_add(radius).min(12)));
                }
            }
        }
    }

    #[test]
    fn equal_text_from_different_sources_keeps_distinct_contexts() {
        let (first, source) = example();
        let (mut second, mut other) = example();
        second.id = "F2".into();
        other.id = "S2".into();
        second.source_id = other.id.clone();
        let output = selections_json(&[&first, &second], &[source, other], 0, None, None).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["contexts"].as_array().unwrap().len(), 2);
        assert_eq!(value["contexts"][0]["source_id"], "S1");
        assert_eq!(value["contexts"][1]["source_id"], "S2");
    }

    #[test]
    fn multiple_export_rejects_invalid_reference_and_ambiguous_sources() {
        let (first, source) = example();
        let (mut second, _) = example();
        second.id = String::from("F2");
        second.line = 0;
        assert!(selections_json(&[&first, &second], &[source], 0, None, None).is_err());
        let (_, source) = example();
        let (_, duplicate) = example();
        assert!(selections_json(&[&first], &[source, duplicate], 0, None, None).is_err());
        assert!(selections_json(&[&first], &[], 0, None, None).is_err());
    }

    #[test]
    fn query_preserves_question_and_evidence_as_separate_data() {
        let question = "O que este trecho mostra?\n";
        let selection = r#"{"fact_id":"F1","statement":"Texto com \"aspas\""}"#;
        let value: serde_json::Value =
            serde_json::from_str(&query_json(question, selection).unwrap()).unwrap();
        assert_eq!(value["question"], question);
        assert_eq!(
            value["evidence"],
            serde_json::from_str::<serde_json::Value>(selection).unwrap()
        );
        assert!(value["instructions"].is_array());
    }

    #[test]
    fn rejects_blank_question_or_malformed_selection() {
        assert!(query_json(" \n\t", "{}").is_err());
        assert!(query_json("Pergunta", "{").is_err());
    }

    #[test]
    fn preserves_dossier_unknowns_without_filtering_or_rewording() {
        let input = serde_json::json!({"sources":[], "facts":[],
            "unknowns":["Não há benchmark.", "Outra questão do dossiê."]});
        let evidence: Evidence = serde_json::from_value(input).unwrap();
        let (fact, source) = example();
        let json = selection_json(&fact, &source, 0, None, evidence.unknowns.as_deref()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            value["evidence_unknowns"],
            serde_json::json!(["Não há benchmark.", "Outra questão do dossiê."])
        );
    }

    #[test]
    fn distinguishes_missing_unknowns_from_explicit_empty_list() {
        let (fact, source) = example();
        for (input, expected) in [
            (r#"{"sources":[],"facts":[]}"#, serde_json::Value::Null),
            (
                r#"{"sources":[],"facts":[],"unknowns":[]}"#,
                serde_json::json!([]),
            ),
        ] {
            let evidence: Evidence = serde_json::from_str(input).unwrap();
            let json =
                selection_json(&fact, &source, 0, None, evidence.unknowns.as_deref()).unwrap();
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(value["evidence_unknowns"], expected);
        }
    }

    #[test]
    fn preserves_dossier_id_alongside_local_fact_id() {
        for id in ["dossie-a", "dossie-b"] {
            let input = serde_json::json!({
                "id":id,
                "sources":[{"id":"S1","lines":["trecho"]}],
                "facts":[{"id":"F1","statement":"Afirmação","source_id":"S1","line":1}]
            });
            let evidence: Evidence = serde_json::from_value(input).unwrap();
            let output = selection_json(
                &evidence.facts[0],
                &evidence.sources[0],
                0,
                evidence.id.as_deref(),
                evidence.unknowns.as_deref(),
            )
            .unwrap();
            let value: serde_json::Value = serde_json::from_str(&output).unwrap();
            assert_eq!(value["evidence_id"], id);
            assert_eq!(value["fact_id"], "F1");
        }
    }

    #[test]
    fn missing_dossier_id_remains_unknown() {
        let evidence: Evidence = serde_json::from_str(r#"{"sources":[],"facts":[]}"#).unwrap();
        assert!(evidence.id.is_none());
        let (fact, source) = example();
        let output = selection_json(
            &fact,
            &source,
            0,
            evidence.id.as_deref(),
            evidence.unknowns.as_deref(),
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(value["evidence_id"].is_null());
    }

    #[test]
    fn preserves_execution_true_false_and_missing_as_distinct_values() {
        let (fact, _) = example();
        for executed in [Some(true), Some(false), None] {
            let input = serde_json::json!({"id": "S1", "lines": ["trecho"], "executed": executed});
            let source: Source = serde_json::from_value(input).unwrap();
            let exported: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap())
                    .unwrap();
            assert_eq!(exported["source"]["executed"], serde_json::json!(executed));
        }
        let source: Source = serde_json::from_str(r#"{"id":"S1","lines":["trecho"]}"#).unwrap();
        assert_eq!(source.executed, None);
    }

    #[test]
    fn preserves_declared_authorship_and_commit() {
        let fact: Fact = serde_json::from_value(serde_json::json!({
            "id":"F1", "statement":"Afirmação", "source_id":"S1", "line":1,
            "authorship":"manual"
        }))
        .unwrap();
        let source: Source = serde_json::from_value(serde_json::json!({
            "id":"S1", "lines":["trecho"], "git_commit":"commit-declarado"
        }))
        .unwrap();
        let exported: serde_json::Value =
            serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap()).unwrap();
        assert_eq!(exported["authorship"], "manual");
        assert_eq!(exported["source"]["git_commit"], "commit-declarado");
    }

    #[test]
    fn rejects_text_instead_of_boolean_execution_metadata() {
        let input = serde_json::json!({"id":"S1", "lines":[], "executed":"false"});
        assert!(serde_json::from_value::<Source>(input).is_err());
    }

    #[test]
    fn preserves_declared_source_kind_from_json_to_export() {
        let (fact, _) = example();
        for kind in ["rust_source", "test_run", "pseudocode", "future_source"] {
            let input = serde_json::json!({
                "id": "S1", "kind": kind, "lines": ["Um trecho"]
            });
            let source: Source = serde_json::from_value(input).unwrap();
            let exported: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap())
                    .unwrap();
            assert_eq!(exported["source"]["kind"], kind);
        }
    }

    #[test]
    fn does_not_infer_kind_when_missing() {
        let (fact, _) = example();
        let source: Source =
            serde_json::from_str(r#"{"id":"S1","lines":["test example ... ok"]}"#).unwrap();
        assert!(source.kind.is_none());
        let exported: serde_json::Value =
            serde_json::from_str(&selection_json(&fact, &source, 0, None, None).unwrap()).unwrap();
        assert!(exported["source"]["kind"].is_null());
    }

    #[test]
    fn context_zero_and_large_radius_preserve_bounds() {
        let (mut fact, mut source) = example();
        source.lines = vec![String::from("a"), String::from("b"), String::from("c")];
        fact.line = 2;
        for (radius, expected) in [(0, vec!["b"]), (usize::MAX, vec!["a", "b", "c"])] {
            let value: serde_json::Value =
                serde_json::from_str(&selection_json(&fact, &source, radius, None, None).unwrap())
                    .unwrap();
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
            selection_json(&fact, &source, 4, None, None).unwrap(),
            selection_json(&fact, &source, 4, None, None).unwrap()
        );
    }

    #[test]
    fn exports_exact_statement_reference_and_excerpt() {
        let (fact, source) = example();
        let json = selection_json(&fact, &source, 3, None, None).unwrap();
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
                serde_json::from_str(&selection_json(&fact, &source, 3, None, None).unwrap())
                    .unwrap();
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
        assert!(selection_json(&fact, &source, 3, None, None).is_err());
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
            capture: None,
            execution: None,
            executed: None,
            git_commit: None,
            kind: None,
            path: None,
            sha256: None,
            id: String::from("S1"),
            lines: vec![String::from("Um trecho da obra.")],
        };
        let fact = Fact {
            authorship: None,
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
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
                path: None,
                sha256: None,
                id: String::from("S1"),
                lines: vec![],
            },
            Source {
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
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
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
                path: None,
                sha256: None,
                id: String::from("S1"),
                lines: vec![String::from("Primeira obra.")],
            },
            Source {
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
                path: None,
                sha256: None,
                id: String::from("S2"),
                lines: vec![],
            },
            Source {
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
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
                capture: None,
                execution: None,
                executed: None,
                git_commit: None,
                kind: None,
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

#[cfg(test)]
fn selection_json(
    fact: &Fact,
    source: &Source,
    radius: usize,
    evidence_id: Option<&str>,
    evidence_unknowns: Option<&[String]>,
) -> Result<String, String> {
    let validation = validate_capture_link(source)?;
    selection_with_capture(
        fact,
        source,
        radius,
        evidence_id,
        evidence_unknowns,
        validation,
    )
}

#[cfg(test)]
fn selections_json(
    facts: &[&Fact],
    sources: &[Source],
    radius: usize,
    evidence_id: Option<&str>,
    evidence_unknowns: Option<&[String]>,
) -> Result<String, String> {
    let mut documents = CaptureCache::new();
    let mut captures: std::collections::HashMap<String, Option<serde_json::Value>> =
        std::collections::HashMap::new();
    selections_json_with_capture(
        facts,
        sources,
        radius,
        evidence_id,
        evidence_unknowns,
        |source| match captures.entry(source.id.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => Ok(entry.get().clone()),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let validation = validate_capture_link_cached(source, &mut documents)?;
                entry.insert(validation.clone());
                Ok(validation)
            }
        },
    )
}

fn main() {
    let _metrics_report = metrics::Report;
    if std::env::args().nth(1).as_deref() == Some("--capture") {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 3 {
            eprintln!("Uso: validate_evidence --capture CAMINHO/execucao.json");
            std::process::exit(1);
        }
        match capture::validate(std::path::Path::new(&args[2])) {
            Ok(report) => println!("{report}"),
            Err(error) => {
                eprintln!("Conferência da captura falhou: {error}");
                std::process::exit(1);
            }
        }
        return;
    }

    let arguments: Vec<String> = std::env::args().collect();
    let mut radius = 3;
    let mut output_path: Option<&str> = None;
    let mut bundle_path: Option<&str> = None;
    let mut question_path: Option<&str> = None;
    let mut selected_ids = Vec::new();
    if arguments.get(2).map(String::as_str) == Some("--fact") {
        let id = arguments
            .get(3)
            .filter(|id| !id.starts_with("--"))
            .unwrap_or_else(|| {
                eprintln!("Informe o ID após --fact.");
                std::process::exit(1);
            });
        selected_ids.push(id.as_str());
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
                "--fact" => selected_ids.push(value.as_str()),
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
                "--bundle" if bundle_path.is_none() => bundle_path = Some(value.as_str()),
                "--question" if question_path.is_none() => question_path = Some(value.as_str()),
                _ => {
                    eprintln!("Opção desconhecida ou repetida: {}", arguments[position]);
                    std::process::exit(1);
                }
            }
            position += 2;
        }
    } else {
        if arguments.len() > 3 || arguments.get(2).is_some_and(|v| v.starts_with("--")) {
            eprintln!("Uso: validate_evidence [DOSSIÊ] [PARECER] ou DOSSIÊ --fact ID [--fact ID ...] [--context N] [--question ARQUIVO] [--output ARQUIVO | --bundle DIRETORIO_NOVO]");
            std::process::exit(1);
        }
    }
    if bundle_path.is_some() && (question_path.is_none() || output_path.is_some()) {
        eprintln!("--bundle exige --question e substitui --output.");
        std::process::exit(1);
    }
    if question_path.is_some() && output_path.is_none() && bundle_path.is_none() {
        eprintln!("Use --output ou --bundle para salvar a consulta com --question.");
        std::process::exit(1);
    }
    let question = match question_path {
        Some(path) => match fs::read_to_string(path) {
            Ok(text) if !text.trim().is_empty() => Some(text),
            Ok(_) => {
                eprintln!("A pergunta não pode estar vazia.");
                std::process::exit(1);
            }
            Err(error) => {
                eprintln!("Não foi possível ler a pergunta: {}", error);
                std::process::exit(1);
            }
        },
        None => None,
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
    let mut checks = SourceChecks::default();
    let prepared = if selected_ids.is_empty() {
        None
    } else {
        Some(
            prepare_query(
                &content,
                question.as_deref(),
                &selected_ids,
                radius,
                |source| checks.validate(source),
            )
            .unwrap_or_else(|error| {
                eprintln!("{error}");
                std::process::exit(1);
            }),
        )
    };
    let evidence = match &prepared {
        Some(query) => query.evidence().clone(),
        None => {
            validate_dossier(&content, |source| checks.validate(source)).unwrap_or_else(|error| {
                eprintln!("{error}");
                std::process::exit(1);
            })
        }
    };
    let sources = evidence.sources;
    let facts = evidence.facts;
    println!("Obras no acervo: {}", sources.len());
    println!("Fichas no catálogo: {}", facts.len());
    println!("Acervo sem IDs de fontes duplicados.");
    println!("Catálogo sem IDs de fatos duplicados.");
    println!(
        "Fichas verificadas: {}. Referências inválidas: 0.",
        facts.len()
    );
    if selected_ids.is_empty() {
        for fact in &facts {
            let source = find_source(&sources, &fact.source_id).expect("validated reference");
            println!("{}: {}", fact.id, fact.statement);
            println!("Origem: {}, linha {}", fact.source_id, fact.line);
            println!("Trecho encontrado: {}", source.lines[fact.line - 1]);
        }
    }

    if !selected_ids.is_empty() {
        let prepared_selection = prepared.as_ref().expect("selection prepared");
        for id in prepared_selection.selected_fact_ids() {
            let fact = find_fact(&facts, id).expect("selected fact was validated");
            // As referências e versões foram conferidas antes da seleção.
            let source =
                find_source(&sources, &fact.source_id).expect("A fonte da ficha foi validada");
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
        }
        if output_path.is_some() || bundle_path.is_some() {
            let prepared = prepared.as_ref().expect("selection prepared");
            let result = (|| -> Result<(), String> {
                if let Some(dir) = bundle_path {
                    prepared.write_bundle(
                        std::path::Path::new(dir),
                        &path,
                        question_path.expect("validado"),
                    )?;
                    return Ok(());
                }
                let json = prepared.query_json().unwrap_or(prepared.selection_json());
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output_path.expect("validado"))
                    .map_err(|error| error.to_string())?;
                writeln!(file, "{}", json).map_err(|error| error.to_string())
            })();
            match result {
                Ok(()) => println!(
                    "Seleção salva em {}",
                    bundle_path.or(output_path).expect("validado")
                ),
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
