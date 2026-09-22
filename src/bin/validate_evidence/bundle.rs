//! Pacote de exportação: origem dos bytes lidos, não autenticação ou snapshot de fontes.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Origin<'a> {
    pub dossier_path: &'a str,
    pub dossier: &'a str,
    pub dossier_id: Option<&'a str>,
    pub question_path: &'a str,
    pub question: &'a str,
    pub facts: &'a [&'a str],
    pub context: usize,
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn save(dir: &Path, name: &str, bytes: &[u8]) -> Result<(), String> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))
        .map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())
}
/// Called only after validation and query export succeed. origin.json is last.
pub fn write(dir: &Path, query: &[u8], origin: &Origin<'_>) -> Result<Value, String> {
    fs::create_dir(dir).map_err(|e| e.to_string())?;
    save(dir, "dossier.json", origin.dossier.as_bytes())?;
    save(dir, "question.txt", origin.question.as_bytes())?;
    save(dir, "query.json", query)?;
    let record = json!({
        "format_version":1,
        "kind":"validated_query_export",
        "export_finished_unix_ms":SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_millis(),
        "dossier":{"source_path":origin.dossier_path,"id":origin.dossier_id,"file":"dossier.json","sha256":hash(origin.dossier.as_bytes())},
        "question":{"source_path":origin.question_path,"file":"question.txt","sha256":hash(origin.question.as_bytes())},
        "selection":{"fact_ids":origin.facts,"context":origin.context},
        "query":{"file":"query.json","sha256":hash(query),"bytes":query.len()},
        "validation_scope":"Conferência pelo validate_evidence antes desta exportação. Não é conferência no momento de um envio posterior.",
        "limits":["Não autentica execução nem comprova bytes compilados.","Não é snapshot atômico das fontes.","Não valida semanticamente fichas ou respostas.","Nenhum envio HTTP realizado; rubrica e avaliação ainda não vinculadas."]
    });
    save(
        dir,
        "origin.pending.json",
        &serde_json::to_vec_pretty(&record).map_err(|e| e.to_string())?,
    )?;
    fs::rename(dir.join("origin.pending.json"), dir.join("origin.json"))
        .map_err(|e| e.to_string())?;
    Ok(record)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    #[test]
    fn preserves_bytes_selection_and_refuses_existing_destination() {
        let root = std::env::temp_dir().join(format!(
            "query-bundle-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let o = Origin {
            dossier_path: "d.json",
            dossier: "{\n}\n",
            dossier_id: None,
            question_path: "q.txt",
            question: "Câmera?\r\n",
            facts: &["F2", "F1"],
            context: 2,
        };
        let record = write(&root, b"{\"query\":true}\n", &o).unwrap();
        assert_eq!(
            fs::read(root.join("question.txt")).unwrap(),
            o.question.as_bytes()
        );
        assert_eq!(record["question"]["sha256"], hash(o.question.as_bytes()));
        assert_eq!(record["selection"]["fact_ids"], json!(["F2", "F1"]));
        assert!(record["dossier"]["id"].is_null());
        let before = fs::read(root.join("origin.json")).unwrap();
        assert!(write(&root, b"different", &o).is_err());
        assert_eq!(fs::read(root.join("origin.json")).unwrap(), before);
        fs::remove_dir_all(root).unwrap();
    }
}
