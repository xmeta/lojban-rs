//! 形態論レイヤーの検証テスト
//!
//! zantufa-1.9999.peg 由来の語形認識が正しく動くかを確認する。

use lojban::grammar::{LojbanParser, Rule};
use pest::Parser;

fn accepts(rule: Rule, input: &str) -> bool {
    LojbanParser::parse(rule, input).is_ok()
}

#[test]
fn koha_代名詞等() {
    for w in ["mi", "do", "ti", "ta", "tu", "ri", "ra", "ru", "ko", "zo'e", "ma", "da", "de", "di"] {
        assert!(accepts(Rule::KOhA_clause, w), "KOhA should accept {w:?}");
    }
    assert!(!accepts(Rule::KOhA_clause, "le"));
}

#[test]
fn le_冠詞類() {
    for w in ["le", "lo", "la", "lei", "loi", "lai", "le'i", "lo'i"] {
        assert!(accepts(Rule::LE_clause, w), "LE should accept {w:?}");
    }
    assert!(!accepts(Rule::LE_clause, "mi"));
}

#[test]
fn 各種クラス() {
    assert!(accepts(Rule::SE_clause, "se"));
    assert!(accepts(Rule::SE_clause, "xe"));
    assert!(accepts(Rule::NU_clause, "nu"));
    assert!(accepts(Rule::NU_clause, "ka"));
    assert!(accepts(Rule::NU_clause, "zu'o"));
    assert!(accepts(Rule::PA_clause, "pa"));
    assert!(accepts(Rule::PA_clause, "rei"));
    assert!(accepts(Rule::PA_clause, "ro"));
    assert!(accepts(Rule::PA_clause, "su'o"));
    assert!(accepts(Rule::UI_clause, "ui"));
    assert!(accepts(Rule::UI_clause, "xu"));
    assert!(accepts(Rule::UI_clause, "u'i"));
    assert!(accepts(Rule::NA_clause, "na"));
    assert!(accepts(Rule::NAhE_clause, "na'e"));
    assert!(accepts(Rule::CU_clause, "cu"));
    assert!(accepts(Rule::I_clause, "i"));
    assert!(accepts(Rule::NIhO_clause, "niho"));
}

#[test]
fn gismu_基本形() {
    for w in [
        "gerku", "blanu", "claxu", "bakni", "slabu", "cipni", "kerfa", "mlatu",
        "prenu", "tavla", "viska", "zdani", "cadzu", "prami", "klama", "djica",
        "remna", "kakne", "limna", "melbi",
    ] {
        assert!(accepts(Rule::BRIVLA_clause, w), "BRIVLA should accept {w:?}");
    }
}

#[test]
fn lujvo_合成語() {
    // gerzda = gerku + zdani の lujvo 形
    assert!(accepts(Rule::BRIVLA_clause, "gerzda"));
    // brivla 自身(bridi + valsi)
    assert!(accepts(Rule::BRIVLA_clause, "brivla"));
}

#[test]
fn ストレス付き大文字表記() {
    assert!(accepts(Rule::BRIVLA_clause, "GERku"));
    assert!(accepts(Rule::BRIVLA_clause, "gerku"));
}

#[test]
fn cmevla_固有名詞() {
    assert!(accepts(Rule::CMEVLA_clause, "alis."));
    assert!(accepts(Rule::CMEVLA_clause, "bob."));
}

#[test]
fn 不正な入力は拒否() {
    // q/w はロジバン字母に存在しない
    assert!(!accepts(Rule::text, "q"));
    // 母音のみの語(aaaa / oai)は語形として不正
    assert!(!accepts(Rule::text, "aaaa"));
    assert!(!accepts(Rule::text, "oai"));
}

#[test]
fn 母音のみの語は_zantufa準拠で受理される() {
    // 注意: zantufa 原典では brivla_head が空音節を許すため
    // "iii" のような母音のみの語も fu'ivla として受理される(原典準拠の挙動)。
    assert!(accepts(Rule::BRIVLA_clause, "iii"));
}

#[test]
fn 文中の語クラス認識() {
    use lojban::tree::to_sexpr;
    let input = "mi viska le gerku";
    let pairs = LojbanParser::parse(Rule::text, input).unwrap();
    let s = to_sexpr(pairs, input);
    assert!(s.contains("KOhA_clause"), "{s}");
    assert!(s.contains("BRIVLA_core \"viska\""), "{s}");
    assert!(s.contains("LE_clause"), "{s}");
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
}

#[test]
fn cmevla_前のポーズ() {
    // 固有名詞は直前のポーズ(.)とともに現れる
    let input = "mi tavla la alis.";
    assert!(LojbanParser::parse(Rule::text, input).is_ok());
}
