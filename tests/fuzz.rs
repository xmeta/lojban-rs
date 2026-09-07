//! 依存なしの簡易ファジングテスト
//!
//! ランダム文字列・コーパス文の変異・深い入れ子について、パーサーが
//! パニックせず有限時間で応答することを確認する(成否は不問)。
//! 外部クレートを使わないため `cargo test` だけで実行できる。

use lojban::parse;

/// xorshift64 による決定論的乱数
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// ロジバン字母 + アポストロフィ + ポーズ記号
const ALPHABET: &[u8] = b"abcdefgijklmnoprstuvxyzh'.,!?";

/// 入力の概算入れ子深度(lu / lohu / vei の最大同時ネスト)。
/// 深い入れ子は MAX_NEST 以下でも拒否解析に時間がかかるため、
/// ランダム系ファザーでは深いものをスキップする(挙動は専用テストで担保)
fn max_nest_of(s: &str) -> i32 {
    let (mut lu, mut lohu, mut vei) = (0i32, 0i32, 0i32);
    let mut max = 0i32;
    for tok in s.split_ascii_whitespace() {
        match tok
            .trim_start_matches(['.', ',', '!', '?'])
            .to_ascii_lowercase()
            .as_str()
        {
            "lu" => lu += 1,
            "li'u" | "lihu" => lu -= 1,
            "lo'u" | "lohu" => lohu += 1,
            "le'u" | "lehu" => lohu -= 1,
            "vei" => vei += 1,
            "ve'o" | "veho" => vei -= 1,
            _ => {}
        }
        max = max.max(lu).max(lohu).max(vei);
    }
    max
}

#[test]
fn ランダム文字列でパニックしない() {
    // 常時実行するスモーク版。ジャンク語の形態論拒否は 1 語あたり
    // 100ms 級かかるため件数を絞っている。重量版は ignore テストを参照
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..60 {
        let len = rng.below(60);
        let s: String = (0..len)
            .map(|_| ALPHABET[rng.below(ALPHABET.len())] as char)
            .collect();
        if max_nest_of(&s) <= 3 {
            let _ = parse(&s);
        }
    }
}

#[test]
fn コーパス文の変異でパニックしない() {
    // 常時実行する軽量版
    const SEEDS: &[&str] = &[
        "mi klama do",
        "le gerku poi cadzu ku'o cu batci le mlatu",
        "lu mi klama li'u cu melbi",
        "zoi .ky. hello world .ky.",
        "li re su'i re du li vo",
        "ge mi klama gi do cadzu",
        "mi tavla do bau la lojban.",
    ];
    for seed in SEEDS {
        let mut rng = Rng(seed.len() as u64 ^ 0xDEAD_BEEF);
        for _ in 0..15 {
            let mut s = seed.to_string();
            match rng.below(3) {
                0 => {
                    // 挿入
                    let pos = rng.below(s.len() + 1);
                    let c = ALPHABET[rng.below(ALPHABET.len())] as char;
                    s.insert(pos.min(s.len()), c);
                }
                1 => {
                    // 削除
                    if !s.is_empty() {
                        let pos = rng.below(s.len());
                        s.remove(pos);
                    }
                }
                _ => {
                    // 部分複製
                    if !s.is_empty() {
                        let a = rng.below(s.len());
                        let b2 = (a + 1 + rng.below(6)).min(s.len());
                        let frag = s[a..b2].to_string();
                        let pos = rng.below(s.len() + 1);
                        s.insert_str(pos.min(s.len()), &frag);
                    }
                }
            }
            if max_nest_of(&s) <= 3 {
                let _ = parse(&s);
            }
        }
    }
}

/// 重量版(実行: `cargo test -- --ignored`)。
/// ランダム入力が深さ上限以下の入れ子拒否パスに当たると 1 件あたり
/// 秒単位かかるため、通常のテスト実行からは除外している。
#[test]
#[ignore = "重量級: 明示的に cargo test -- --ignored で実行する"]
fn ランダム文字列でパニックしない_拡張() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ 0xABCD);
    for _ in 0..3000 {
        let len = rng.below(60);
        let s: String = (0..len)
            .map(|_| ALPHABET[rng.below(ALPHABET.len())] as char)
            .collect();
        let _ = parse(&s);
    }
}

