//! docs/coverage.md が lojban.pest と同期しているかを検証する。
//!
//! 文法から全 `*_core` クラスの語彙と統語接続状況を再生成し、
//! ドキュメントの表と突き合わせる。文法を変更したら
//! `cargo test --test coverage_doc -- --nocapture` の出力で更新する。

use std::fs;

fn extract(src: &str) -> Vec<(String, Vec<String>, bool)> {
    let mut cores: Vec<(String, Vec<String>)> = Vec::new();
    let mut idx = 0usize;
    while let Some(rel) = src[idx..].find("_core") {
        let at = idx + rel;
        let line_start = src[..at].rfind('\n').map(|p| p + 1).unwrap_or(0);
        let name = src[line_start..at].trim().to_string();
        let valid_name =
            !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        let after = src[at + 5..].trim_start();
        if !valid_name || !after.starts_with("= @{") {
            idx = at + 5;
            continue;
        }
        // 対応する閉じ(本体はネストした {} を含まない想定)
        let close = match src[at..].find("~ &word_boundary }") {
            Some(p) => at + p,
            None => break,
        };
        let body = &src[at..close];
        let mut words = Vec::new();
        let mut rest = body;
        while let Some(pos) = rest.find("^\"") {
            let after_w = &rest[pos + 2..];
            let end = after_w.find('"').unwrap_or(0);
            if end == 0 {
                break;
            }
            words.push(after_w[..end].to_string());
            rest = &after_w[end..];
        }
        cores.push((name, words));
        idx = close;
    }
    // 統語接続: *_clause への参照数 - 定義行そのもの
    cores
        .into_iter()
        .map(|(name, words)| {
            let clause = format!("{name}_clause");
            let refs = src.matches(&clause).count();
            let self_def = format!("{clause} = {{ {name}_core }}");
            let wired = refs > src.matches(&self_def).count();
            (name, words, wired)
        })
        .collect()
}

#[test]
fn coverage_doc_は文法と同期している() {
    let pest_src = fs::read_to_string("src/grammar/lojban.pest").unwrap();
    let doc = fs::read_to_string("docs/coverage.md").unwrap();

    let classes = extract(&pest_src);
    assert!(!classes.is_empty(), "クラス抽出に失敗");

    for (base, words, wired) in &classes {
        // 各クラス行がドキュメントに存在すること
        let first_word = words.first().map(|w| format!("`{w}`")).unwrap_or_default();
        assert!(
            doc.contains(&format!("| {base} |")) && doc.contains(&first_word),
            "docs/coverage.md に {base} の行が古い/存在しない。再生成してください"
        );
        let mark = if *wired { "✅" } else { "—" };
        let row_prefix = format!("| {base} |");
        let row = doc
            .lines()
            .find(|l| l.starts_with(&row_prefix))
            .expect("row exists");
        assert!(
            row.contains(mark),
            "docs/coverage.md の {base} 行の接続マークが不正(期待: {mark}): {row}"
        );
    }

    // 計数行も整合
    let defined = classes.len();
    let wired_n = classes.iter().filter(|(_, _, w)| *w).count();
    let count_line = format!("計 {defined} クラス定義 / {wired_n} クラスが統語に接続。");
    assert!(
        doc.contains(&count_line),
        "docs/coverage.md の計数行が不整合(期待: {count_line})"
    );
    println!("coverage.md OK: {defined} classes / {wired_n} wired");
}
