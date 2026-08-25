//! 統語レイヤーの統合テスト
//!
//! 基本文型が解析木として正しく構築されるかを検証する。

use lojban::grammar::{LojbanParser, Rule};
use lojban::tree::to_sexpr;
use pest::Parser;

fn parse_ok(input: &str) -> String {
    // 公開 API 経由(ZOI 正規化などの前処理を含む)
    let pairs = lojban::parse(input).unwrap_or_else(|e| panic!("解析失敗: {input:?}: {e}"));
    to_sexpr(pairs)
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

#[test]
fn tanru接続_je() {
    let s = parse_ok("mi viska melbi je cmalu nixli");
    assert!(s.contains("JA_core \"je\""), "{s}");
}

#[test]
fn sumti接続_e() {
    let s = parse_ok("mi e do klama");
    assert!(s.contains("A_core \"e\""), "{s}");
    let s = parse_ok("mi joi do klama");
    assert!(s.contains("JOI_core \"joi\""), "{s}");
}

#[test]
fn bridi_tail接続_gihe() {
    let s = parse_ok("mi nelci gi'e viska do");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
}

#[test]
fn 文接続_ije() {
    let s = parse_ok("mi klama .ije do stali");
    // 結合形(.ije)は IJ_joint ノードになる
    assert!(s.contains("IJ_joint \"ije\""), "{s}");
    let s = parse_ok("mi klama .i je do stali");
    assert!(s.contains("JA_core \"je\""), "{s}");
}

#[test]
fn 文接続_結合形_ijanai() {
    let s = parse_ok("mi klama .ijanai do stali");
    // JA_bare は silent のため IJ_joint ノードと NAI のみ確認
    assert!(s.contains("IJ_joint"), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
}

#[test]
fn 先接続詞_ge_gi() {
    let s = parse_ok("ge mi klama gi do cadzu");
    assert!(s.contains("gek_sentence"), "{s}");
    let s = parse_ok("mi viska ge le gerku gi le mlatu");
    assert!(s.contains("GI_core \"gi\""), "{s}");
}

#[test]
fn 接続詞_nai() {
    let s = parse_ok("mi e nai do klama");
    assert!(s.contains("A_core \"e\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    let s = parse_ok("ta melbi je nai cmalu");
    assert!(s.contains("JA_core \"je\""), "{s}");
}

#[test]
fn 不完全な接続は拒否される() {
    // 連鎖先のない gi'e
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e").is_err());
    // 閉鎖のない先接続
    assert!(LojbanParser::parse(Rule::text, "ge mi klama").is_err());
    // 接続先のない je
    assert!(LojbanParser::parse(Rule::text, "mi viska je").is_err());
}

#[test]
fn bai_タグの項() {
    let s = parse_ok("mi tavla do bau la lojban.");
    assert!(s.contains("BAI_core \"bau\""), "{s}");
}

#[test]
fn bai_タグの文頭() {
    let s = parse_ok("mu'i le nu do klama mi gleki");
    assert!(s.contains("BAI_core \"mu'i\""), "{s}");
    let s = parse_ok("mi ba zi vitke do mu'i le nu penmi");
    assert!(s.contains("BAI_core \"mu'i\""), "{s}");
}

#[test]
fn nahe_述語スケール反転() {
    let s = parse_ok("mi na'e prami do");
    assert!(s.contains("NAhE_core \"na'e\""), "{s}");
    let s = parse_ok("ta to'e melbi");
    assert!(s.contains("NAhE_core \"to'e\""), "{s}");
}

#[test]
fn 接続詞のboグルーピング() {
    let s = parse_ok("mi joi bo do klama");
    assert!(s.contains("BO_core \"bo\""), "{s}");
    let s = parse_ok("ta melbi je bo cmalu zdani");
    assert!(s.contains("BO_core \"bo\""), "{s}");
}

#[test]
fn bihi_間隔接続() {
    let s = parse_ok("mi klama pa bi'o re");
    assert!(s.contains("BIhI_core \"bi'o\""), "{s}");
    // 連鎖先のない間隔接続は拒否
    assert!(LojbanParser::parse(Rule::text, "mi klama bi'o").is_err());
}

#[test]
fn li_による数式の項() {
    let s = parse_ok("li re su'i re du li vo");
    assert!(s.contains("VUhU_core \"su'i\""), "{s}");
    assert!(s.contains("li_mex"), "{s}");
}

#[test]
fn 演算子連鎖と括弧() {
    let s = parse_ok("li pa su'i re pi'i ci");
    assert!(s.contains("VUhU_core \"pi'i\""), "{s}");
    let s = parse_ok("li vei pa su'i re ve'o pi'i ci");
    assert!(s.contains("VEI_core \"vei\""), "{s}");
}

#[test]
fn 負数と桁区切り() {
    let s = parse_ok("li ni'u ci du li vo vu'u ze");
    assert!(s.contains("PA_core \"ni'u\""), "{s}");
    let s = parse_ok("li pa ki'o re ci");
    assert!(s.contains("PA_core \"ki'o\""), "{s}");
}

#[test]
fn 不完全な数式は拒否される() {
    // 演算子で終わる数式
    assert!(LojbanParser::parse(Rule::text, "li pa su'i").is_err());
    // 被演算子のない数式
    assert!(LojbanParser::parse(Rule::text, "li su'i").is_err());
}

#[test]
fn 描述内の数式() {
    let s = parse_ok("mi viska le re su'i ci gerku");
    assert!(s.contains("VUhU_core \"su'i\""), "{s}");
    // 従来の単純数量詞も引き続き受理
    let s = parse_ok("mi viska le ci gerku");
    assert!(s.contains("PA_core \"ci\""), "{s}");
}

#[test]
fn 不完全な描述内数式は拒否される() {
    // 演算子で終わる数量詞
    assert!(LojbanParser::parse(Rule::text, "mi viska le re su'i").is_err());
}

#[test]
fn zoi_による任意テキスト引用() {
    let s = parse_ok("mi cusku zoi .ky. hello world .ky.");
    assert!(s.contains("zoi_quote"), "{s}");
    let s = parse_ok("zoi gy English text gy cu gliru");
    assert!(s.contains("ZOI_core \"zoi\""), "{s}");
}

#[test]
fn lerfu_文字参照は項() {
    let s = parse_ok("mi viska xy");
    assert!(s.contains("BY_core \"xy\""), "{s}");
    let s = parse_ok("abu prami do");
    assert!(s.contains("BY_core \"abu\""), "{s}");
}

#[test]
fn lerfu_数式内() {
    let s = parse_ok("li xy su'i re du li vo");
    assert!(s.contains("BY_core \"xy\""), "{s}");
}

#[test]
fn quant_selbri_は従来形のみ() {
    // 注: quant_selbri への mex 埋め込みは文字参照との貪欲競合のため不採用
    let s = parse_ok("pa prenu cu klama");
    assert!(s.contains("PA_core \"pa\""), "{s}");
}

#[test]
fn bu_による文字化() {
    // 代名詞の文字化が項として機能
    let s = parse_ok("mi bu prami do");
    assert!(s.contains("bu_lerfu"), "{s}");
    assert!(s.contains("BU_core \"bu\""), "{s}");
    // cmavo の文字化
    let s = parse_ok("mi cusku zo'e bu");
    assert!(s.contains("bu_lerfu"), "{s}");
}

#[test]
fn bu_なしでは従来どおり() {
    // bu が続かない場合は通常の項として解析される
    let s = parse_ok("mi prami do");
    assert!(!s.contains("bu_lerfu"), "{s}");
}

#[test]
fn 先接続_述語_guhe() {
    let s = parse_ok("mi gu'e klama gi cadzu");
    assert!(s.contains("GUhA_core \"gu'e\""), "{s}");
    // NAhE との組み合わせ(camxes selbri_6 準拠)
    let s = parse_ok("ta na'e gu'e melbi gi cmalu");
    assert!(s.contains("NAhE_core \"na'e\""), "{s}");
}

#[test]
fn gaho_間隔端点() {
    let s = parse_ok("mi klama pa ga'o bi'o ke'i re");
    assert!(s.contains("GAhO_core \"ga'o\""), "{s}");
    let s = parse_ok("mi klama pa bi'o re");
    // 従来形(GAhO 省略)も後方互換
    assert!(s.contains("BIhI_core \"bi'o\""), "{s}");
}

#[test]
fn 空間時制_faha() {
    // 文頭の時制マーク位置
    let s = parse_ok("mi pu ca'u klama le zdani");
    assert!(s.contains("FAhA_core \"ca'u\""), "{s}");
    // selbri 頭の時制マーク位置
    let s = parse_ok("le gerku cu ne'a batci le mlatu");
    assert!(s.contains("FAhA_core \"ne'a\""), "{s}");
}

#[test]
fn 移動時制_mohi() {
    let s = parse_ok("mi mo'i ca'u klama");
    assert!(s.contains("MOhI_core \"mo'i\""), "{s}");
    assert!(s.contains("FAhA_core"), "{s}");
}

#[test]
fn lahe_項修飾() {
    let s = parse_ok("mi nelci lu'e le cukta");
    assert!(s.contains("LAhE_core \"lu'e\""), "{s}");
    let s = parse_ok("la'e di'u cu xamgu");
    assert!(s.contains("LAhE_core \"la'e\""), "{s}");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
}

#[test]
fn koha_補完語() {
    for w in ["mi'a", "ma'a", "do'o", "di'u"] {
        let s = parse_ok(&format!("{w} klama"));
        assert!(s.contains(&format!("KOhA_core \"{w}\"")), "{s}");
    }
}

#[test]
fn naku_項否定() {
    let s = parse_ok("naku le gerku cu batci le mlatu");
    assert!(s.contains("NAKU_joint"), "{s}");
    let s = parse_ok("mi naku klama");
    assert!(s.contains("NAKU_joint"), "{s}");
    let s = parse_ok("mi na ku klama");
    assert!(s.contains("NA_clause") && s.contains("KU_core"), "{s}");
}

#[test]
fn 素のnaは項になれない() {
    assert!(lojban::parse("na le gerku cu batci").is_err());
}

#[test]
fn 時制タグ_sumti付き() {
    // selbri 前の項位置
    let s = parse_ok("mi ca le cabdei cu klama");
    assert!(s.contains("PU_core \"ca\""), "{s}");
    assert!(s.contains("tagged"), "{s}");
    assert!(s.contains("LE_core \"le\""), "{s}");
    // selbri 後の項位置
    let s = parse_ok("mi klama ca le cabdei");
    assert!(s.contains("PU_core \"ca\""), "{s}");
}

#[test]
fn 空間タグと_ku_閉鎖() {
    // VA + FAhA の連鎖タグ
    let s = parse_ok("mi vi ne'i le zdani cu klama");
    assert!(s.contains("VA_core \"vi\""), "{s}");
    assert!(s.contains("FAhA_core \"ne'i\""), "{s}");
    // ku による明示閉鎖
    let s = parse_ok("mi klama pu le cabdei ku");
    assert!(s.contains("KU_core"), "{s}");
}

#[test]
fn 期間_zei() {
    let s = parse_ok("mi ze'a lo cacra cu tavla do");
    assert!(s.contains("ZI_core \"ze'a\""), "{s}");
}

#[test]
fn 肯定_jaha() {
    let s = parse_ok("mi ja'a klama");
    assert!(s.contains("JAhA_core \"ja'a\""), "{s}");
    // 応答表現 ja'a go'i
    let s = parse_ok("ja'a go'i");
    assert!(s.contains("JAhA_core"), "{s}");
    assert!(s.contains("GOhA_core \"go'i\""), "{s}");
}

#[test]
fn koha_tua_dei() {
    let s = parse_ok("mi djica tu'a do");
    assert!(s.contains("KOhA_core \"tu'a\""), "{s}");
    let s = parse_ok("dei jetnu");
    assert!(s.contains("KOhA_core \"dei\""), "{s}");
}

#[test]
fn 記述詞_lohe() {
    let s = parse_ok("lo'e gerku cu batci");
    assert!(s.contains("LE_core \"lo'e\""), "{s}");
}

#[test]
fn 感情標識_追加語彙() {
    let s = parse_ok("e'o ko cusku");
    assert!(s.contains("UI_core \"e'o\""), "{s}");
    let s = parse_ok("mi gleki bu'o");
    assert!(s.contains("UI_core \"bu'o\""), "{s}");
}

#[test]
fn fio_モダルタグ() {
    // fe'u による明示閉鎖
    let s = parse_ok("ti fi'o dunda fe'u do cukta");
    assert!(s.contains("FIhO_core"), "{s}");
    assert!(s.contains("FEhU_core"), "{s}");
    // fe'u 省略形(selbri は tail_terms を含まないため境界が自明)
    let s = parse_ok("ti fi'o dunda do cukta");
    assert!(s.contains("FIhO_core"), "{s}");
}

#[test]
fn se_bai_変換タグ() {
    let s = parse_ok("mi se ki'u le nu do sidju cu snada");
    assert!(s.contains("SE_core \"se\""), "{s}");
    assert!(s.contains("BAI_core \"ki'u\""), "{s}");
}

#[test]
fn 先接続_ganai() {
    // 結合表記 ganai … gi …(if-then の標準形)
    let s = parse_ok("ganai do klama gi mi cadzu");
    assert!(s.contains("gek_sentence"), "{s}");
    assert!(s.contains("GANAI_joint"), "{s}");
    // 分離表記 ga nai
    let s = parse_ok("ga nai do klama gi mi cadzu");
    assert!(s.contains("GA_core \"ga\""), "{s}");
    assert!(s.contains("NAI_clause"), "{s}");
    // 後半否定: 分離形 gi nai は GI+NAI、結合形 ginai は GINAI_joint
    let s = parse_ok("ganai do klama gi nai mi cadzu");
    assert!(
        s.contains("GI_core \"gi\"") && s.contains("NAI_clause"),
        "{s}"
    );
    let s = parse_ok("ganai do klama ginai mi cadzu");
    assert!(s.contains("GINAI_joint"), "{s}");
}

#[test]
fn 先接続_sumti_gek() {
    let s = parse_ok("mi viska ge lo gerku gi lo mlatu");
    assert!(s.contains("gek_sumti"), "{s}");
}

#[test]
fn 結合_sebai_タグ() {
    let s = parse_ok("mi pilno sepi'o lo xarju");
    assert!(s.contains("SEBAI_joint \"sepi'o\""), "{s}");
    // 文頭のタグ位置でも受理
    let s = parse_ok("seva'u do mi klama");
    assert!(s.contains("SEBAI_joint \"seva'u\""), "{s}");
}

#[test]
fn 時制間隔_zeha() {
    let s = parse_ok("mi pu bi'o ba klama");
    assert!(s.contains("ZEhA_core \"bi'o\""), "{s}");
    let s = parse_ok("mi ca bi'i ba tavla do");
    assert!(s.contains("ZEhA_core \"bi'i\""), "{s}");
}

#[test]
fn bai_nai_否定タグ() {
    let s = parse_ok("mi klama ri'a nai le nu carvi");
    assert!(s.contains("BAI_core \"ri'a\""), "{s}");
    assert!(s.contains("NAI_clause"), "{s}");
    // 文頭タグ位置
    let s = parse_ok("mu'i nai le nu do gleki mi cu prami do");
    assert!(s.contains("NAI_clause"), "{s}");
}

#[test]
fn 結合_sedu() {
    // 結合形 sedu'u を fu'ivla として誤認しない(項位置では抽象、
    // selbri 直後は PEG の優先順位で nu_form として取り込まれる)
    let s = parse_ok("mi cusku sedu'u broda");
    assert!(s.contains("SEDUHU_joint"), "{s}");
    let s = parse_ok("sedu'u broda cu jitfa");
    assert!(s.contains("abstraction"), "{s}");
    // 分離形 se du'u は従来どおり s_marks 経由
    let s = parse_ok("mi cusku se du'u broda");
    assert!(
        s.contains("SE_core \"se\"") && s.contains("NU_core \"du'u\""),
        "{s}"
    );
}

#[test]
fn 文接続_ibo() {
    let s = parse_ok("mi klama .ibo do cadzu");
    assert!(s.contains("IBO_joint"), "{s}");
    let s = parse_ok("mi klama .ijebo do cadzu");
    assert!(s.contains("IBO_joint"), "{s}");
    // 分離形 .i bo
    let s = parse_ok("mi klama .i bo do cadzu");
    assert!(s.contains("BO_clause"), "{s}");
}

#[test]
fn 時制疑問_cue() {
    let s = parse_ok("do cu'e klama");
    assert!(s.contains("CUhE_core \"cu'e\""), "{s}");
}

#[test]
fn 述語連鎖_bo_グルーピング() {
    let s = parse_ok("mi nelci gi'e bo citka");
    assert!(s.contains("GIhA_core") && s.contains("BO_core"), "{s}");
}

#[test]
fn 発話序数_mai() {
    // .i 直後
    let s = parse_ok(".i pamai mi klama");
    assert!(s.contains("MAI_core \"pamai\""), "{s}");
    // 文末(自由修飾語)
    let s = parse_ok("mi klama pamai");
    assert!(s.contains("MAI_core"), "{s}");
}

#[test]
fn 感情強度_cai() {
    let s = parse_ok("ui sai do gleki");
    assert!(s.contains("UI_core \"ui\""), "{s}");
    assert!(s.contains("CAI_core \"sai\""), "{s}");
    let s = parse_ok("mi gleki ui cu'i");
    assert!(s.contains("CAI_core \"cu'i\""), "{s}");
}

#[test]
fn 数終端_boi() {
    let s = parse_ok("li re boi su'i ci du li mu");
    assert!(s.contains("BOI_core \"boi\""), "{s}");
}

#[test]
fn 談話標識_語彙拡充() {
    for w in ["ku'i", "ja'o", "po'o", "da'i", "je'u"] {
        let s = parse_ok(&format!("{w} mi klama"));
        assert!(s.contains(&format!("UI_core \"{w}\"")), "{s}");
    }
    // 文中挿入(項と述語の間)
    let s = parse_ok("mi ku'i cadzu");
    assert!(s.contains("UI_core \"ku'i\""), "{s}");
}

#[test]
fn 感情語彙_追加() {
    let s = parse_ok("u'o mi klama");
    assert!(s.contains("UI_core \"u'o\""), "{s}");
    let s = parse_ok("mi gleki ga'i");
    assert!(s.contains("UI_core \"ga'i\""), "{s}");
}

#[test]
fn 語彙拡充_coi_bai_joi_pa() {
    let s = parse_ok("vi'o mi klama");
    assert!(s.contains("COI_core \"vi'o\""), "{s}");
    let s = parse_ok("ci'u le nu broda mi co'a gleki");
    assert!(s.contains("BAI_core \"ci'u\""), "{s}");
    let s = parse_ok("mi jo'u do klama");
    assert!(s.contains("JOI_core \"jo'u\""), "{s}");
    let s = parse_ok("su'e re prenu cu klama");
    assert!(s.contains("PA_core \"su'e\""), "{s}");
}

#[test]
fn mex_nahu_演算子() {
    let s = parse_ok("li re na'u zmadu ci du li vo");
    assert!(s.contains("NAhU_core \"na'u\""), "{s}");
    // SE 変換演算子
    let s = parse_ok("li re se pi'i ci du li xa");
    assert!(
        s.contains("SE_core \"se\"") && s.contains("VUhU_core \"pi'i\""),
        "{s}"
    );
}

#[test]
fn mex_moe_被演算子() {
    let s = parse_ok("li mo'e ti su'i re du li ci");
    assert!(s.contains("MOhE_core \"mo'e\""), "{s}");
}

#[test]
fn 時制固定_ki() {
    let s = parse_ok("mi ba ki klama");
    assert!(s.contains("KI_core \"ki\""), "{s}");
}

#[test]
fn 時制後の述語マーク() {
    let s = parse_ok("ti ba se citka");
    assert!(s.contains("SE_core \"se\""), "{s}");
    let s = parse_ok("ko'a pu zi je'a citka");
    assert!(s.contains("NAhE_core \"je'a\""), "{s}");
}

#[test]
fn 抽象内の先接続() {
    let s = parse_ok("mi troci lo ka ganai broda gi brode");
    assert!(s.contains("gek_sentence"), "{s}");
}

#[test]
fn 談話標識_zuu_baa() {
    let s = parse_ok("zu'u do klama");
    assert!(s.contains("UI_core \"zu'u\""), "{s}");
    let s = parse_ok("ba'a mi snada");
    assert!(s.contains("UI_core \"ba'a\""), "{s}");
}

#[test]
fn 結合感情否定_uinai() {
    let s = parse_ok("ta'onai mi co'a jgari");
    assert!(s.contains("UINAI_joint"), "{s}");
}

#[test]
fn 連結数詞() {
    let s = parse_ok("de'i li renono mi klama");
    assert!(s.contains("LI_core"), "{s}");
    assert!(s.contains("renono"), "{s}");
}

#[test]
fn 感情標識_ai_au_kia() {
    let s = parse_ok(".ai mi klama");
    assert!(s.contains("UI_core \"ai\""), "{s}");
    let s = parse_ok("au forca");
    assert!(s.contains("UI_core \"au\""), "{s}");
    let s = parse_ok("ki'a do cusku ma");
    assert!(s.contains("UI_core \"ki'a\""), "{s}");
}

#[test]
fn 呼格_naiと_miai() {
    let s = parse_ok("ju'i nai do klama");
    assert!(
        s.contains("COI_core \"ju'i\"") && s.contains("NAI_clause"),
        "{s}"
    );
    let s = parse_ok("nabmi mi'ai");
    assert!(s.contains("KOhA_core \"mi'ai\""), "{s}");
    // フラグメント+自由修飾語
    let s = parse_ok("mi'a uu");
    assert!(
        s.contains("KOhA_core \"mi'a\"") && s.contains("UI_core \"uu\""),
        "{s}"
    );
}

#[test]
fn 空間間隔_veha_viha() {
    // VEhA + FAhA をタグとして(sumti を取る)
    let s = parse_ok("mi ve'i ne'i le zdani cu klama");
    assert!(s.contains("VEhA_core \"ve'i\""), "{s}");
    assert!(s.contains("FAhA_core \"ne'i\""), "{s}");
    // selbri 頭: VIhA + FAhA
    let s = parse_ok("lo gerku cu vi'a ca'u batci");
    assert!(s.contains("VIhA_core \"vi'a\""), "{s}");
}

#[test]
fn 先置数理_peho() {
    let s = parse_ok("li peho su'i re ci kuhe du li mu");
    assert!(s.contains("PEhO_core"), "{s}");
    // kuhe 省略形
    let s = parse_ok("li peho pi'i vo mu du li xa");
    assert!(s.contains("PEhO_core"), "{s}");
}

#[test]
fn 数理演算子_maho() {
    let s = parse_ok("li re ma'o ny ci du li vo");
    assert!(s.contains("MAhO_core"), "{s}");
}

#[test]
fn 前置スコープ_zou() {
    let s = parse_ok("su'o da zo'u da prami mi");
    assert!(s.contains("ZOhU_core \"zo'u\""), "{s}");
    // 前置スコープ + 先接続文
    let s = parse_ok("ro da zo'u ganai da broda gi da brode");
    assert!(s.contains("ZOhU_core"), "{s}");
    assert!(s.contains("gek_sentence"), "{s}");
}

#[test]
fn 抽象内の前置スコープ() {
    let s = parse_ok("mi jinvi lo du'u su'o da zo'u da nenri");
    assert!(s.contains("ZOhU_core"), "{s}");
}

#[test]
fn 演算子終端_tehu() {
    let s = parse_ok("li re na'u zmadu te'u ci du li mu");
    assert!(s.contains("TEhU_core"), "{s}");
}

#[test]
fn jai_変換() {
    // JAI + タグ + tanru_unit(camxes tanru_unit_2 準拠)
    let s = parse_ok("mi jai gau zdani");
    assert!(s.contains("JAI_core \"jai\""), "{s}");
    assert!(s.contains("BAI_core \"gau\""), "{s}");
    // タグ省略形
    let s = parse_ok("mi jai zdani");
    assert!(s.contains("JAI_core"), "{s}");
}

#[test]
fn 数詞_moi_述語() {
    let s = parse_ok("mi re moi");
    assert!(s.contains("MOI_core \"moi\""), "{s}");
    let s = parse_ok("mi ci mei");
    assert!(s.contains("MOI_core \"mei\""), "{s}");
    // 描述内の tanru
    let s = parse_ok("le re moi prenu cu klama");
    assert!(s.contains("MOI_core"), "{s}");
}

#[test]
fn me_meu() {
    let s = parse_ok("ti me mi me'u");
    assert!(s.contains("MEhU_core \"me'u\""), "{s}");
    // me'u 省略形は従来どおり
    let s = parse_ok("ti me mi");
    assert!(s.contains("ME_core \"me\""), "{s}");
}

#[test]
fn zei_複合語() {
    let s = parse_ok("mi tavla lo zdani zei sinxa");
    assert!(s.contains("ZEI_core"), "{s}");
    // 3語連結
    let s = parse_ok("lo melbi zei cmalu zei noltru cu cizra");
    assert!(s.contains("ZEI_core"), "{s}");
}

#[test]
fn soi_入れ替え() {
    let s = parse_ok("mi prami do soi vo'a vo'e");
    assert!(s.contains("SOI_core \"soi\""), "{s}");
    // se'u 省略形・sumti 1個
    let s = parse_ok("do se prami mi soi vo'a");
    assert!(s.contains("SOI_core"), "{s}");
}

#[test]
fn 添字_xi() {
    let s = parse_ok("mi viska lo gerku xi re");
    assert!(s.contains("XI_core \"xi\""), "{s}");
    // 文字語への添字
    let s = parse_ok("mi nelci li xy xi pa");
    assert!(s.contains("XI_core"), "{s}");
}

#[test]
fn daho_自由修飾語() {
    let s = parse_ok("su'o da zo'u da klama .i da'o");
    assert!(s.contains("DAhO_core \"da'o\""), "{s}");
}

#[test]
fn 自由修飾語_先接続() {
    let s = parse_ok("xu ganai broda gi brode");
    assert!(s.contains("gek_sentence"), "{s}");
    assert!(s.contains("UI_core \"xu\""), "{s}");
    let s = parse_ok("pe'i ganai mi klama gi mi cadzu");
    assert!(s.contains("gek_sentence"), "{s}");
}

#[test]
fn 項set_nuhi() {
    let s = parse_ok("mi zmadu nu'i la djan. le ka cuxna");
    assert!(s.contains("NUhI_core \"nu'i\""), "{s}");
    // nu'u による明示閉鎖
    let s = parse_ok("mi nelci nu'i do nu'u");
    assert!(s.contains("NUhU_core \"nu'u\""), "{s}");
}

#[test]
fn 感情スコープ_fuhe() {
    let s = parse_ok("fu'e ui mi klama fu'o");
    assert!(s.contains("FUhE_core \"fu'e\""), "{s}");
    assert!(s.contains("FUhO_core \"fu'o\""), "{s}");
}

#[test]
fn 項間の自由修飾語() {
    let s = parse_ok("mi .ui do tavla");
    assert!(
        s.contains("UI_core \".ui\"") || s.contains("UI_core \"ui\""),
        "{s}"
    );
    let s = parse_ok("mi ku'i do tavla");
    assert!(s.contains("UI_core \"ku'i\""), "{s}");
}

#[test]
fn 感情標識_eu_ou() {
    let s = parse_ok("e'u do klama");
    assert!(s.contains("UI_core \"e'u\""), "{s}");
    let s = parse_ok("o'u mi gleki");
    assert!(s.contains("UI_core \"o'u\""), "{s}");
}

#[test]
fn moi_be_連結() {
    let s = parse_ok("lo re moi be lo ci gerku cu barda");
    assert!(s.contains("MOI_core \"moi\""), "{s}");
    assert!(s.contains("BE_core \"be\""), "{s}");
}

#[test]
fn faho_と_vuo() {
    // fa'o = テキストの明示終端
    let s = parse_ok("mi klama fa'o");
    assert!(s.contains("FAhO_core"), "{s}");
    // vu'o = 項を連結して関係節を共有
    let s = parse_ok("mi vu'o do poi tavla mi cu prami");
    assert!(s.contains("VUhO_core \"vu'o\""), "{s}");
}

#[test]
fn cehe_項区切り() {
    let s = parse_ok("mi ce'e do tavla");
    assert!(s.contains("CEhE_core \"ce'e\""), "{s}");
}

#[test]
fn pehe_項グループ接続() {
    let s = parse_ok("mi zmadu lo klama pe'e je lo cadzu");
    assert!(s.contains("PEhE_core \"pe'e\""), "{s}");
    assert!(s.contains("JA_core \"je\""), "{s}");
    // 非論理接続
    let s = parse_ok("mi nelci lo gerku pe'e joi lo mlatu");
    assert!(s.contains("PEhE_core"), "{s}");
}

#[test]
fn 結合形と談話標識_バッテリー6() {
    // LAhE+KOhA 結合形
    let s = parse_ok("la'edi'u cu xamgu");
    assert!(s.contains("LAHEDI_joint"), "{s}");
    // 量化 ROI 結合形
    let s = parse_ok("roroi mi cadzu");
    assert!(s.contains("ROROI_joint"), "{s}");
    // UI+NAI 追加分
    let s = parse_ok("ji'anai mi gleki");
    assert!(s.contains("UINAI_joint"), "{s}");
    let s = parse_ok("ru'anai do drani");
    assert!(s.contains("UINAI_joint"), "{s}");
    // BAhE 強調
    let s = parse_ok("ba'e do viska mi");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
}

#[test]
fn mex_fihu_と_前置単項() {
    let s = parse_ok("li pa fi'u re du li pimu");
    assert!(s.contains("FIhU_core"), "{s}");
    let s = parse_ok("li va'a pa du li ni'u pa");
    assert!(s.contains("VUhU_core \"va'a\""), "{s}");
}

#[test]
fn bixe_演算子強調() {
    // 2 + (3 × 4): bi'e 以降が右結合
    let s = parse_ok("li re su'i ci bi'e pi'i vo du li pano");
    assert!(s.contains("BIhE_core \"bi'e\""), "{s}");
    // vei 内でも有効
    let s = parse_ok("li vei re bi'e pi'i ci ve'o su'i vo du li xa");
    assert!(s.contains("BIhE_core"), "{s}");
}

#[test]
fn fehe_空間間隔プロパティ() {
    let s = parse_ok("mi ve'i fe'e roi le zdani cu klama");
    assert!(s.contains("FEhE_core \"fe'e\""), "{s}");
    // 単独形
    let s = parse_ok("fe'e ru'i mi cadzu");
    assert!(s.contains("FEhE_core"), "{s}");
}

#[test]
fn pa_roi_複合タグ() {
    // 数詞を前置した ROI(so'u roi = まれに 等)。tense_mark から
    // interval_property 経由で到達する(zantufa 準拠)
    let s = parse_ok("so'u roi klama");
    assert!(s.contains("PA_seq \"so'u\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // PU との連鎖(pu re roi)。selbri 頭の時制位置でも受理
    let s = parse_ok("mi pu re roi klama");
    assert!(s.contains("PU_core \"pu\""), "{s}");
    assert!(s.contains("PA_seq \"re\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    // NAI 否定形(so'u roi nai = まれに…ない)。interval_property の
    // (sp1 ~ NAI_clause)? の回帰固定
    let s = parse_ok("so'u roi nai klama");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 代名詞主語の形も受理される(公開 API 経由)
    assert!(lojban::parse("mi so'u roi klama").is_ok());
    assert!(lojban::parse("mi re roi klama").is_ok());
}

#[test]
fn pa_roi_ku_閉鎖との相互作用() {
    // 複合タグの ku 明示閉鎖(tense_marks の (sp1 ~ KU_clause)?)
    let s = parse_ok("so'u roi ku klama");
    assert!(s.contains("PA_seq \"so'u\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 代名詞主語 + PU 連鎖でも ku まで時制側に取る
    assert!(lojban::parse("mi so'u roi ku klama").is_ok());
    let s = parse_ok("mi pu re roi ku klama");
    assert!(s.contains("PU_core \"pu\""), "{s}");
    assert!(s.contains("PA_seq \"re\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
}

#[test]
fn pa_tahe_zaho_複合タグ() {
    // interval_property の許容範囲: PA + TAhE / PA + ZAhO(so'i ta'e は
    // 非標準だが文法上の複合タグとして受理する)
    let s = parse_ok("so'i ta'e viska");
    assert!(s.contains("PA_seq \"so'i\""), "{s}");
    assert!(s.contains("TAhE_core \"ta'e\""), "{s}");
    let s = parse_ok("za'u co'u citka");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ZAhO_core \"co'u\""), "{s}");
}

#[test]
fn 末尾の文区切り() {
    // 裸区切り(.i / ni'o)は後続なしで発話を終えられる(zantufa 準拠。実文で頻出)
    parse_ok("mi klama .i");
    parse_ok("mi klama ni'o");
    // 先頭 sep(lead 経由)は従来どおり
    parse_ok(".i mi klama");
}

#[test]
fn 文区切りの連続() {
    // 裸区切りの後続 item が任意のため、文中の連続区切りも受理
    let s = parse_ok("mi klama .i .i do tavla");
    assert!(s.matches("sentence").count() >= 2, "{s}");
    assert_eq!(s.matches("I_core \"i\"").count(), 2, "{s}");
}

#[test]
fn 接続詞付き文区切りの後は文が必須() {
    // 接続詞付き sep(.ije / .ibo / .ijanai 等)の後の文は必須(zantufa 準拠)。
    // 接続詞だけ宙吊りの入力は拒否される
    assert!(lojban::parse("mi klama .i je").is_err());
    assert!(lojban::parse("mi klama .i bo").is_err());
    assert!(lojban::parse("mi klama .ijanai").is_err());
    assert!(lojban::parse("mi klama .ijebo").is_err());
    // 対比: 後続に文が続く接続付き sep は従来どおり受理
    parse_ok("mi klama .ijanai do tavla");
    parse_ok("mi klama .ije do tavla");
    parse_ok("mi klama .ibo do cadzu");
    // 文頭のリード(lead 経由)でも接続付き sep は従来どおり受理
    parse_ok(".ije do tavla");
    parse_ok(".i je do tavla");
    // 文頭かつ後続なしの宙吊りも拒否
    assert!(lojban::parse(".i je").is_err());
    assert!(lojban::parse(".ijanai").is_err());
}

#[test]
fn 対象文_pa_roi複合タグと末尾区切りを含む実文() {
    // zantufa が受理する実文(so'u roi 複合タグ + 末尾 .i を含む)
    let s = parse_ok("ni'o la .alis. co'a tatpi lo nu zutse lo rirxe korbi re'o lo mensi gi'e zukte fi no da .i .abu cu so'u roi sutra zgana lo cukta poi my tcidu .i ku'i cy vasru no pixra ja nuncasnu .i ");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("ZAhO_core \"co'a\""), "{s}");
    assert!(s.contains("FAhA_core \"re'o\""), "{s}");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(s.contains("PA_seq \"so'u\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    // 文区切り .i は3回(.abu / ku'i の前と末尾)
    assert_eq!(s.matches("I_core \"i\"").count(), 3, "{s}");
}