/// 重量版(同上)
#[test]
#[ignore = "重量級: 明示的に cargo test -- --ignored で実行する"]
fn コーパス文の変異でパニックしない_拡張() {
    const SEEDS: &[&str] = &[
        "mi klama do",
        "le gerku poi cadzu ku'o cu batci le mlatu",
        "lu mi klama li'u cu melbi",
        "zoi .ky. hello world .ky.",
        "li re su'i re du li vo",
        "ge mi klama gi do cadzu",
        "mi tavla do bau la lojban.",
    ];
    for seed in SEEDS {
        let mut rng = Rng(seed.len() as u64 ^ 0xCAFE);
        for _ in 0..800 {
            let mut s = seed.to_string();
            match rng.below(3) {
                0 => {
                    let pos = rng.below(s.len() + 1);
                    let c = ALPHABET[rng.below(ALPHABET.len())] as char;
                    s.insert(pos.min(s.len()), c);
                }
                1 => {
                    if !s.is_empty() {
                        let pos = rng.below(s.len());
                        s.remove(pos);
                    }
                }
                _ => {
                    if !s.is_empty() {
                        let a = rng.below(s.len());
                        let b2 = (a + 1 + rng.below(6)).min(s.len());
                        let frag = s[a..b2].to_string();
                        let pos = rng.below(s.len() + 1);
                        s.insert_str(pos.min(s.len()), &frag);
                    }
                }
            }
            let _ = parse(&s);
        }
    }
}

#[test]
fn 入れ子の各深度で有限時間で応答する() {
    // 深さ上限(MAX_NEST)により、深い入れ子は高速に拒否される。
    // 上限以下の深さでは文法がフルに動くため時間がかかる場合があるが、
    // 有限時間で完了すること自体がこのテストの目的
    for d in 1..=30 {
        let s = format!("{}{}", "lu ".repeat(d), "li'u ".repeat(d));
        let _ = parse(&s);
        let v = format!("li {}pa{}", "vei ".repeat(d), " ve'o".repeat(d));
        let _ = parse(&v);
    }
}

/// lu ネスト最悪深度(MAX_NEST=8)の境界性能。
/// 実測(v0.113 調査): lu ネストは深度+1 で約 3 倍の指数成長
/// (vei/gek/mex は平坦)。MAX_NEST ガードにより最悪ケースは深度 8 で
/// 飽和するため、深度 8 の解析がハングせず時間上限内で完了することを
/// 確認する(v0.107 の terms 二重解析による指数化退行の再発検出を兼ねる)。
/// debug ビルドは release の約 13 倍遅いため上限は debug を考慮して寛容に
#[test]
fn luネストの最悪深度は時間上限内で完了する() {
    let s = format!("{}{}", "lu ".repeat(8), "li'u ".repeat(8));
    let t0 = std::time::Instant::now();
    let _ = parse(&s);
    let dt = t0.elapsed();
    // 実測: release 約 2.5 秒 / debug 約 32 秒(約 13 倍)。
    // 上限はその数倍の余裕。ハングや指数化退行(release で 10 倍級)は
    // この上限を容易に超える
    let limit_secs = if cfg!(debug_assertions) { 120 } else { 20 };
    assert!(
        dt.as_secs() < limit_secs,
        "lu ネスト最悪深度の解析が遅すぎる(上限 {limit_secs} 秒): {dt:?}"
    );
}

// -------------------------------------------------------------------------
// 語長上限(v0.113: MAX_TOKEN_CHARS = 50 のリソース保護)
//
// 背景(実測): pest は packrat 型でないため kaprenmit 型の CVC 連鎖トークン
// で rafsi 分割のバックトラックが指数時間になり(9文字あたり約2.1倍)、
// さらに長いトークンでは規則の相互再帰がスタックオーバーフローして
// プロセスが異常終了していた(参照実装 z0 も JS スタック上限で約1万字超は
// RangeError 破綻のため、受理上限は文法上の保証ではなく環境依存)。
// -------------------------------------------------------------------------

/// kaprenmit 型(CVC 連鎖・クラスタあり)の最悪トークンを生成する
fn worst_chain(chars: usize) -> String {
    let base = "kaprenmit";
    let reps = chars / base.len();
    let mut s = base.repeat(reps);
    s.push_str(&base[..chars - s.len()]);
    s
}

