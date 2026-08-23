//! lujvo(新語)生成アルゴリズム(CLL 第4章準拠)
//!
//! 入力は rafsi(結合形)の列。ハイフン挿入規則([CLL 4.11](
//! https://lojban.github.io/cll/4/11/))とスコアリング([CLL 4.12](
//! https://lojban.github.io/cll/4/12/))を実装する。
//! rafsi の割り当て(どの gismu がどの CVC 形を持つか)は辞書データであり
//! 本モジュールの対象外。呼び出し側が候補形を与える。

use std::fmt;

/// rafsi の形態分類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// CVC(例: nun)- 非末尾・末尾両用
    Cvc,
    /// CCV(例: zba)- 非末尾・末尾両用
    Ccv,
    /// CVV アポストロフィ付き(例: ta'u)
    CvvApo,
    /// CVV アポストロフィなし(例: sai)
    Cvv,
    /// 4文字 rafsi(gismu から末尾母音を除いた CVCC / CCVC、例: zbas)- 非末尾のみ
    Long4,
    /// 5文字の末尾形 CVCCV(例: sarji)- 末尾のみ。スコア値 1
    FinalCvccv,
    /// 5文字の末尾形 CCVCV(例: zbasu)- 末尾のみ。スコア値 3
    FinalCcvcv,
}

impl Form {
    /// CLL 4.12 のスコア表の値(R の算出用)。
    /// 表は CCVC=4 / CCVCV=3 のみ掲載のため、CVCC 系を 4、CVCCV 系を 3 として扱う
    fn score_value(self) -> i64 {
        match self {
            Form::Cvc => 5,
            Form::Ccv => 7,
            Form::CvvApo => 6,
            Form::Cvv => 8,
            Form::Long4 => 4,
            // CLL 表の「CVC/CV (final) (-sarji) 1」「CCVCV (final) (-zbasu) 3」
            Form::FinalCvccv => 1,
            Form::FinalCcvcv => 3,
        }
    }
}

/// rafsi 文字列のエラー
#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    /// 語形として分類できない
    Unclassifiable(String),
    /// 位置に適さない形(例: 4文字 rafsi が末尾)
    Misplaced { rafsi: String, reason: &'static str },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::Unclassifiable(s) => write!(f, "rafsi として解釈できない語形: {s:?}"),
            BuildError::Misplaced { rafsi, reason } => {
                write!(f, "rafsi {rafsi:?}: {reason}")
            }
        }
    }
}

/// [`build`] の結果。生成語と、スコア計算に必要な情報を保持する
#[derive(Debug, Clone)]
pub struct Built {
    /// 生成された lujvo
    pub word: String,
    /// 入力 rafsi の分類
    pub forms: Vec<Form>,
    /// 挿入したハイフン(y / r / n)の数
    pub hyphens: i64,
}

impl Built {
    /// lujvo のスコアを計算する(CLL 4.12)。低いほど辞書形として好ましい。
    ///
    /// score = 1000·L − 500·A + 100·H − 10·R − V
    /// (L=文字数、A=アポストロフィ数、H=ハイフン数、R=rafsi 種別値の和、
    ///  V=y を除く母音数)
    pub fn score(&self) -> i64 {
        let l = self.word.chars().count() as i64;
        let a = self.word.chars().filter(|&c| c == '\'').count() as i64;
        let h = self.hyphens;
        let r: i64 = self.forms.iter().map(|f| f.score_value()).sum();
        let v = self
            .word
            .chars()
            .filter(|&c| c != 'y' && is_v(c as u8))
            .count() as i64;
        1000 * l - 500 * a + 100 * h - 10 * r - v
    }
}

fn is_c(c: u8) -> bool {
    c.is_ascii_alphabetic() && !b"aeiou".contains(&c.to_ascii_lowercase())
}
fn is_v(c: u8) -> bool {
    b"aeiou".contains(&c.to_ascii_lowercase())
}

