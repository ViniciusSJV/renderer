//! Contagem e tempo de chamadas para o Librarian, so com a feature `librarian-count`. Cada
//! `count!()` no comeco de uma funcao conta quantas vezes a sua linha rodou e quanto tempo a
//! funcao levou ate retornar; `report()` imprime, no formato da Regua,
//! `librarian-measure calls=N at=src/arquivo.rs:linha` e `librarian-measure cpu_ms=T ms at=...`.
//! Sem a feature, `count!()` nao gera codigo e `report()` nao imprime nada.
//!
//! O tempo e inclusivo (conta o que a funcao chama), somado de todas as threads do render, e
//! numa recursao so a chamada mais externa conta. Medir custa algumas dezenas de nanossegundos
//! por chamada: os totais ficam inflados; a ordem entre as funcoes e o que vale.
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread::LocalKey;
use std::time::Instant;

pub struct Site {
    file: &'static str,
    line: u32,
    calls: AtomicU64,
    nanos: AtomicU64,
    registered: AtomicBool,
}

static SITES: Mutex<Vec<&'static Site>> = Mutex::new(Vec::new());

impl Site {
    pub const fn new(file: &'static str, line: u32) -> Self {
        Site { file, line, calls: AtomicU64::new(0), nanos: AtomicU64::new(0), registered: AtomicBool::new(false) }
    }

    pub fn hit(&'static self) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if !self.registered.swap(true, Ordering::Relaxed) {
            SITES.lock().unwrap().push(self);
        }
    }
}

/// Conta a chamada e, ao sair do escopo, soma o tempo dela; numa recursao (a mesma linha ja
/// aberta nesta thread), so a chamada mais externa soma tempo.
pub struct Guard {
    site: &'static Site,
    depth: &'static LocalKey<Cell<u32>>,
    start: Option<Instant>,
}

impl Guard {
    pub fn enter(site: &'static Site, depth: &'static LocalKey<Cell<u32>>) -> Self {
        site.hit();
        let outer = depth.with(|d| {
            d.set(d.get() + 1);
            d.get() == 1
        });
        Guard { site, depth, start: outer.then(Instant::now) }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            self.site.nanos.fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
        }
        self.depth.with(|d| d.set(d.get() - 1));
    }
}

/// Conta uma passagem por esta linha e o tempo ate o fim do bloco em que esta (so com a feature
/// `librarian-count`). Fica no comeco do corpo da funcao.
#[macro_export]
macro_rules! count {
    () => {
        #[cfg(feature = "librarian-count")]
        let _librarian_count = {
            static SITE: $crate::count::Site = $crate::count::Site::new(file!(), line!());
            thread_local! { static DEPTH: ::std::cell::Cell<u32> = const { ::std::cell::Cell::new(0) }; }
            $crate::count::Guard::enter(&SITE, &DEPTH)
        };
    };
}

/// As contagens e os tempos, duas linhas por `count!()` que rodou, em ordem de arquivo e linha.
pub fn lines() -> Vec<String> {
    let mut sites: Vec<(String, u32, u64, u64)> = SITES
        .lock()
        .unwrap()
        .iter()
        .map(|s| (s.file.replace('\\', "/"), s.line, s.calls.load(Ordering::Relaxed), s.nanos.load(Ordering::Relaxed)))
        .collect();
    sites.sort();
    sites
        .into_iter()
        .flat_map(|(file, line, calls, nanos)| {
            [
                format!("librarian-measure calls={calls} at={file}:{line}"),
                format!("librarian-measure cpu_ms={:.3} ms at={file}:{line}", nanos as f64 / 1e6),
            ]
        })
        .collect()
}

/// Zera as contagens e os tempos: o que vier depois conta sozinho (por exemplo, so o render, sem
/// a montagem da cena).
pub fn reset() {
    for site in SITES.lock().unwrap().iter() {
        site.calls.store(0, Ordering::Relaxed);
        site.nanos.store(0, Ordering::Relaxed);
    }
}

pub fn report() {
    for line in lines() {
        println!("{line}");
    }
}

#[cfg(all(test, feature = "librarian-count"))]
mod tests_count {
    use std::time::Duration;

    #[test]
    fn each_counted_line_reports_how_many_times_it_ran_and_for_how_long() {
        fn counted() {
            crate::count!();
            std::thread::sleep(Duration::from_millis(2));
        }
        for _ in 0..3 {
            counted();
        }
        let line = line!() - 6;
        let lines = super::lines();
        assert!(lines.contains(&format!("librarian-measure calls=3 at=src/count.rs:{line}")));
        let time = lines.iter().find(|l| l.starts_with("librarian-measure cpu_ms=") && l.ends_with(&format!("at=src/count.rs:{line}"))).unwrap();
        let ms: f64 = time["librarian-measure cpu_ms=".len()..].split(' ').next().unwrap().parse().unwrap();
        assert!(ms >= 6.0, "{time}");
    }

    #[test]
    fn a_recursion_counts_every_call_but_the_time_once() {
        fn down(n: u32) {
            crate::count!();
            std::thread::sleep(Duration::from_millis(1));
            if n > 0 {
                down(n - 1);
            }
        }
        down(3);
        let line = line!() - 7;
        let lines = super::lines();
        assert!(lines.contains(&format!("librarian-measure calls=4 at=src/count.rs:{line}")));
        let time = lines.iter().find(|l| l.starts_with("librarian-measure cpu_ms=") && l.ends_with(&format!("at=src/count.rs:{line}"))).unwrap();
        let ms: f64 = time["librarian-measure cpu_ms=".len()..].split(' ').next().unwrap().parse().unwrap();
        // The outermost call holds the four sleeps (4 ms); counted four times it would be ~10 ms.
        assert!((4.0..8.0).contains(&ms), "{time}");
    }
}