/// 上限ちょうどの最悪トークンは受理され、所要時間が実用範囲に収まる
#[test]
fn 語長上限ちょうどの最悪トークンは時間内に受理される() {
    let s = worst_chain(50);
    let t0 = std::time::Instant::now();
    let _pairs = parse(&s).unwrap_or_else(|e| panic!("上限ちょうどの語でエラー: {e}"));
    let dt = t0.elapsed();
    // 受理可否は文法の問題(50文字の kaprenmit 連鎖は cmevla として受理実測)。
    // このテストの主眼は panic・ハングなしで完了すること
    // (release 実測 約1.5秒だが cargo test は debug ビルドで十数倍遅いため
    // 上限は寛容に。ハング(指数爆発)はこの上限を容易に超える)
    assert!(
        parse(&s).is_ok(),
        "上限ちょうどの語は受理されるべき(cmevla)"
    );
    assert!(dt.as_secs() < 60, "語長上限の解析が遅すぎる: {dt:?}");
}

/// 上限 +1 文字は panic せずクリーンなエラーになる
#[test]
fn 語長上限超過はクリーンエラー() {
    let s = worst_chain(51);
    let err = parse(&s).expect_err("上限超過の語が受理された");
    let msg = err.to_string();
    assert!(msg.contains("語が長すぎます"), "{msg}");
    assert!(msg.contains("50"), "{msg}");
    // カスタムメッセージ部はトークン全体でなく先頭12文字+…の truncate 形。
    // 注: pest Error の Display はキャレット図に入力行全体を表示するため、
    // メッセージ全体の文字数上限では断言できない(切断はメッセージ部の役割)
    assert!(msg.contains("「kaprenmitkap…」"), "{msg}");

    // スタックオーバーフローしていた規模(10万字)でも即座にエラー
    let huge = worst_chain(100_000);
    let t0 = std::time::Instant::now();
    let err = parse(&huge).expect_err("10万字トークンが受理された");
    assert!(err.to_string().contains("語が長すぎます"));
    assert!(t0.elapsed().as_secs() < 2, "長大トークンの拒否が遅い");
}

/// classify_word / lujvo 分解も長大入力でパニックしない
/// (旧実装は --classify・--split-lujvo で10万字入力時に SIGABRT していた)
#[test]
fn 長大トークンの分類と分解でパニックしない() {
    use lojban::classify_word;
    use lojban::lujvo;

    let huge = worst_chain(100_000);
    // 旧実装: fatal runtime error: stack overflow, aborting
    assert_eq!(classify_word(&huge), "unknown");
    assert!(lujvo::decompose(&huge).is_err());

    // 境界: 上限ちょうどは従来どおり分類を試み、超過は unknown
    assert_ne!(classify_word(&worst_chain(50)), "");
    assert_eq!(classify_word(&worst_chain(51)), "unknown");
}

/// 語長ガードは ZOI 正規化・SI 消去の後に適用される
/// (zoi 本文の長い語は zo'e に置換済み。「長い語 si」は消去後に残らない)
#[test]
fn 語長ガードは前処理後に適用される() {
    // zoi 本文に長い非ロジバン語(URL 等)が来ても受理される
    let long_junk = "a".repeat(300);
    assert!(parse(&format!("mi cusku zoi gy. {long_junk} .gy")).is_ok());
    // 消去された長い語は解析対象にならないため受理
    let long_tok = worst_chain(60);
    assert!(parse(&format!("mi {long_tok} si cadzu")).is_ok());
    // 消去されずに残る長い語は拒否
    assert!(parse(&format!("mi {long_tok} cadzu")).is_err());
}

/// 線形時間の語形(cmavo 連鎖・y 係音)は上限以内であれば従来どおり受理
#[test]
fn 上限以内の長いcmavo連鎖は受理される() {
    // UI 連鎖(コーパス実在最長は 29 文字)
    assert!(parse("oicairo'aro'ero'iro'oro'ure'e").is_ok());
    // y 係音の連鎖(ためらい音。Y_core は線形時間)
    assert!(parse(&"y".repeat(48)).is_ok());
}