/// rafsi 文字列を分類する
pub fn classify(s: &str) -> Result<Form, BuildError> {
    let b = s.as_bytes();
    let pat: Vec<u8> = b
        .iter()
        .map(|&c| {
            if c == b'\'' {
                b'\''
            } else if is_v(c) {
                b'V'
            } else if is_c(c) {
                b'C'
            } else {
                b'?'
            }
        })
        .collect();
    if pat.contains(&b'?') || s != s.to_ascii_lowercase() {
        return Err(BuildError::Unclassifiable(s.into()));
    }
    Ok(match pat.as_slice() {
        b"CVC" => Form::Cvc,
        b"CCV" => Form::Ccv,
        b"CVV" => Form::Cvv,
        b"CV'V" => Form::CvvApo,
        // 4文字形は gismu から末尾母音を除いた CVCC / CCVC
        b"CVCC" | b"CCVC" => Form::Long4,
        b"CVCCV" => Form::FinalCvccv,
        b"CCVCV" => Form::FinalCcvcv,
        _ => return Err(BuildError::Unclassifiable(s.into())),
    })
}

/// 許容される初期子音ペア(48 種、CLL 3.7)。tosmabru 検査で使用
fn is_initial_pair(a: u8, b: u8) -> bool {
    let p = [a.to_ascii_lowercase(), b.to_ascii_lowercase()];
    matches!(
        &p,
        b"bl"
            | b"br"
            | b"cf"
            | b"ck"
            | b"cl"
            | b"cm"
            | b"cn"
            | b"cp"
            | b"cr"
            | b"ct"
            | b"dj"
            | b"dr"
            | b"dz"
            | b"fl"
            | b"fr"
            | b"gl"
            | b"gr"
            | b"jb"
            | b"jd"
            | b"jg"
            | b"jm"
            | b"jv"
            | b"kl"
            | b"kr"
            | b"ml"
            | b"mr"
            | b"pl"
            | b"pr"
            | b"sf"
            | b"sk"
            | b"sl"
            | b"sm"
            | b"sn"
            | b"sp"
            | b"st"
            | b"tc"
            | b"tr"
            | b"ts"
            | b"vl"
            | b"vr"
            | b"xk"
            | b"xl"
            | b"xr"
            | b"zb"
            | b"zd"
            | b"zg"
            | b"zl"
            | b"zv"
    )
}

/// 語中の子音ペアの許容性(CLL 3.6)。
///
/// 1) 同一子音の連続は不可
/// 2) 有声と無声の混在は不可(l m n r は中立で例外)
/// 3) c j s z のうち2つの組合せは不可
/// 4) cx kx xc xk mz は不可
fn is_permissible_medial(a: u8, b: u8) -> bool {
    let x = a.to_ascii_lowercase();
    let y = b.to_ascii_lowercase();
    if x == y {
        return false;
    }
    if matches!(
        (x, y),
        (b'c', b'x') | (b'x', b'c') | (b'k', b'x') | (b'x', b'k') | (b'm', b'z')
    ) {
        return false;
    }
    let voiced = |c: u8| matches!(c, b'b' | b'd' | b'g' | b'j' | b'v' | b'z');
    let unvoiced = |c: u8| matches!(c, b'c' | b'f' | b'k' | b'p' | b's' | b't' | b'x');
    let neutral = |c: u8| matches!(c, b'l' | b'm' | b'n' | b'r');
    if !neutral(x) && !neutral(y) && voiced(x) != voiced(y) && (voiced(x) || unvoiced(x)) {
        return false;
    }
    let sibilant = |c: u8| matches!(c, b'c' | b'j' | b's' | b'z');
    if sibilant(x) && sibilant(y) {
        return false;
    }
    true
}

