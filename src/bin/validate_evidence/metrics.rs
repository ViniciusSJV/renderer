//! Contagens lógicas da conferência, não tráfego físico do disco.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

#[derive(Default)]
struct Counts {
    calls: u64,
    bytes: u64,
}
static ENABLED: OnceLock<bool> = OnceLock::new();
static COUNTS: Mutex<BTreeMap<String, Counts>> = Mutex::new(BTreeMap::new());
fn enabled() -> bool {
    *ENABLED.get_or_init(|| std::env::var("BIBLIOTECARIO_METRICS").as_deref() == Ok("1"))
}
pub fn add(category: &str, bytes: usize) {
    if !enabled() {
        return;
    }
    let mut counts = COUNTS.lock().unwrap();
    let c = counts.entry(category.to_owned()).or_default();
    c.calls += 1;
    c.bytes += bytes as u64;
}
pub fn emit() {
    if !enabled() {
        return;
    }
    let counts = COUNTS.lock().unwrap();
    let values: BTreeMap<&str, Value> = counts
        .iter()
        .map(|(k, v)| (k.as_str(), json!({"calls":v.calls,"bytes":v.bytes})))
        .collect();
    eprintln!(
        "BIBLIOTECARIO_METRICS {}",
        json!({"schema_version":1,"counts":values,"scope":"Bytes entregues à aplicação nas leituras instrumentadas de conferência; não I/O físico. Não inclui dossiê, pergunta, parecer ou escrita. Relatório apenas ao concluir com sucesso."})
    );
}
pub struct Report;
impl Drop for Report {
    fn drop(&mut self) {
        emit();
    }
}
