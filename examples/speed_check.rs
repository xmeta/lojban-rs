//! 同一文章での簡易速度測定(JS 実装との比較用)
//!
//! 実行: `cargo run --release --example speed_check`

use std::time::Instant;

use lojban::parse;

fn main() {
    let sentences = [
        ("short", "mi klama do"),
        ("medium", "le gerku poi cadzu ku'o cu batci le mlatu"),
        ("complex", "mi viska le re ci gerku gi'e cusku do"),
    ];
    for (name, s) in sentences {
        for _ in 0..200 {
            parse(s).unwrap(); // warmup
        }
        let n = 3000u32;
        let t = Instant::now();
        for _ in 0..n {
            parse(s).unwrap();
        }
        let us = t.elapsed().as_nanos() as f64 / 1000.0 / n as f64;
        println!("lojban(rust)\t{name}\t{us:.1}");
    }
}