/// rafsi 列から lujvo を生成する(CLL 4.11)。
///
/// ハイフン規則は右から左へ適用することを推奨するとの記載に従い、
/// 境界を右側から処理する。
pub fn build(rafsi: &[&str]) -> Result<Built, BuildError> {
    if rafsi.is_empty() {
        return Err(BuildError::Unclassifiable("(empty)".into()));
    }
    let mut forms = Vec::with_capacity(rafsi.len());
    for (i, r) in rafsi.iter().enumerate() {
        let f = classify(r)?;
        let last = i + 1 == rafsi.len();
        match f {
            Form::Long4 if last => {
                return Err(BuildError::Misplaced {
                    rafsi: (*r).into(),
                    reason: "4文字 rafsi は末尾に置けない",
                })
            }
            Form::Cvc if last => {
                return Err(BuildError::Misplaced {
                    rafsi: (*r).into(),
                    reason: "末尾には CVV / CCV / 5文字形が必要(CLL 4.11 手順2)",
                })
            }
            Form::FinalCvccv | Form::FinalCcvcv if !last => {
                return Err(BuildError::Misplaced {
                    rafsi: (*r).into(),
                    reason: "5文字形(gismu)は末尾でのみ使用できる",
                })
            }
            _ => {}
        }
        forms.push((*r, f));
    }

    let n = forms.len();
    // 各境界のハイフン(None | Some('y') | Some('r') | Some('n'))
    let mut hyphens: Vec<Option<char>> = vec![None; n.saturating_sub(1)];

    let hyphen_letter = |next_rafsi: &str| {
        if next_rafsi.starts_with('r') {
            'n'
        } else {
            'r'
        }
    };

    // 4a) r/n ハイフン: CVV 形の後ろ(右から左へ)
    for i in (0..n.saturating_sub(1)).rev() {
        let (_, f) = forms[i];
        if !matches!(f, Form::Cvv | Form::CvvApo) {
            continue;
        }
        let need = if n == 2 {
            // 2語の場合: 次が CCV なら不要
            forms[1].1 != Form::Ccv
        } else {
            // 3語以上の場合: 先頭の CVV の後ろのみ
            i == 0
        };
        if need {
            hyphens[i] = Some(hyphen_letter(forms[i + 1].0));
        }
    }

    // 4c) 4文字 rafsi の後ろに y
    for i in 0..n.saturating_sub(1) {
        if forms[i].1 == Form::Long4 {
            hyphens[i] = Some('y');
        }
    }

    // 組み立て。4b) 不許容の語中子音ペアがある境界には y を挿入し、
    // tosmabru 検査用に「y を挿入していない子音-子音境界」を記録する
    let mut out = String::new();
    let mut hyphen_count: i64 = 0;
    // (out 内での位置, 境界インデックス)
    let mut open_joints: Vec<usize> = Vec::new();
    for (i, (text, _)) in forms.iter().enumerate() {
        out.push_str(text);
        if i + 1 >= n {
            break;
        }
        match hyphens[i] {
            Some(h) => {
                out.push(h);
                hyphen_count += 1;
            }
            None => {
                let prev = out.as_bytes()[out.len() - 1];
                let next = forms[i + 1].0.as_bytes()[0];
                if is_c(prev) && is_c(next) {
                    if !is_permissible_medial(prev, next) {
                        out.push('y');
                        hyphen_count += 1;
                    } else {
                        open_joints.push(out.len()); // この位置が継ぎ目
                    }
                }
            }
        }
    }

    // 5) tosmabru 検査: 先頭が CVC のとき、最初の y ハイフンより前の継ぎ目が
    //    すべて許容初期ペアなら cmavo+brivla に分解されるため、最初の継ぎ目に y
    let ob = out.as_bytes();
    if forms[0].1 == Form::Cvc && n >= 2 {
        let limit = out.find('y').unwrap_or(out.len());
        let joints: Vec<usize> = open_joints.iter().copied().filter(|&p| p < limit).collect();
        let all_initial = !joints.is_empty()
            && joints
                .iter()
                .all(|&p| p >= 1 && is_initial_pair(ob[p - 1], ob[p]));
        if all_initial {
            let p = joints[0];
            out.insert(p, 'y');
            hyphen_count += 1;
        }
    }

    Ok(Built {
        word: out,
        forms: forms.into_iter().map(|(_, f)| f).collect(),
        hyphens: hyphen_count,
    })
}

/// 分解結果の要素
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    /// rafsi 本体
    Rafsi { text: String, form: Form },
    /// 挿入されたハイフン(y / r / n)
    Hyphen(char),
}

