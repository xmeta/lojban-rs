//! 統語レイヤーの統合テスト
//!
//! 基本文型が解析木として正しく構築されるかを検証する。

use lojban::grammar::{LojbanParser, Rule};
use lojban::tree::to_sexpr;
use pest::Parser;

fn parse_ok(input: &str) -> String {
    let pairs = LojbanParser::parse(Rule::text, input)
        .unwrap_or_else(|e| panic!("解析失敗: {input:?}: {e}"));
    to_sexpr(pairs, input)
}

#[test]
fn 基本文_主語述語目的語() {
    let s = parse_ok("mi tavla do");
    assert!(s.contains("sentence"), "{s}");
    assert!(s.contains("KOhA_core \"mi\""), "{s}");
    assert!(s.contains("selbri"), "{s}");
    assert!(s.contains("BRIVLA_core \"tavla\""), "{s}");
    assert!(s.contains("KOhA_core \"do\""), "{s}");
}

#[test]
fn 冠詞句と_cu() {
    let s = parse_ok("le mlatu cu cadzu");
    assert!(s.contains("LE_core \"le\""), "{s}");
    assert!(s.contains("BRIVLA_core \"mlatu\""), "{s}");
    assert!(s.contains("CU_core \"cu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"cadzu\""), "{s}");
}

#[test]
fn tanru_名詞句修飾() {
    let s = parse_ok("mi viska lo cnino zdani");
    assert!(s.contains("tanru"), "{s}");
    assert!(s.contains("BRIVLA_core \"cnino\""), "{s}");
    assert!(s.contains("BRIVLA_core \"zdani\""), "{s}");
}

#[test]
fn 固有名詞() {
    let s = parse_ok("la alis. cu tavla la bob.");
    assert_eq!(s.matches("CMEVLA_clause").count(), 2, "{s}");
}

#[test]
fn 否定_na() {
    let s = parse_ok("mi na prami");
    assert!(s.contains("NA_core \"na\""), "{s}");
}

#[test]
fn 変換_se() {
    let s = parse_ok("mi se prami do");
    assert!(s.contains("SE_core \"se\""), "{s}");
}

#[test]
fn 疑問_xu_は自由修飾語() {
    let s = parse_ok("xu do djica");
    assert!(s.contains("UI_core \"xu\""), "{s}");
    assert!(s.contains("free"), "{s}");
}

#[test]
fn 感情標識_ui() {
    let s = parse_ok("mi gleki ui");
    assert!(s.contains("UI_core \"ui\""), "{s}");
}

#[test]
fn 量化描述と抽象() {
    let s = parse_ok("ro lo remna cu kakne lo ka limna");
    assert!(s.contains("PA_core \"ro\""), "{s}");
    assert!(s.contains("NU_core \"ka\""), "{s}");
    assert!(s.contains("abstraction") || s.contains("nu_form"), "{s}");
}

#[test]
fn 文の連結_i() {
    let s = parse_ok("mi klama .i do stali");
    assert!(s.matches("sentence").count() >= 2, "{s}");
    assert!(s.contains("I_core \"i\""), "{s}");
}

#[test]
fn nu_抽象を含む描述() {
    let s = parse_ok("mi djica lo nu do klama");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
}

#[test]
fn 数量詞付き描述() {
    let s = parse_ok("mi viska re lo mlatu");
    assert!(s.contains("PA_core \"re\""), "{s}");
}

#[test]
fn 呼格() {
    let s = parse_ok("coi la alis.");
    assert!(s.contains("COI_core \"coi\""), "{s}");
    assert!(s.contains("vocative"), "{s}");
}

#[test]
fn 関係節_poi() {
    let s = parse_ok("le gerku poi cadzu cu batci");
    assert!(s.contains("NOI_core \"poi\""), "{s}");
    assert!(s.contains("relative_clause"), "{s}");
    assert!(s.contains("BRIVLA_core \"batci\""), "{s}");
}

#[test]
fn 項フラグメントは受理される() {
    // 述語を伴わない項のみの発話は fragment として正当
    assert!(LojbanParser::parse(Rule::text, "mi").is_ok());
    assert!(LojbanParser::parse(Rule::text, "zu'i").is_ok());
}

#[test]
fn 不完全な描述は拒否() {
    // 空の描述
    assert!(LojbanParser::parse(Rule::text, "le cu").is_err());
}

