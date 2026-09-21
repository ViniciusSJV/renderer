//! Conferência local do formato 2; não autentica execução.
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;

#[derive(Deserialize)]
struct Record {
    schema_version: u32,
    run_id: String,
    argv: Vec<String>,
    cwd: PathBuf,
    started_at: String,
    finished_at: String,
    environment: serde_json::Map<String, serde_json::Value>,
    result: serde_json::Value,
    output: Output,
    sources: Vec<Source>,
}
#[derive(Deserialize)]
struct Output {
    path: String,
    streams: String,
    bytes: u64,
    sha256: String,
}
#[derive(Deserialize)]
struct Source {
    path: PathBuf,
    resolved_path: PathBuf,
    before_sha256: String,
    after_sha256: Option<String>,
    after_error: Option<String>,
    comparison: String,
}

fn hash_is_valid(hash: &str) -> bool {
    hash.len() == 64
        && hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn digest(path: &Path) -> Result<(u64, String)> {
    super::metrics::add("digest_calls", 0);
    let mut file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err(format!("{}: esperado arquivo regular", path.display()));
    }
    let mut hash = Sha256::new();
    let mut bytes = 0;
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        super::metrics::add("digest_chunks", n);
        hash.update(&buffer[..n]);
        bytes += n as u64;
    }
    Ok((bytes, format!("{:x}", hash.finalize())))
}