/// 分解の失敗
#[derive(Debug, PartialEq, Eq)]
pub struct DecomposeError(pub String);

impl fmt::Display for DecomposeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lujvo として分解できない語形: {:?}", self.0)
    }
}

struct Decomp<'a> {
    b: &'a [u8],
}

impl<'a> Decomp<'a> {
    /// 位置 pos から残りを分解できるか(バックトラック探索)
    fn seg(&self, pos: usize, parts: &mut Vec<Part>, first: bool) -> bool {
        if pos == self.b.len() {
            return !first;
        }
        // y ハイフン
        if self.b[pos] == b'y' {
            parts.push(Part::Hyphen('y'));
            if self.seg(pos + 1, parts, false) {
                return true;
            }
            parts.pop();
            return false;
        }
        let rem = self.b.len() - pos;
        // 末尾の 5文字形(gismu)
        if rem == 5 {
            if let Ok(f) = classify(&self.b[pos..].iter().map(|&c| c as char).collect::<String>()) {
                if matches!(f, Form::FinalCvccv | Form::FinalCcvcv) {
                    parts.push(Part::Rafsi {
                        text: String::from_utf8_lossy(&self.b[pos..]).into_owned(),
                        form: f,
                    });
                    return true;
                }
            }
        }
        // CVV(アポストロフィ付き、4文字)
        if rem >= 4
            && self.b[pos + 1] != b'y'
            && is_c(self.b[pos])
            && is_v(self.b[pos + 1])
            && self.b[pos + 2] == b'\''
            && is_v(self.b[pos + 3])
        {
            if let Some(res) = self.try_cvv(pos, 4, parts) {
                return res;
            }
        }
        // CVV(3文字)
        if rem >= 3 && is_c(self.b[pos]) && is_v(self.b[pos + 1]) && is_v(self.b[pos + 2]) {
            if let Some(res) = self.try_cvv(pos, 3, parts) {
                return res;
            }
        }
        // CCV(初期子音ペアが正当なもののみ)
        if rem >= 3
            && is_c(self.b[pos])
            && is_c(self.b[pos + 1])
            && is_v(self.b[pos + 2])
            && is_initial_pair(self.b[pos], self.b[pos + 1])
        {
            parts.push(Part::Rafsi {
                text: String::from_utf8_lossy(&self.b[pos..pos + 3]).into_owned(),
                form: Form::Ccv,
            });
            if self.seg(pos + 3, parts, false) {
                return true;
            }
            parts.pop();
        }
        // CVC(非末尾。次が子音か y のみ)
        if rem >= 4 && is_c(self.b[pos]) && is_v(self.b[pos + 1]) && is_c(self.b[pos + 2]) {
            parts.push(Part::Rafsi {
                text: String::from_utf8_lossy(&self.b[pos..pos + 3]).into_owned(),
                form: Form::Cvc,
            });
            if self.seg(pos + 3, parts, false) {
                return true;
            }
            parts.pop();
        }
        // 4文字形(CVCC / CCVC)+ y
        if rem >= 5
            && self.b[pos + 4] == b'y'
            && ((is_c(self.b[pos])
                && is_v(self.b[pos + 1])
                && is_c(self.b[pos + 2])
                && is_c(self.b[pos + 3]))
                || (is_c(self.b[pos])
                    && is_c(self.b[pos + 1])
                    && is_v(self.b[pos + 2])
                    && is_c(self.b[pos + 3])))
        {
            parts.push(Part::Rafsi {
                text: String::from_utf8_lossy(&self.b[pos..pos + 4]).into_owned(),
                form: Form::Long4,
            });
            if self.seg(pos + 4, parts, false) {
                return true;
            }
            parts.pop();
        }
        false
    }

