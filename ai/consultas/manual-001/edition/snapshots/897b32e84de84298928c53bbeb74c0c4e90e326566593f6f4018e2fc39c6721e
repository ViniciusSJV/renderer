//! Confere/exporta e envia na mesma invocação; não executa sugestões do modelo.
#[path = "ollama_common/client.rs"]
mod client;
mod ollama_common;
use client::{hash, save, save_json, Config, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    request_id: String,
    project_root: PathBuf,
    dossier: PathBuf,
    question: PathBuf,
    rubric: PathBuf,
    facts: Vec<String>,
    context: usize,
    endpoint: String,
    model: String,
    timeout_ms: u64,
    max_response_bytes: usize,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rubric {
    id: String,
    criteria: Vec<Criterion>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Criterion {
    id: String,
    description: String,
    expected: String,
}
fn validate_rubric(bytes: &[u8]) -> Result<()> {
    let r: Rubric = serde_json::from_slice(bytes)?;
    let mut ids = HashSet::new();
    if r.id.trim().is_empty()
        || r.criteria.is_empty()
        || r.criteria.iter().any(|c| {
            c.id.trim().is_empty()
                || c.description.trim().is_empty()
                || c.expected.trim().is_empty()
                || !ids.insert(&c.id)
        })
    {
        return Err(
            "Rubrica exige ID e critérios únicos com description/expected não vazios".into(),
        );
    }
    Ok(())
}
fn prepare(input: &[u8], dir: &Path, validator: &Path) -> Result<(Input, Config, Value)> {
    let cfg: Input = serde_json::from_slice(input)?;
    let transport = Config {
        endpoint: cfg.endpoint.clone(),
        model: cfg.model.clone(),
        timeout_ms: cfg.timeout_ms,
        max_bytes: cfg.max_response_bytes,
    };
    transport.validate()?;
    let mut ids = HashSet::new();
    if cfg.request_id.trim().is_empty()
        || cfg.facts.is_empty()
        || cfg
            .facts
            .iter()
            .any(|s| s.trim().is_empty() || s.starts_with("--") || !ids.insert(s))
    {
        return Err("request_id e fichas únicas explícitas são obrigatórios".into());
    }
    let root = fs::canonicalize(&cfg.project_root)?;
    let rubric = fs::read(root.join(&cfg.rubric))?;
    validate_rubric(&rubric)?;
    // The rubric is frozen before validation and never included in the model prompt.
    save(dir, "rubric.json", &rubric)?;
    save(dir, "config.json", input)?;
    let mut command = Command::new(validator);
    command.current_dir(&root).arg(root.join(&cfg.dossier));
    for id in &cfg.facts {
        command.arg("--fact").arg(id);
    }
    command
        .arg("--context")
        .arg(cfg.context.to_string())
        .arg("--question")
        .arg(root.join(&cfg.question))
        .arg("--bundle")
        .arg(dir.join("bundle"));
    let start = Instant::now();
    let output = command.output()?;
    save(dir, "validation.stdout", &output.stdout)?;
    save(dir, "validation.stderr", &output.stderr)?;
    save_json(
        dir,
        "validation.json",
        &json!({"exit_code":output.status.code(),"elapsed_ms":start.elapsed().as_secs_f64()*1000.0,
        "validator":validator,"working_directory":root,"stdout_sha256":hash(&output.stdout),"stderr_sha256":hash(&output.stderr)}),
    )?;
    if !output.status.success() {
        return Err(
            "Conferência/exportação falhou; consulte validation.stderr. Nenhum envio realizado"
                .into(),
        );
    }
    let origin_bytes = fs::read(dir.join("bundle/origin.json"))?;
    let origin: Value = serde_json::from_slice(&origin_bytes)?;
    if origin["format_version"] != 1
        || origin["kind"] != "validated_query_export"
        || origin["selection"]["fact_ids"] != json!(cfg.facts)
        || origin["selection"]["context"] != cfg.context
    {
        return Err("Origem/seleção incompatível com a invocação".into());
    }
    for (key, file) in [
        ("dossier", "dossier.json"),
        ("question", "question.txt"),
        ("query", "query.json"),
    ] {
        let bytes = fs::read(dir.join("bundle").join(file))?;
        if origin[key]["sha256"].as_str() != Some(hash(&bytes).as_str()) {
            return Err("Hash do pacote divergente".into());
        }
    }
    let link = json!({"query_sha256":origin["query"]["sha256"],"dossier_origin":origin["dossier"],"selection":origin["selection"],
        "origin":{"file":"../bundle/origin.json","sha256":hash(&origin_bytes)},
        "rubric":{"file":"../rubric.json","sha256":hash(&rubric)},
        "files_base":"Diretório transport; arquivos dossier/question/query da origem ficam em ../bundle",
        "limits":"Conferência nesta invocação, sem snapshot atômico; não autentica execução nem valida semântica."});
    save_json(
        dir,
        "ready.json",
        &json!({"request_id":cfg.request_id,"link":link,"semantic_evaluation":"pending"}),
    )?;
    Ok((cfg, transport, link))
}
fn pipeline(config_file: &Path, destination: &Path, validator: &Path) -> Result<String> {
    fs::create_dir(destination)?;
    let dir = fs::canonicalize(destination)?;
    save_json(
        &dir,
        "started.json",
        &json!({"format_version":1,"started_unix_ms":client::now_ms(),"config_source":config_file}),
    )?;
    let started = Instant::now();
    let prepared = fs::read(config_file)
        .map_err(Into::into)
        .and_then(|input| prepare(&input, &dir, validator));
    let (cfg, transport, link) = match prepared {
        Ok(v) => v,
        Err(e) => {
            // An I/O failure leaves the pipeline partial and exits 1; never mark it complete.
            if e.downcast_ref::<std::io::Error>().is_some() {
                return Err(e);
            }
            client::finish(
                &dir,
                &json!({"format_version":1,"state":"input_rejected","sent":false,"diagnostic":e.to_string(),"elapsed_ms":started.elapsed().as_secs_f64()*1000.0}),
            )?;
            return Ok("input_rejected".into());
        }
    };
    let state = client::attempt_linked(
        &dir.join("bundle/query.json"),
        &dir.join("transport"),
        &cfg.request_id,
        &transport,
        Some(&link),
    )?;
    let result = fs::read(dir.join("transport/result.json"))?;
    client::finish(
        &dir,
        &json!({"format_version":1,"request_id":cfg.request_id,"state":state,
        "elapsed_ms":started.elapsed().as_secs_f64()*1000.0,"transport_result":{"file":"transport/result.json","sha256":hash(&result)},
        "link":link,"semantic_evaluation":"pending"}),
    )?;
    Ok(state)
}
fn run() -> Result<bool> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("Uso: explain_evidence CONFIG.json DIRETORIO_NOVO (compile validate_evidence no mesmo perfil)".into());
    }
    let validator = std::env::current_exe()?
        .with_file_name(format!("validate_evidence{}", std::env::consts::EXE_SUFFIX));
    let state = pipeline(Path::new(&args[1]), Path::new(&args[2]), &validator)?;
    println!(
        "Estado integrado: {state}; registro: {}/result.json. Avaliação semântica pendente.",
        args[2]
    );
    Ok(state == "completed")
}
fn main() {
    std::process::exit(match run() {
        Ok(true) => 0,
        Ok(false) => 2,
        Err(e) => {
            eprintln!("Falha: {e}; diretório sem result.json é parcial.");
            1
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::atomic::{AtomicUsize, Ordering},
        thread,
    };
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new(endpoint: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "integrated-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&dir).unwrap();
            fs::write(dir.join("source.txt"), b"model must be explicit\n").unwrap();
            let dossier = json!({"id":"D_TEST","sources":[{"id":"S","kind":"text","path":dir.join("source.txt"),"sha256":hash(b"model must be explicit\n"),"lines":["model must be explicit"]}],"facts":[{"id":"F","statement":"A linha exige modelo explícito.","source_id":"S","line":1}]});
            fs::write(
                dir.join("dossier.json"),
                serde_json::to_vec(&dossier).unwrap(),
            )
            .unwrap();
            fs::write(dir.join("question.txt"), "Explique e cite F/S.\n").unwrap();
            fs::write(dir.join("rubric.json"),br#"{"id":"R","criteria":[{"id":"C1","description":"Cite IDs","expected":"RUBRIC_SECRET F e S"}]}"#).unwrap();
            let cfg = json!({"request_id":"I_TEST","project_root":dir,"dossier":"dossier.json","question":"question.txt","rubric":"rubric.json","facts":["F"],"context":1,"endpoint":endpoint,"model":"simulated","timeout_ms":1000,"max_response_bytes":4096});
            fs::write(dir.join("config.json"), serde_json::to_vec(&cfg).unwrap()).unwrap();
            Self(dir)
        }
        fn run(&self) -> String {
            let validator = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target/debug")
                .join(format!("validate_evidence{}", std::env::consts::EXE_SUFFIX));
            assert!(
                validator.is_file(),
                "Compile cargo build --bin validate_evidence antes dos testes integrados"
            );
            pipeline(
                &self.0.join("config.json"),
                &self.0.join("attempt"),
                &validator,
            )
            .unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn run_success(record: Option<&Path>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let f = Fixture::new(&format!(
            "http://{}/api/generate",
            listener.local_addr().unwrap()
        ));
        let dir = f.0.join("attempt");
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();
            let mut headers = Vec::new();
            let mut b = [0];
            while !headers.ends_with(b"\r\n\r\n") {
                socket.read_exact(&mut b).unwrap();
                headers.push(b[0]);
            }
            let text = String::from_utf8(headers).unwrap();
            let len: usize = text
                .lines()
                .find_map(|l| {
                    l.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .map(|s| s.parse().unwrap())
                })
                .unwrap();
            let mut body = vec![0; len];
            socket.read_exact(&mut body).unwrap();
            assert!(dir.join("rubric.json").is_file());
            assert!(dir.join("ready.json").is_file());
            assert!(!String::from_utf8_lossy(&body).contains("RUBRIC_SECRET"));
            let req: Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(
                req["prompt"],
                fs::read_to_string(dir.join("bundle/query.json")).unwrap()
            );
            let response =
                br#"{"model":"simulated","response":"F/S","done":true,"done_reason":"stop"}"#;
            write!(
                socket,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                response.len()
            )
            .unwrap();
            socket.write_all(response).unwrap();
        });
        assert_eq!(f.run(), "completed");
        server.join().unwrap();
        let r: Value =
            serde_json::from_slice(&fs::read(f.0.join("attempt/transport/result.json")).unwrap())
                .unwrap();
        assert_eq!(r["evidence_rechecked"], true);
        assert_eq!(r["dossier_origin"]["id"], "D_TEST");
        assert_eq!(r["selection"]["fact_ids"], json!(["F"]));
        assert_eq!(
            r["integration"]["rubric"]["sha256"],
            hash(&fs::read(f.0.join("rubric.json")).unwrap())
        );
        if let Some(destination) = record {
            fn copy_tree(from: &Path, to: &Path) {
                fs::create_dir(to).unwrap();
                for entry in fs::read_dir(from).unwrap() {
                    let entry = entry.unwrap();
                    if entry.file_type().unwrap().is_dir() {
                        copy_tree(&entry.path(), &to.join(entry.file_name()));
                    } else {
                        fs::copy(entry.path(), to.join(entry.file_name())).unwrap();
                    }
                }
            }
            copy_tree(&f.0, destination);
        }
    }
    #[test]
    fn full_pipeline_links_fresh_export_and_prior_rubric_without_sending_answers() {
        run_success(None);
    }
    #[test]
    #[ignore = "Grava somente com OLLAMA_INTEGRATION_RECORD_DIR explícito e novo"]
    fn record_lesson_pipeline() {
        let path = PathBuf::from(
            std::env::var("OLLAMA_INTEGRATION_RECORD_DIR").expect("Defina diretório novo"),
        );
        run_success(Some(&path));
    }
    #[test]
    fn query_changed_after_export_is_rejected_before_http() {
        let f = Fixture::new("http://127.0.0.1:1/api/generate");
        let cfg = Config {
            endpoint: "http://127.0.0.1:1/api/generate".into(),
            model: "m".into(),
            timeout_ms: 100,
            max_bytes: 1024,
        };
        let link = json!({"query_sha256":hash(b"different bytes")});
        let state = client::attempt_linked(
            &f.0.join("question.txt"),
            &f.0.join("transport"),
            "CHANGED",
            &cfg,
            Some(&link),
        )
        .unwrap();
        assert_eq!(state, "input_rejected");
        assert!(!f.0.join("transport/request.json").exists());
    }
    #[test]
    fn changed_source_blocks_http_and_preserves_diagnostic() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let f = Fixture::new(&format!(
            "http://{}/api/generate",
            listener.local_addr().unwrap()
        ));
        fs::write(f.0.join("source.txt"), b"changed\n").unwrap();
        assert_eq!(f.run(), "input_rejected");
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        assert!(!f.0.join("attempt/transport").exists());
        assert!(!fs::read(f.0.join("attempt/validation.stderr"))
            .unwrap()
            .is_empty());
    }
    #[test]
    fn evaluated_rubric_rejected_before_validation_or_http() {
        let f = Fixture::new("http://127.0.0.1:1/api/generate");
        fs::write(
            f.0.join("rubric.json"),
            br#"{"id":"R","criteria":[],"score":3}"#,
        )
        .unwrap();
        assert_eq!(f.run(), "input_rejected");
        assert!(!f.0.join("attempt/bundle").exists());
        assert!(!f.0.join("attempt/transport").exists());
    }
    #[test]
    fn rubric_requires_unique_nonempty_criteria() {
        assert!(validate_rubric(br#"{"id":"R","criteria":[{"id":"C","description":"d","expected":"e"},{"id":"C","description":"d","expected":"e"}]}"#).is_err());
        assert!(validate_rubric(
            br#"{"id":"R","criteria":[{"id":"C","description":"","expected":"e"}]}"#
        )
        .is_err());
    }
}
