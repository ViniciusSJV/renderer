//! Prepara o corpo JSON de /api/generate. Não envia nem reconfere evidências.
mod ollama_common;
use ollama_common::request;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("Uso: prepare_ollama CONSULTA MODELO DESTINO_NOVO.json".into());
    }
    let query = fs::read_to_string(&args[1])?;
    let body = request(&query, &args[2])?;
    let bytes = serde_json::to_vec_pretty(&body)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("Requisição preparada; nenhum envio realizado. Não reconfere as evidências.");
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Falha na preparação: {e}");
        std::process::exit(1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_exact_prompt_and_deterministic_body() {
        let query = "{\n\"question\":\"Explique a câmera.\",\"evidence\":{},\"instructions\":[\"Cite IDs.\"]}\n";
        let a = request(query, "renderer-analyst").unwrap();
        assert_eq!(a["prompt"], query);
        assert_eq!(a["stream"], false);
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&request(query, "renderer-analyst").unwrap()).unwrap()
        );
        assert!(a.get("system").is_none());
        assert!(a.get("options").is_none());
    }
    #[test]
    fn model_selection_does_not_change_prompt() {
        let q = r#"{"question":"Q","evidence":{},"instructions":["I"]}"#;
        let a = request(q, "renderer-analyst").unwrap();
        let b = request(q, "modelo-alternativo").unwrap();
        assert_eq!(a["prompt"], b["prompt"]);
        assert_ne!(a["model"], b["model"]);
    }
    #[test]
    fn rejects_invalid_input() {
        for q in [
            "invalid",
            "{}",
            r#"{"question":" ","evidence":{},"instructions":["I"]}"#,
            r#"{"question":"Q","evidence":{},"instructions":[1]}"#,
        ] {
            assert!(request(q, "renderer-analyst").is_err());
        }
        assert!(request(
            r#"{"question":"Q","evidence":{},"instructions":["I"]}"#,
            " "
        )
        .is_err());
    }
}
