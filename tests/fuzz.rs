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

#[test]
fn probe_slow_inputs() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut times: Vec<(u128, String)> = Vec::new();
    for _ in 0..400 {
        let len = rng.below(60);
        let s: String = (0..len)
            .map(|_| ALPHABET[rng.below(ALPHABET.len())] as char)
            .collect();
        if max_nest_of(&s) > 3 {
            continue;
        }
        let t0 = std::time::Instant::now();
        let _ = parse(&s);
        let ms = t0.elapsed().as_millis();
        if ms > 20 {
            times.push((ms, s));
        }
    }
    times.sort_by(|a, b| b.0.cmp(&a.0));
    for (ms, s) in times.iter().take(8) {
        println!("{ms}ms: {s:?}");
    }
}
