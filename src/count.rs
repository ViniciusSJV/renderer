//! Contagem de chamadas para o Librarian, so com a feature `librarian-count`. Cada `count!()`
//! conta quantas vezes a sua linha rodou; `report()` imprime cada contagem no formato da Regua,
//! `librarian-measure calls=N at=src/arquivo.rs:linha`. Sem a feature, `count!()` nao gera
//! codigo e `report()` nao imprime nada.
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

pub struct Site {
    file: &'static str,
    line: u32,
    calls: AtomicU64,
    registered: AtomicBool,
}

static SITES: Mutex<Vec<&'static Site>> = Mutex::new(Vec::new());

impl Site {
    pub const fn new(file: &'static str, line: u32) -> Self {
        Site { file, line, calls: AtomicU64::new(0), registered: AtomicBool::new(false) }
    }

    pub fn hit(&'static self) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if !self.registered.swap(true, Ordering::Relaxed) {
            SITES.lock().unwrap().push(self);
        }
    }
}

/// Conta uma passagem por esta linha (so com a feature `librarian-count`).
#[macro_export]
macro_rules! count {
    () => {
        #[cfg(feature = "librarian-count")]
        {
            static SITE: $crate::count::Site = $crate::count::Site::new(file!(), line!());
            SITE.hit();
        }
    };
}

/// As contagens, uma linha por `count!()` que rodou, em ordem de arquivo e linha.
pub fn lines() -> Vec<String> {
    let mut sites: Vec<(String, u32, u64)> = SITES
        .lock()
        .unwrap()
        .iter()
        .map(|s| (s.file.replace('\\', "/"), s.line, s.calls.load(Ordering::Relaxed)))
        .collect();
    sites.sort();
    sites.into_iter().map(|(file, line, calls)| format!("librarian-measure calls={calls} at={file}:{line}")).collect()
}

/// Zera as contagens: o que vier depois conta sozinho (por exemplo, so o render, sem a montagem
/// da cena).
pub fn reset() {
    for site in SITES.lock().unwrap().iter() {
        site.calls.store(0, Ordering::Relaxed);
    }
}

pub fn report() {
    for line in lines() {
        println!("{line}");
    }
}

#[cfg(all(test, feature = "librarian-count"))]
mod tests_count {
    #[test]
    fn each_counted_line_reports_how_many_times_it_ran() {
        fn counted() {
            crate::count!();
        }
        for _ in 0..3 {
            counted();
        }
        let line = line!() - 5;
        assert!(super::lines().contains(&format!("librarian-measure calls=3 at=src/count.rs:{line}")));
    }
}
