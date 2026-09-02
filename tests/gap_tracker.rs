//! 未解決 GAP(zantufa 系参照パーサーが受理するのに本パーサーが拒否する形)の
//! 追跡用テスト。
//!
//! このファイルは未解決 GAP の追跡用。各テストは GAP 解消時に緑化される。
//! cargo test 全体は本ファイルの分だけ失敗する状態が正。
//!
//! - 出典: 語彙×統語位置のプローブ行列(1412 プローブ)を本パーサーと
//!   参照パーサー 3 種(zantufa-0.9999.js / zantufa-1.9999.js /
//!   maftufa-1.9999.js = z0 / z1 / maftufa)で一括比較した掃引の結果。
//!   再実行: `bash tests/data/run_gap_sweep.sh`
//!   (tests/data/gap_probes.txt + tests/data/gap_sweep_results.csv に
//!   プローブと比較表を保存)
//! - 各テストは GAP の解消をアサートするため、現時点では必ず失敗する(RED)。
//!   解消時は本テストが緑になる。既存テストに同一入力の「現状の拒否」を
//!   ピンしている箇所がある場合は、そちらも同時に更新すること
//!   (各テストのコメントに記載)。
//! - z0 のみの緩受理(裸 mo'i / fe'e タグ位置)、無ポーズ隣接(caku 等)、
//!   v0.97 で見送りを明記した尾部形 quantifier+sumti(lo pa le gerku ku)
//!   は意図的差分として GAP に入れていない(比較表には残っている)。

// テスト名は既存規約に倣い日本語 + GAP_ 接頭辞とする(大文字開始のため
// non_snake_case 警告をファイル内で抑止)
#![allow(non_snake_case)]

