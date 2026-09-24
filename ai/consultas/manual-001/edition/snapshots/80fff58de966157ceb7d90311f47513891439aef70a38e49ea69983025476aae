use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

const DOSSIER: &str = "ai/experimentos/20-render-cpu/escena-externa/evidencias.json";
const QUESTION: &str = "ai/experimentos/20-render-cpu/escena-externa/pergunta.txt";
const FACTS: &[&str] = &[
    "F_EXTERNAL_PROTOCOL",
    "F_EXTERNAL_SAMPLE_TOTAL",
    "F_EXTERNAL_MEDIAN",
    "F_EXTERNAL_VISUAL_DIGEST",
    "F_EXTERNAL_LIMIT",
];

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "renderer-librarian-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn export(dossier: &Path, destination: &Path) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_validate_evidence"));
    command.current_dir(root()).arg(dossier);
    for fact in FACTS {
        command.args(["--fact", fact]);
    }
    command
        .args(["--context", "2", "--question", QUESTION, "--bundle"])
        .arg(destination)
        .output()
        .unwrap()
}

#[test]
fn external_dossier_preserves_reference_query_through_renderer_cli() {
    let dir = TempDir::new();
    let bundle = dir.0.join("bundle");
    let output = export(&root().join(DOSSIER), &bundle);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read(bundle.join("query.json")).unwrap(),
        include_bytes!("../ai/experimentos/21-extracao-bibliotecario/before/query.json")
    );
    assert_eq!(
        fs::read(bundle.join("dossier.json")).unwrap(),
        fs::read(root().join(DOSSIER)).unwrap()
    );
    assert_eq!(
        fs::read(bundle.join("question.txt")).unwrap(),
        fs::read(root().join(QUESTION)).unwrap()
    );
    let origin: Value =
        serde_json::from_slice(&fs::read(bundle.join("origin.json")).unwrap()).unwrap();
    assert_eq!(
        origin["query"]["sha256"],
        "520afae5122d23e42c24ad17e60a79a5f1afe0c5dae7bf78329b13204ad11075"
    );
    assert_eq!(origin["selection"]["fact_ids"], serde_json::json!(FACTS));
}

#[test]
fn modified_unselected_source_blocks_export_without_changing_previous_bundle() {
    let dir = TempDir::new();
    let mut dossier: Value =
        serde_json::from_slice(&fs::read(root().join(DOSSIER)).unwrap()).unwrap();
    let mut extra = dossier["sources"][0].clone();
    let source = dir.0.join("unselected-source.rs");
    fs::copy(root().join(extra["path"].as_str().unwrap()), &source).unwrap();
    extra["id"] = Value::String("UNSELECTED_SOURCE".into());
    extra["path"] = serde_json::to_value(&source).unwrap();
    dossier["sources"].as_array_mut().unwrap().push(extra);
    let dossier_path = dir.0.join("dossier.json");
    fs::write(&dossier_path, serde_json::to_vec(&dossier).unwrap()).unwrap();

    let previous = dir.0.join("previous");
    assert!(export(&dossier_path, &previous).status.success());
    let previous_query = fs::read(previous.join("query.json")).unwrap();
    fs::write(source, b"changed source\n").unwrap();

    let rejected = dir.0.join("rejected");
    let output = export(&dossier_path, &rejected);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("SHA-256 diferente"));
    assert!(!rejected.exists());
    assert_eq!(
        fs::read(previous.join("query.json")).unwrap(),
        previous_query
    );
}