fn validate_result(result: &serde_json::Value) -> Result<()> {
    let null_code = result.get("exit_code") == Some(&serde_json::Value::Null);
    let valid = match result["status"].as_str() {
        Some("exited") => {
            result["exit_code"]
                .as_i64()
                .is_some_and(|n| (0..=i32::MAX as i64).contains(&n))
                && result.get("signal").is_none()
                && result.get("error").is_none()
        }
        Some("start_failed") => {
            null_code
                && result["error"]
                    .as_str()
                    .is_some_and(|e| !e.trim().is_empty())
                && result.get("signal").is_none()
        }
        Some("signaled") => {
            null_code
                && result["signal"]
                    .as_i64()
                    .is_some_and(|n| n > 0 && n <= i32::MAX as i64)
                && result.get("error").is_none()
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err("Resultado do comando inconsistente".into())
    }
}

pub fn validate(path: &Path) -> Result<String> {
    super::metrics::add("capture_validate", 0);
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    super::metrics::add("record_validate_read", bytes.len());
    // Verificar a versão primeiro evita interpretar registros históricos como versão 2.
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if value["schema_version"] != 2 {
        return Err(
            "Somente schema_version 2 é aceito; registros antigos não são convertidos".into(),
        );
    }
    let record: Record = serde_json::from_value(value).map_err(|e| e.to_string())?;
    if record.schema_version != 2
        || record.run_id.trim().is_empty()
        || record.argv.is_empty()
        || record.argv[0].is_empty()
        || !record.cwd.is_absolute()
        || record.started_at.trim().is_empty()
        || record.finished_at.trim().is_empty()
        || record.environment.is_empty()
    {
        return Err("Cabeçalho da captura incompleto".into());
    }
    validate_result(&record.result)?;
    if record.output.path != "saida.bin"
        || record.output.streams != "stdout+stderr"
        || !hash_is_valid(&record.output.sha256)
    {
        return Err("Metadados da saída inválidos".into());
    }
    let (size, hash) = digest(&path.parent().unwrap_or(Path::new(".")).join("saida.bin"))?;
    if size != record.output.bytes || hash != record.output.sha256 {
        return Err("Saída atual diverge do tamanho ou SHA-256 registrado".into());
    }
    let mut equal = 0;
    let mut different = 0;
    let mut unavailable = 0;
    for source in &record.sources {
        let err = |message: &str| format!("{}: {message}", source.path.display());
        if source.path.as_os_str().is_empty()
            || record.cwd.join(&source.path) != source.resolved_path
            || !hash_is_valid(&source.before_sha256)
        {
            return Err(err("caminho ou hash anterior inconsistente"));
        }
        match source.comparison.as_str() {
            "equal" | "different" => {
                let after = source
                    .after_sha256
                    .as_ref()
                    .ok_or_else(|| err("hash posterior ausente"))?;
                if !hash_is_valid(after)
                    || source.after_error.is_some()
                    || (source.before_sha256 == *after) != (source.comparison == "equal")
                {
                    return Err(err("comparação declarada inconsistente com os hashes"));
                }
                let (_, current) = digest(&source.resolved_path)?;
                if current != *after {
                    return Err(err("arquivo atual diverge do hash posterior; isso não refuta a execução histórica"));
                }
                if source.comparison == "equal" {
                    equal += 1;
                } else {
                    different += 1;
                }
            }
            "unavailable" => {
                if source.after_sha256.is_some()
                    || !source
                        .after_error
                        .as_ref()
                        .is_some_and(|e| !e.trim().is_empty())
                {
                    return Err(err("indisponibilidade posterior inconsistente"));
                }
                unavailable += 1;
            }
            _ => return Err(err("comparação desconhecida")),
        }
    }
    Ok(format!("{}: registro consistente; saída conferida ({} bytes).\nFontes: {} iguais, {} diferentes, {} indisponíveis depois. Arquivos atuais conferidos: {}.\nResultado declarado: {}.\nDatas e ambiente têm presença/estrutura básica conferida; conteúdo não autenticado. Indisponibilidades históricas não são comprovadas.\nNão autentica execução nem comprova bytes compilados.", record.run_id, size, equal, different, unavailable, equal + different, record.result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Fixture(PathBuf, Value);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "verify-capture-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            std::fs::write(p.join("saida.bin"), b"abc").unwrap();
            std::fs::write(p.join("source"), b"abc").unwrap();
            let hash = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
            let v = json!({"schema_version":2,"run_id":"RUN", "argv":["example"], "cwd":p,
                "started_at":"2026-09-20T00:00:00Z", "finished_at":"2026-09-20T00:00:01Z", "environment":{"os":"linux"},
                "result":{"status":"exited","exit_code":7},
                "output":{"path":"saida.bin","streams":"stdout+stderr","bytes":3,"sha256":hash},
                "sources":[{"path":"source","resolved_path":p.join("source"),"before_sha256":hash,"after_sha256":hash,"comparison":"equal"}]});
            Self(p, v)
        }
        fn check(&self) -> Result<String> {
            let path = self.0.join("execucao.json");
            std::fs::write(&path, serde_json::to_vec(&self.1).unwrap()).unwrap();
            validate(&path)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn accepts_nonzero_and_reports_limits() {
        let f = Fixture::new();
        let report = f.check().unwrap();
        assert!(report.contains("1 iguais"));
        assert!(report.contains("Não autentica"));
    }
    #[test]
    fn rejects_output_tampering_even_with_same_size() {
        let f = Fixture::new();
        std::fs::write(f.0.join("saida.bin"), b"xyz").unwrap();
        assert!(f.check().unwrap_err().contains("Saída atual diverge"));
    }
    #[test]
    fn rejects_incorrect_output_size() {
        let mut f = Fixture::new();
        f.1["output"]["bytes"] = json!(4);
        assert!(f.check().is_err());
    }
    #[test]
    fn rejects_current_source_drift_and_missing_source() {
        let f = Fixture::new();
        std::fs::write(f.0.join("source"), b"xyz").unwrap();
        assert!(f.check().unwrap_err().contains("arquivo atual diverge"));
        std::fs::remove_file(f.0.join("source")).unwrap();
        assert!(f.check().is_err());
    }
    #[test]
    fn checks_declared_comparison_but_accepts_real_difference() {
        let mut f = Fixture::new();
        f.1["sources"][0]["comparison"] = json!("different");
        assert!(f.check().is_err());
        f.1["sources"][0]["before_sha256"] = json!("0".repeat(64));
        assert!(f.check().unwrap().contains("1 diferentes"));
    }
    #[test]
    fn unavailable_is_not_promoted_to_verified_file() {
        let mut f = Fixture::new();
        let s = &mut f.1["sources"][0];
        s["comparison"] = json!("unavailable");
        s["after_sha256"] = Value::Null;
        assert!(f.check().is_err());
        f.1["sources"][0]["after_error"] = json!("ausente");
        assert!(f.check().unwrap().contains("Arquivos atuais conferidos: 0"));
    }
    #[test]
    fn rejects_versions_paths_and_invalid_hashes() {
        let mut f = Fixture::new();
        let original = f.1.clone();
        for version in [1, 3] {
            f.1["schema_version"] = json!(version);
            assert!(f.check().is_err());
        }
        f.1 = original.clone();
        f.1["output"]["path"] = json!("../saida.bin");
        assert!(f.check().is_err());
        f.1 = original.clone();
        f.1["sources"][0]["resolved_path"] = json!("/wrong");
        assert!(f.check().is_err());
        f.1 = original;
        f.1["sources"][0]["before_sha256"] = json!("invalid");
        assert!(f.check().is_err());
    }
    #[test]
    fn validates_result_states() {
        for r in [
            json!({"status":"exited","exit_code":0}),
            json!({"status":"start_failed","exit_code":null,"error":"missing"}),
            json!({"status":"signaled","exit_code":null,"signal":15}),
        ] {
            assert!(validate_result(&r).is_ok());
        }
        for r in [
            json!({"status":"exited","exit_code":null}),
            json!({"status":"start_failed","exit_code":0,"error":"missing"}),
            json!({"status":"signaled","exit_code":null,"signal":0}),
            json!({"status":"start_failed","error":"missing"}),
        ] {
            assert!(validate_result(&r).is_err());
        }
    }
}
