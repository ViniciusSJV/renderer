//! Renderer source selection; extraction and verification belong to Librarian.
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args = renderer_args(args);
    match librarian_ingest::cli::run(&args) {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn renderer_args(args: Vec<String>) -> Vec<String> {
    if args.first().is_some_and(|a| a == "search") && args.len() == 3 {
        let mut args = args;
        args.push("ai/acervo/renderer-lexicon.json".into());
        return args;
    }
    if args.first().is_some_and(|a| a == "generate") && args.len() == 2 {
        vec![
            "generate".into(),
            ".".into(),
            "ai/acervo/renderer-sources.json".into(),
            args[1].clone(),
        ]
    } else {
        args
    }
}

#[cfg(test)]
#[path = "catalog_sources/tests.rs"]
mod tests;