#[test]
fn 時制詞を含む文() {
    let s = parse_ok("mi pu klama do");
    assert!(s.contains("PU_core \"pu\""), "{s}");
    let s = parse_ok("do ba'o cadzu");
    assert!(s.contains("ZAhO_core \"ba'o\""), "{s}");
    let s = parse_ok("ta'e do simsa le mlatu");
    assert!(s.contains("TAhE_core \"ta'e\""), "{s}");
}

#[test]
fn be_による項連結() {
    let s = parse_ok("mi klama be le zdani");
    assert!(s.contains("linked_args"), "{s}");
    assert!(s.contains("BE_core \"be\""), "{s}");
    assert!(s.contains("LE_core \"le\""), "{s}");
    assert!(s.contains("BRIVLA_core \"zdani\""), "{s}");
}

#[test]
fn bei_による連結項の列挙() {
    let s = parse_ok("mi klama be le zdani bei le zarci");
    assert!(s.contains("linked_args"), "{s}");
    assert!(s.contains("BEI_core \"bei\""), "{s}");
    assert!(s.contains("BRIVLA_core \"zdani\""), "{s}");
    assert!(s.contains("BRIVLA_core \"zarci\""), "{s}");
}

#[test]
fn beo_明示閉鎖() {
    // be'o で連結項を閉じ、後続の項と区切る
    let s = parse_ok("mi klama be le zdani be'o do");
    assert!(s.contains("linked_args"), "{s}");
    assert!(s.contains("BEhO_core \"be'o\""), "{s}");
    assert!(s.contains("KOhA_core \"do\""), "{s}");
}

#[test]
fn zo_による単語引用() {
    let s = parse_ok("mi cusku zo coi");
    assert!(s.contains("ZO_core \"zo\""), "{s}");
    assert!(s.contains("CMAVO_core \"coi\""), "{s}");
}

#[test]
fn 引用は項として機能する() {
    let s = parse_ok("zo coi cu cmavo");
    assert!(s.contains("zo_quote"), "{s}");
}

#[test]
fn lu_による文引用() {
    let s = parse_ok("lu mi klama lihu");
    assert!(s.contains("LU_core \"lu\""), "{s}");
    assert!(s.contains("LUhU_core"), "{s}");
}

#[test]
fn lohu_による誤文引用は_free() {
    let s = parse_ok("mi gleki lohu coi lehu");
    assert!(s.contains("LOhU_core"), "{s}");
    assert!(s.contains("lohu_quote"), "{s}");
}

#[test]
fn 引用の標準表記_lihu_lehu() {
    let s = parse_ok("lu mi klama li'u");
    assert!(s.contains("LUhU_core \"li'u\""), "{s}");
    let s = parse_ok("mi gleki lo'u coi le'u");
    assert!(s.contains("LOhU_core \"lo'u\""), "{s}");
    assert!(s.contains("LEhU_core \"le'u\""), "{s}");
}

#[test]
fn 文連結_niho_の標準表記() {
    let s = parse_ok("ni'o mi klama .i do stali");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
}

#[test]
fn 不完全な引用や項連結は拒否される() {
    // 未閉鎖の LU 引用
    assert!(LojbanParser::parse(Rule::text, "lu mi klama").is_err());
    // 未閉鎖の LOhU 引用
    assert!(LojbanParser::parse(Rule::text, "mi gleki lohu coi").is_err());
    // 引用対象のない zo
    assert!(LojbanParser::parse(Rule::text, "zo").is_err());
    // 項のない be
    assert!(LojbanParser::parse(Rule::text, "mi klama be").is_err());
}

#[test]
fn 終端詞の標準アポストロフィ表記() {
    // 相対節の閉鎖 ku'o
    let s = parse_ok("le gerku poi cadzu ku'o cu batci");
    assert!(s.contains("KUhO_core \"ku'o\""), "{s}");
    // 所有の閉鎖 ge'u
    let s = parse_ok("le gerku pe mi ge'u cu batci");
    assert!(s.contains("GEhU_core \"ge'u\""), "{s}");
    // ke グルーピングの閉鎖 ke'e
    let s = parse_ok("mi viska ke cmalu ke'e nixli");
    assert!(s.contains("KEhE_core \"ke'e\""), "{s}");
}

#[test]
fn 挿入と呼格の標準閉鎖() {
    // sei の閉鎖 se'u
    let s = parse_ok("sei mi gleki se'u mi klama");
    assert!(s.contains("SEhU_core \"se'u\""), "{s}");
    // 呼格の閉鎖 do'u
    let s = parse_ok("coi la alis. do'u mi klama");
    assert!(s.contains("DOhU_core \"do'u\""), "{s}");
}