#[test]
fn GAP_レタル接頭lujvo_byklesi() {
    // 対象文: 「lo byklesi ku」「mi byklesi」「lo cyklesi ku」
    //   (by/cy 等のレタル語 + brivla の無空白融合 lujvo)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 4 形)
    // 原因推定: レタル接頭 lujvo の解析未対応。BY_core は語境界ガード
    //   (lojban.pest L842-845)により by+brivla 融合語には一致せず、
    //   tanru_unit の BRIVLA 枝のレタル・ガード群(lojban.pest L412-431)も
    //   レタル接頭形の経路を持たない
    // 発見経緯: 既知 GAP(v0.106 STATUS 記録)。掃引 L1369-1372 で裏取り。
    //   現状の拒否を tests/syntax.rs L3274-3276 がピンしているため、
    //   解消時はそちらも更新すること
    for input in ["lo byklesi ku", "mi byklesi", "lo cyklesi ku"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_FA_nai_不定格の否定() {
    // 対象文: 「fa nai mi klama」「mi klama fa nai」「fai nai mi klama」
    //   (FA スロット割当て語 + NAI)
    // 実測: z0=ok / z1=ok / maftufa=err(参照側で見解が分かれるが z0/z1 は受理)、
    //   本パーサー=err(全 5 形: fa/fe/fi/fo/fu/fi'a/fai × nai)
    // 原因推定: tagged の FA 枝(lojban.pest L187)が NAI を取らない
    //   (L185-186 の注記に既知 GAP として記録済み)
    // 発見経緯: 既知 GAP(v0.105 STATUS 記録)。掃引 L1373-1377 で裏取り。
    //   現状の拒否を tests/syntax.rs L3108-3109(fai nai)がピンしているため、
    //   解消時はそちらも更新すること
    for input in ["fa nai mi klama", "mi klama fa nai", "fai nai mi klama"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は z0/z1 が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_接続詞疑問_ji() {
    // 対象文: 「mi ji do klama」「mi ji do klama lo zdani」「do ji mi broda」
    //   (ji = 接続詞疑問。項・selbri・文の接続に用いる)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 3 形)
    // 原因推定: ji が JOI_core(lojban.pest L866-867)/JA_core(L850)の
    //   いずれにも未収録(L861-863 の注記に GAP として記録済み)。
    //   zantufa は JOI に ji を含み「mi ji do klama」を JOI の項接続で受理する
    // 発見経緯: 既知差分(v0.101 STATUS 記録)。掃引 L1378-1380 で裏取り
    for input in [
        "mi ji do klama",
        "mi ji do klama lo zdani",
        "do ji mi broda",
    ] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_単独_vau_と_se_vau() {
    // 対象文: 「va'u mi klama」「se va'u mi klama」「mi klama se va'u lo nu broda」
    //   (va'u = BAI タグ「~のおかげで」。結合形 seva'u の単独 2 語形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 3 形)
    // 原因推定: va'u が BAI_core(lojban.pest L750-768)未収録のため、
    //   単独 va'u のタグも SE 変換の 2 語形 se va'u も解析不能
    //   (SEBAI_joint 注記 L927-929 に「意図的に維持する」と記録済みの揺れ)
    // 発見経緯: 既知差分(SEBAI_joint 注記)。掃引 L1381-1383 で裏取り
    for input in [
        "va'u mi klama",
        "se va'u mi klama",
        "mi klama se va'u lo nu broda",
    ] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_free後のco転換selbri継続() {
    // 対象文: 「farlu ju'i co cnita」「.oi ta ca'o farlu ju'i co cnita」
    //   (tanru 単位間の自由修飾語 ju'i に続き co 転換 selbri が継続する形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 3 形)
    // 原因推定: tanru_post(lojban.pest L390-393)の継続枝
    //   (tanru_link|tanru_unit|DOhU)に co 枝がなく、free 後の co 転換
    //   selbri 継続を受理できない(L387-389 の注記に既知 GAP として記録済み)
    // 発見経緯: 既知 GAP(v0.103 STATUS 記録)。掃引 L1384-1386 で裏取り
    for input in ["farlu ju'i co cnita", ".oi ta ca'o farlu ju'i co cnita"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_裸tanru_BO接続() {
    // 対象文: 「mi klama bo cadzu」(tanru 単位を BO で短スコープ接続)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err
    // 原因推定: tanru_link(lojban.pest L522)は JA 付き接続のみで、
    //   裸 BO の selbri 接続(zantufa の selbri_6 相当)が未実装
    //   (L327 の注記に「未実装」と記録済み)
    // 発見経緯: 既知 GAP(v0.95 STATUS 記録)。掃引 L293(BO クラスの
    //   タグ位置プローブから副次的に捕捉)で裏取り。
    //   現状の拒否を tests/syntax.rs L891-893 がピンしているため、
    //   解消時はそちらも更新すること
    assert!(
        lojban::parse("mi klama bo cadzu").is_ok(),
        "GAP: 「mi klama bo cadzu」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
    );
}

#[test]
fn GAP_JOIによるselbri接続() {
    // 対象文: 「mi broda joi brode」「mi broda jo'e brode」
    //   「mi broda fa'u brode」「mi broda ku'a brode」(jo'u / johu も同様)
    //   (bridi_tail の連結部に JOI 系の非論理接続詞を置く形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 6 形)
    // 原因推定: gihek_link(lojban.pest L521)が GIhA(+NAI/BO)のみで
    //   JOI/A を含まない。camxes 系の gihek は jek/joik を含むため
    //   bridi_tail の JOI 接続が受理される。
    //   項接続(ek_joik)と mex 演算子(mex_conn)は v0.101 で対応済みだが
    //   bridi_tail 連結部は非対応のまま
    // 発見経緯: 新規発見(掃引 L636-641。JOI クラスの selbri 接続プローブ)
    for input in [
        "mi broda joi brode",
        "mi broda jo'e brode",
        "mi broda fa'u brode",
        "mi broda ku'a brode",
    ] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_VUhO後の関係節共有() {
    // 対象文: 「mi viska lo broda vu'o noi mi klama」
    //   (vu'o で sumti を連結した上で関係節を共有する形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(h 変体 vuho も同様)
    // 原因推定: sumti の繰り返し部(lojban.pest L223-224)の VUhO 枝は
    //   「VUhO ~ sumti」(項どうしの連結)のみで、zantufa/camxes が持つ
    //   「vu'o + relative_clauses」(関係節の共有)経路が存在しない
    // 発見経緯: 新規発見(掃引 L1064-1065。VUhO クラスのプローブ)
    assert!(
        lojban::parse("mi viska lo broda vu'o noi mi klama").is_ok(),
        "GAP: 「mi viska lo broda vu'o noi mi klama」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
    );
}

#[test]
fn GAP_時制タグの_nai() {
    // 対象文: 「mi pu nai klama」「mi ba nai klama」「mi ca nai klama」
    //   (時制タグ + NAI。「過去ではない」等の否定付き時制)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err(全 3 形)
    // 原因推定: tense_mark(lojban.pest L330-338)の NAI 後置は BAI 枝
    //   (L331)と間隔プロパティ ip_tail(L357-358)にしかなく、
    //   PU/CAhA/ZAhO/ZI/VA 等の時制タグには NAI を後置できない
    // 発見経緯: 新規発見(掃引 L1066。NAI クラスのプローブ)
    for input in ["mi pu nai klama", "mi ba nai klama", "mi ca nai klama"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_zoi区切り語のピリオド正規化() {
    // 対象文: 「mi cusku zoi gy. broda .gy」「zoi gy. broda .gy」
    //   (区切り語の前後にポーズのピリオドを置く標準的な書記形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err。
    //   なお「zoi gy English text gy」(ピリオドなし)と
    //   「zoi .broda. broda .broda.」(前後対称)は本パーサーも受理する
    // 原因推定: lib.rs の normalize_zoi(L352-423)が区切り語を
    //   生トークンの完全一致で比較するため「gy.」と「.gy」が不一致となり
    //   未閉鎖扱いになる。zantufa/camxes は区切り語前後のピリオドを
    //   ポーズ記号(区切り語の一部ではない)として扱う。
    //   zoi_quote 規則は lojban.pest L514
    // 発見経緯: 新規発見(掃引 L1368。ZOI クラスのプローブ)
    for input in ["mi cusku zoi gy. broda .gy", "zoi gy. broda .gy"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_jai_se変換タグ() {
    // 対象文: 「mi jai se gau broda」「mi jai se gau klama lo zdani」
    //   (JAI + SE 変換タグ。jai gau の se 変換形)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err。
    //   なお裸の「jai se gau」(フラグメント)は z0=err / z1=ok / maftufa=ok で
    //   参照側が割れるため本テストには含めない
    // 原因推定: tanru_unit の JAI 枝(lojban.pest L447)は JAI 直後に
    //   tense_mark か tanru_unit のみを取り、SE 変換タグ(se gau 等)を
    //   間に挟めない
    // 発見経緯: 既知 GAP 候補(掃引 L1390-1392 で裏取り)
    for input in ["mi jai se gau broda", "mi jai se gau klama lo zdani"] {
        assert!(
            lojban::parse(input).is_ok(),
            "GAP: 「{input}」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
        );
    }
}

#[test]
fn GAP_ke_group内の項() {
    // 対象文: 「mi klama ke lo zdani broda ke'e」
    //   (ke … ke'e グループ内に sumti + selbri を含む bridi_tail グループ)
    // 実測: z0=ok / z1=ok / maftufa=ok、本パーサー=err
    // 原因推定: ke_group(lojban.pest L459)は「KE ~ selbri ~ KEhE?」で
    //   selbri(tanru 単位列)しか括れず、zantufa/camxes が許す
    //   bridi_tail(項+selbri)単位の ke グループ化に対応していない
    // 発見経緯: 新規発見(掃引 L1408。KEhE クラスの構造プローブ)
    assert!(
        lojban::parse("mi klama ke lo zdani broda ke'e").is_ok(),
        "GAP: 「mi klama ke lo zdani broda ke'e」は参照パーサー(z0/z1/maftufa)が受理するが本パーサーは拒否する"
    );
}
