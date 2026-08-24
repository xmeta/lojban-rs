//! 解析性能のベンチマーク(criterion)
//!
//! 実行: `cargo bench`

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lojban::grammar::{LojbanParser, Rule};
use lojban::{parse, tree};
use pest::Parser;

fn configure(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>) {
    // 全体の実行時間を抑えるため短めに設定
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(3));
}

fn bench_battery(c: &mut Criterion) {
    let mut g = c.benchmark_group("battery");
    configure(&mut g);
    // 混成実文(バッテリー回帰スイートから抽出): 実用的ワークロードの指標
    let mixed = [
        "ganai lo gerku poi mi viska ke'a ku'o cu batci fi'o spaji fe'u do gi mi cusku lu xamgu li'u",
        "mi pu ze'a ca le cabdei troci lo ka su'o da zo'u da se prami",
        "le nanmu noi pu zi je'a tavla bau la lojban. cu se jimpe mi",
        "li vei ny su'i pa ve'o pi'i xa du li xa",
        "la'edi'u se pi'o lo nu lo cipni vo'a cliva",
    ]
    .join(" .i ");
    g.bench_function("mixed_5sentences", |b| {
        b.iter(|| parse(black_box(&mixed)).unwrap())
    });
    g.finish();
}

fn bench_parse(c: &mut Criterion) {
    let mut g = c.benchmark_group("parse");
    configure(&mut g);
    g.bench_function("短文_mi_klama_do", |b| {
        b.iter(|| LojbanParser::parse(Rule::text, black_box("mi klama do")).unwrap())
    });
    g.bench_function("描述_関係節", |b| {
        b.iter(|| {
            LojbanParser::parse(
                Rule::text,
                black_box("le gerku poi cadzu ku'o cu batci le mlatu"),
            )
            .unwrap()
        })
    });
    g.bench_function("複合_接続詞引用mex", |b| {
        b.iter(|| {
            parse(black_box(
                "ge mi viska le re su'i ci gerku gi mi cusku zoi .ky. hello .ky.",
            ))
            .unwrap()
        })
    });
    g.finish();
}

fn bench_morphology(c: &mut Criterion) {
    let mut g = c.benchmark_group("morphology");
    configure(&mut g);
    g.bench_function("gismu", |b| {
        b.iter(|| LojbanParser::parse(Rule::BRIVLA_clause, black_box("gerku")).unwrap())
    });
    g.bench_function("lujvo", |b| {
        b.iter(|| LojbanParser::parse(Rule::BRIVLA_clause, black_box("gerzda")).unwrap())
    });
    g.finish();
}

fn bench_output(c: &mut Criterion) {
    let input = "le gerku poi cadzu ku'o cu batci le mlatu";
    let mut g = c.benchmark_group("output");
    configure(&mut g);
    g.bench_function("to_sexpr", |b| {
        b.iter(|| tree::to_sexpr(LojbanParser::parse(Rule::text, black_box(input)).unwrap()))
    });
    g.bench_function("to_json", |b| {
        b.iter(|| tree::to_json(LojbanParser::parse(Rule::text, black_box(input)).unwrap()))
    });
    g.finish();
}

criterion_group!(
    benches,
    bench_parse,
    bench_morphology,
    bench_output,
    bench_battery
);
criterion_main!(benches);