    /// CVV 形 rafsi の試行。後続は語末・r/n ハイフン+子音・(2語目以降の例外として)
    /// 母音なし直接 CCV 継接(saicli 型)のいずれかでなければならない
    fn try_cvv(&self, pos: usize, clen: usize, parts: &mut Vec<Part>) -> Option<bool> {
        let after = pos + clen;
        let text = String::from_utf8_lossy(&self.b[pos..after]).into_owned();
        if after == self.b.len() {
            // 語末の CVV は正当な終端形
            parts.push(Part::Rafsi {
                text,
                form: if clen == 4 { Form::CvvApo } else { Form::Cvv },
            });
            let ok = self.seg(after, parts, false);
            if !ok {
                parts.pop();
            }
            return Some(ok);
        }
        let n = self.b[after];
        if (n == b'r' || n == b'n') && after + 1 < self.b.len() && is_c(self.b[after + 1]) {
            parts.push(Part::Rafsi {
                text: text.clone(),
                form: if clen == 4 { Form::CvvApo } else { Form::Cvv },
            });
            parts.push(Part::Hyphen(n as char));
            if self.seg(after + 1, parts, false) {
                return Some(true);
            }
            parts.pop();
            parts.pop();
        }
        // saicli 型: CVV + CCV 直結(ハイフン不要)。次の 3 文字が CCV で
        // その後に続く場合のみ許容
        if is_c(n) && self.b.len() >= after + 3 {
            parts.push(Part::Rafsi {
                text: text.clone(),
                form: if clen == 4 { Form::CvvApo } else { Form::Cvv },
            });
            let ok = self.seg(after, parts, false);
            if !ok {
                parts.pop();
            }
            return Some(ok);
        }
        None
    }
}

