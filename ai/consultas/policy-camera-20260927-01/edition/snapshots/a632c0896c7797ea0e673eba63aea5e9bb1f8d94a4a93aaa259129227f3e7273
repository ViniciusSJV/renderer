use std::{
    fs,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);

#[test]
fn questions_retrieve_expected_first_hit_or_no_evidence() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog =
        librarian_ingest::Catalog::load(&root.join("ai/acervo/renderer-auto-v1/edition")).unwrap();
    let lexicon =
        serde_json::from_slice(include_bytes!("../../../ai/acervo/renderer-lexicon.json")).unwrap();
    let index = librarian_ingest::search::SearchIndex::new(&catalog, lexicon).unwrap();
    let questions: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../../ai/acervo/renderer-search-v1/questions.json"
    ))
    .unwrap();
    for case in questions {
        let question = case["question"].as_str().unwrap();
        let report = index.search(question, 5, 2).unwrap();
        match case["expected_name"].as_str() {
            Some(expected) => {
                let hit = report.hits.first().expect(question);
                assert_eq!(hit.name, expected, "{question}");
                assert_eq!(hit.path, case["expected_path"].as_str().unwrap());
                assert!(!report.graph.unwrap().edges.is_empty());
            }
            None => assert!(report.hits.is_empty(), "{question}"),
        }
    }
}

#[test]
fn search_defaults_to_renderer_vocabulary_but_allows_another_project() {
    let input = vec!["search".into(), "edition".into(), "camera".into()];
    assert_eq!(
        super::renderer_args(input)[3],
        "ai/acervo/renderer-lexicon.json"
    );
    let explicit = vec![
        "search".into(),
        "edition".into(),
        "camera".into(),
        "other.json".into(),
        "2".into(),
        "0".into(),
    ];
    assert_eq!(super::renderer_args(explicit.clone()), explicit);
}

#[test]
fn consumer_generates_verifies_and_detects_current_divergence() {
    let root = std::env::temp_dir().join(format!(
        "renderer-catalog-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join("src")).unwrap();
    fs::create_dir(root.join("tests")).unwrap();
    fs::create_dir_all(root.join("ai/acervo")).unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn ray_for_pixel() {}\n").unwrap();
    fs::write(root.join("Cargo.toml"), "fixture").unwrap();
    fs::write(root.join("Cargo.lock"), "fixture").unwrap();
    fs::write(
        root.join("ai/acervo/renderer-sources.json"),
        include_bytes!("../../../ai/acervo/renderer-sources.json"),
    )
    .unwrap();
    let edition = root.join("edition").to_str().unwrap().to_owned();
    let mut generate_args = super::renderer_args(vec!["generate".into(), edition.clone()]);
    assert_eq!(generate_args[1], ".");
    assert_eq!(generate_args[2], "ai/acervo/renderer-sources.json");
    generate_args[1] = root.to_str().unwrap().into();
    generate_args[2] = root.join(&generate_args[2]).to_str().unwrap().into();
    let run = |args: &[&str]| {
        librarian_ingest::cli::run(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    };
    assert_eq!(librarian_ingest::cli::run(&generate_args).unwrap(), 0);
    assert_eq!(
        run(&["verify", &edition, root.to_str().unwrap()]).unwrap(),
        0
    );
    assert_eq!(run(&["show", &edition, "ray_for_pixel"]).unwrap(), 0);
    assert!(librarian_ingest::cli::run(&generate_args).is_err());
    fs::write(root.join("src/lib.rs"), "pub fn changed() {}\n").unwrap();
    assert_eq!(run(&["verify", &edition]).unwrap(), 0);
    assert_eq!(
        run(&["verify", &edition, root.to_str().unwrap()]).unwrap(),
        2
    );
    fs::remove_dir_all(root).unwrap();
}
