//! 一時計測ハーネス(長トークンの挙動)。実装完了後は削除または examples/perf_probe として整理。
//! 実行: cargo run --release --example perf_probe --
use std::time::Instant;

use lojban::parse;

fn chain(chars: usize) -> String {
    let base = "kaprenmit";
    let reps = chars / base.len();
    let mut s = base.repeat(reps);
    s.push_str(&base[..chars - s.len()]);
    s
}

fn main() {
    let mut sizes = vec![20usize, 40, 54, 60, 72, 80];
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        sizes = args[1].split(',').filter_map(|x| x.parse().ok()).collect();
    }
    for n in sizes {
        let s = chain(n);
        let t = Instant::now();
        let r = parse(&s);
        let dt = t.elapsed();
        println!(
            "len={n:>6} time={:>10.4?} result={}",
            dt,
            if r.is_ok() { "ok" } else { "err" }
        );
    }
}