/// lujvo を rafsi 列とハイフンに分解する。
///
/// [`build`] の逆操作であり、生成テストで双方向の整合を検証している。
/// 入力は本プロジェクトの文法で brivla として受理される語であること。
pub fn decompose(lujvo: &str) -> Result<Vec<Part>, DecomposeError> {
    let d = Decomp {
        b: lujvo.as_bytes(),
    };
    let mut parts = Vec::new();
    if d.seg(0, &mut parts, true) {
        // gismu 単体は lujvo ではない(構成要素が1つの最終形だけ)なら拒否
        if parts.len() == 1
            && matches!(
                parts[0],
                Part::Rafsi {
                    form: Form::FinalCvccv | Form::FinalCcvcv,
                    ..
                }
            )
        {
            return Err(DecomposeError(lujvo.into()));
        }
        Ok(parts)
    } else {
        Err(DecomposeError(lujvo.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 分類() {
        assert_eq!(classify("nun"), Ok(Form::Cvc));
        assert_eq!(classify("zba"), Ok(Form::Ccv));
        assert_eq!(classify("sai"), Ok(Form::Cvv));
        assert_eq!(classify("ta'u"), Ok(Form::CvvApo));
        assert_eq!(classify("zbas"), Ok(Form::Long4));
        assert_eq!(classify("cmav"), Ok(Form::Long4));
        assert_eq!(classify("sarji"), Ok(Form::FinalCvccv));
        assert_eq!(classify("zbasu"), Ok(Form::FinalCcvcv));
        assert!(classify("qqq").is_err());
    }

    #[test]
    fn cll_公式例_生成とスコア() {
        // CLL 4.12 の例(生成結果とスコア値の両方を検証)
        let b = build(&["zba", "sai"]).unwrap();
        assert_eq!(b.word, "zbasai");
        assert_eq!(b.score(), 5847);

        let b = build(&["nun", "nau"]).unwrap();
        assert_eq!(b.word, "nunynau");
        assert_eq!(b.score(), 6967);

        let b = build(&["sai", "zba", "ta'u"]).unwrap();
        assert_eq!(b.word, "sairzbata'u");
        assert_eq!(b.score(), 10385);

        let b = build(&["zba", "zbas", "sarji"]).unwrap();
        assert_eq!(b.word, "zbazbasysarji");
        assert_eq!(b.score(), 12976);
    }

    #[test]
    fn tosmabru_検査() {
        // tos + mabru: 境界の継ぎ目 sm が許容初期ペアのため cmavo+brivla に
        // 分解されてしまう → 最初の継ぎ目に y を挿入
        let b = build(&["tos", "mabru"]).unwrap();
        assert_eq!(b.word, "tosymabru");
    }

    #[test]
    fn 語中クラスタ規則による_y_挿入() {
        // ger + zda: 境界 rz は語中では合法なので y は不要
        let b = build(&["ger", "zda"]).unwrap();
        assert_eq!(b.word, "gerzda");
        // 境界が不許容(例: sd は有声無声混在)の場合は y が入る
        let b = build(&["kes", "dirgo"]).unwrap();
        assert_eq!(b.word, "kesydirgo");
    }

    #[test]
    fn 生成語がパーサーで受理される() {
        // 自己統合検証: 生成した lujvo が本プロジェクトの文法で brivla として認識されること
        use crate::grammar::{LojbanParser, Rule};
        use pest::Parser;
        for built in [
            build(&["zba", "sai"]).unwrap().word,
            build(&["nun", "nau"]).unwrap().word,
            build(&["sai", "zba", "ta'u"]).unwrap().word,
            build(&["zba", "zbas", "sarji"]).unwrap().word,
            build(&["tos", "mabru"]).unwrap().word,
            build(&["ger", "zda"]).unwrap().word,
            build(&["kes", "dirgo"]).unwrap().word,
        ] {
            assert!(
                LojbanParser::parse(Rule::BRIVLA_clause, &built).is_ok(),
                "生成語 {built} が brivla として受理されない"
            );
        }
    }

    #[test]
    fn 分解_既知例() {
        use Part::*;
        assert_eq!(
            decompose("gerzda").unwrap(),
            vec![
                Rafsi {
                    text: "ger".into(),
                    form: Form::Cvc
                },
                Rafsi {
                    text: "zda".into(),
                    form: Form::Ccv
                },
            ]
        );
        assert_eq!(
            decompose("sairzbata'u").unwrap(),
            vec![
                Rafsi {
                    text: "sai".into(),
                    form: Form::Cvv
                },
                Hyphen('r'),
                Rafsi {
                    text: "zba".into(),
                    form: Form::Ccv
                },
                Rafsi {
                    text: "ta'u".into(),
                    form: Form::CvvApo
                },
            ]
        );
        assert_eq!(
            decompose("tosymabru").unwrap(),
            vec![
                Rafsi {
                    text: "tos".into(),
                    form: Form::Cvc
                },
                Hyphen('y'),
                Rafsi {
                    text: "mabru".into(),
                    form: Form::FinalCvccv
                },
            ]
        );
        assert_eq!(
            decompose("zbazbasysarji").unwrap(),
            vec![
                Rafsi {
                    text: "zba".into(),
                    form: Form::Ccv
                },
                Rafsi {
                    text: "zbas".into(),
                    form: Form::Long4
                },
                Hyphen('y'),
                Rafsi {
                    text: "sarji".into(),
                    form: Form::FinalCvccv
                },
            ]
        );
    }

    #[test]
    fn 分解_roundtrip() {
        // build の逆写像として decompose が機能することを全パターンで検証
        let cases: &[&[&str]] = &[
            &["zba", "sai"],
            &["nun", "nau"],
            &["sai", "zba", "ta'u"],
            &["zba", "zbas", "sarji"],
            &["tos", "mabru"],
            &["ger", "zda"],
            &["kes", "dirgo"],
            &["zan", "ru'e", "gei"],
        ];
        for rafsi in cases {
            let built = build(rafsi).unwrap();
            let parts = decompose(&built.word).unwrap();
            let texts: Vec<&str> = parts
                .iter()
                .filter_map(|p| match p {
                    Part::Rafsi { text, .. } => Some(text.as_str()),
                    _ => None,
                })
                .collect();
            assert_eq!(&texts, rafsi, "word={}", built.word);
        }
    }

    #[test]
    fn 分解_失敗系() {
        // gismu 単体は lujvo ではない
        assert!(decompose("gerku").is_err());
        assert!(decompose("qqqqq").is_err());
    }

    #[test]
    fn エラー系() {
        assert!(build(&[]).is_err());
        assert!(build(&["zzz"]).is_err()); // 分類不能
        assert!(build(&["zbas", "nun"]).is_err()); // 4文字形が末尾
        assert!(build(&["sarji", "nun"]).is_err()); // gismu が非末尾
    }
}
