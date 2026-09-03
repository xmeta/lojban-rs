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
    // 閉鎖 li'u は引用終端の LIhU(LUhU は la'e/lu'e 参照の閉鎖 lu'u)
    assert!(s.contains("LIhU_core"), "{s}");
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
    assert!(s.contains("LIhU_core \"li'u\""), "{s}");
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
fn tai_様態タグを含む実文() {
    // zantufa が受理する実文(BAI:tai + 描述内 tanru、末尾 .i を含む)
    let s =
        parse_ok(".i .oi lo kusru ci mei ni'a lo tai senva tcima cu ruble pikci fi lo lisri .i ");
    assert!(s.contains("BAI_core \"tai\""), "{s}");
    assert!(s.contains("FAhA_core \"ni'a\""), "{s}");
    // 文区切り .i は先頭と末尾
    assert_eq!(s.matches("I_core \"i\"").count(), 2, "{s}");
}

#[test]
fn tai_タグの描述内前置() {
    let s = parse_ok("lo tai senva tcima cu barda");
    assert!(s.contains("BAI_core \"tai\""), "{s}");
    assert!(s.contains("BRIVLA_core \"senva\""), "{s}");
    assert!(s.contains("BRIVLA_core \"tcima\""), "{s}");
}

#[test]
fn tai_タグの述語前置と項接続() {
    // selbri 前置のモダンタグ(受容の確認)
    let s = parse_ok("mi tai sutra cadzu");
    assert!(s.contains("BAI_core \"tai\""), "{s}");
    // タグ + sumti 形(受容の確認)
    let s = parse_ok("mi cadzu tai lo xanri");
    assert!(s.contains("BAI_core \"tai\""), "{s}");
}

#[test]
fn タグの_nai_結合() {
    let s = parse_ok("mi tai nai sutra cadzu");
    assert!(s.contains("BAI_core \"tai\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
}

#[test]
fn tahi_と_tai_のトークン区別() {
    // ta'i(方法)は BAI。アポストロフィ有無で tai(様態)と別トークン
    let s = parse_ok("mi ta'i klama");
    assert!(s.contains("BAI_core \"ta'i\""), "{s}");
    assert!(!s.contains("BAI_core \"tai\""), "{s}");
    // 対比: 時制の ta'e は TAhE(ta'i は TAhE ではない)
    let s = parse_ok("mi ta'e klama");
    assert!(s.contains("TAhE_core \"ta'e\""), "{s}");
}

#[test]
fn qqq_は引き続き拒否される() {
    // 語形として不正な語は従来どおりエラー(tai 追加の影響を受けない)
    assert!(LojbanParser::parse(Rule::text, "qqq").is_err());
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
fn 描述内の埋め込みsumti() {
    // 描述内で selbri の前に埋め込み sumti を許容(所有形 lo mi gerku /
    // lo di'u valsi 等。zantufa の
    // sumti_tail <- relative_clauses? (!quantifier sumti)? sumti_tail_1
    // の埋め込みスロット相当)。mex 枝を先にすることで数詞描述は既存の
    // mex 経路に誘導され木は不変
    for input in [
        "lo di'u valsi cu barda",
        "lo mi gerku cu barda",
        "lo do zdani cu barda",
        "lo mi klama cu barda",
        // KU 明示閉鎖との相互作用(新規受容)
        "lo mi gerku ku cu barda",
        // 入れ子描述(内側を ku で閉じれば外側 selbri が成立)
        "lo lo nanmu ku gerku cu barda",
        // 相対節は desc 全体に付く(埋め込み sumti の内側ではない)
        "lo mi gerku poi barda cu cadzu",
    ] {
        let s = parse_ok(input);
        assert!(s.contains("desc"), "{s}");
    }
    // 埋め込み sumti が desc 内の sumti として現れる
    let s = parse_ok("lo di'u valsi cu barda");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"valsi\""), "{s}");
    let s = parse_ok("lo mi gerku cu barda");
    assert!(s.contains("KOhA_core \"mi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
    // 相対節は desc の外(sumti レベル)に付く
    let s = parse_ok("lo mi gerku poi barda cu cadzu");
    assert!(s.contains("relative_clauses"), "{s}");
    // 数詞描述は従来どおり mex 経路(木形状不変)
    let s = parse_ok("lo pa mlatu cu barda");
    assert!(s.contains("mex"), "{s}");
    assert!(!s.contains("(sumti (number"), "{s}");
    // 数詞起源の埋め込み sumti は zantufa の !quantifier 先読み相当の
    // !mex ガードで拒否。2系統ある:
    // ・zantufa も拒否(本実装と一致): 数詞+相対節 / 数詞+VUhO
    // ・zantufa は sumti_tail_1 の quantifier sumti 枝で受理するが、
    //   本実装は既知差異として拒否: 数詞+所有形 /
    //   尾部形の表面形 lo pa le gerku ku(語順 pa le gerku ku のみ
    //   quant_desc で受理済み)
    for input in [
        "lo pa poi gerku barda cu cadzu",
        "lo pa vu'o mi gerku cu barda",
        "lo pa mi gerku",
        "lo pa le gerku ku",
    ] {
        assert!(LojbanParser::parse(Rule::text, input).is_err(), "{input}");
    }
    // 数詞+JOI 項接続(lo pa joi re gerku)は v0.100 まで既知差異として
    // 拒否していたが、v0.101 から mex 演算子の接続詞枝(mex_conn)により
    // 「pa joi re」が mex(中置接続詞演算子)として解析され、
    // sumti_tail の mex 枝経由で受理される(z0 も受理。実測)。
    // 木は z0 の sumti_tail_1 quantifier sumti 枝ではなく mex 枝だが、
    // 受理は z0 と一致(既知差異の解消)
    let s = parse_ok("lo pa joi re gerku cu barda");
    assert!(s.contains("desc"), "{s}");
    assert!(s.contains("JOI_core \"joi\""), "{s}");
    assert!(
        s.contains("PA_core \"pa\"") && s.contains("PA_core \"re\""),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
    // 埋め込み sumti 単独では描述が閉じない(zantufa z0/z1 と一致)
    assert!(LojbanParser::parse(Rule::text, "lo di'u cu barda").is_err());
    // 入れ子描述は ku なしでは内側 tanru が貪欲吸収され zantufa とともに拒否
    assert!(LojbanParser::parse(Rule::text, "lo lo nanmu gerku cu barda").is_err());
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 描述内埋め込みsumtiを含む対象文() {
    // 描述内の selbri 前埋め込み sumti(di'u)を含む抽象+BE 連結の全体文
    let s = parse_ok("ni'o ca lo nu .abu cusku lo di'u valsi kei lo jamfu be .abu cu sakli .i");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("PU_core \"ca\""), "{s}");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
    assert!(s.contains("KEI_core \"kei\""), "{s}");
    assert!(s.contains("BRIVLA_core \"jamfu\""), "{s}");
    assert!(s.contains("BE_core \"be\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sakli\""), "{s}");
    assert!(s.contains("I_core \"i\""), "{s}");
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
    // lu'e 系の明示閉鎖 lu'u(LUhU)
    let s = parse_ok("lu'e le cukta lu'u");
    assert!(s.contains("LAhE_core \"lu'e\""), "{s}");
    assert!(s.contains("LUhU_core \"lu'u\""), "{s}");
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
fn タグ直後の_bo_短スコープ結合() {
    // zantufa の tag? BO_clause 相当。tense_mark がタグの直後の BO を
    // 一様に消費する(v0.95)。文頭の .i + タグ + BO の実文
    let s = parse_ok(
        ".i ni'i bo lo nunfarlu temci cu mutce banzu \
         lo nu catlu lo sruri gi'e kucli lo du'u ma kau ba zi fasnu",
    );
    // タグ(ni'i)と BO が sentence 直下に並ぶ(tense_mark は silent)
    assert!(s.contains("BAI_clause (BAI_core \"ni'i\")"), "{s}");
    assert!(s.contains("BO_clause (BO_core \"bo\")"), "{s}");
    assert!(s.contains("LE_core \"lo\""), "{s}");
    assert!(s.contains("BRIVLA_core \"nunfarlu\""), "{s}");
    assert!(s.contains("NU_core \"du'u\""), "{s}");
    // BO は文の本体より先に消費される(タグ+BO の短スコープ結合)
    let bo = s.find("BO_clause (BO_core \"bo\")").unwrap();
    let body = s.find("terms_full").unwrap();
    assert!(bo < body, "{s}");
}

#[test]
fn タグ_bo_の各位置() {
    // 文頭タグ+BO(lead + sentence の tense_marks 経路)
    parse_ok("ni'i bo mi klama");
    parse_ok("pu bo mi klama");
    parse_ok(".i ki'u bo mi klama");
    parse_ok("ni'o ki'u bo mi klama");
    parse_ok(".i ni'i bo mi klama");
    // 項に続くタグ+BO(zantufa の sumti_2 / term_1 ループ相当)
    let s = parse_ok("mi ni'i bo do klama");
    assert!(
        s.contains("BAI_core \"ni'i\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
    // selbri 後の tail_terms 内(zantufa の tag_term + BO 相当)
    parse_ok("mi viska pu bo lo mlatu");
    // selbri 前タグ+BO(selbri の tense_marks 経路)
    parse_ok("mi pu bo klama");
    // 連鎖途中の BO(v0.95)。zantufa は連鎖単位で1回だが本実装は各要素の
    // 直後に BO を許可する意図的緩和(「ca bo ba」は z1 が拒否、z0 は
    // tag_term+BO の2項として受理。「pu bo ca bo」は z0/z1 とも拒否)
    parse_ok("ca bo ba");
    parse_ok("pu bo ca bo");
    // BAI+NAI+BO(z0/z1 は拒否する意図的緩和)
    let s = parse_ok("mu'i nai bo");
    assert!(
        s.contains("NAI_core \"nai\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
}

#[test]
fn 既存_bo_経路は維持される() {
    // tanru の JA+BO 接続(tanru_link 経路。本件で変更していない)
    let s = parse_ok("mi klama je bo cadzu");
    assert!(
        s.contains("JA_core \"je\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
    // タグなし .i bo(既存 sep 経路)
    let s = parse_ok(".i bo mi klama");
    assert!(s.contains("BO_clause"), "{s}");
    parse_ok("mi klama .i bo mi klama");
    parse_ok("mi joi bo do klama");
    parse_ok("ta melbi je bo cmalu zdani");
    // BO を含まないタグの解析木は不変
    parse_ok("ni'i mi klama");
    parse_ok(".i ni'i mi klama");
    // 裸の tanru BO 接続(zantufa selbri_6 相当)は v0.109 で受理
    // (z0 実測: selbri_6 = tanru_unit (BO tanru_unit)* の tanru 接続。
    // 詳細は 裸tanru_bo接続_v0_109 を参照)
    let s = parse_ok("mi klama bo cadzu");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))) \
             (BO_clause (BO_core \"bo\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"cadzu\"))))"
        ),
        "{s}"
    );
    // 非文は拒否
    assert!(lojban::parse("qqq").is_err());
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
fn 発話序数_分離形_数詞_mai() {
    // 分離形「数詞+mai」は自由修飾語(zantufa free <- mex_2 MAI_clause の
    // number サブセット)。v0.96 まで裸の mai 自体が MAI_core に無く
    // 「pa mai …」全体が拒否されていた回帰
    let s = parse_ok("pa mai");
    assert!(
        s.contains(
            "mai_free (number (PA_clause (PA_core \"pa\"))) (MAI_clause (MAI_core \"mai\"))"
        ),
        "{s}"
    );
    // 文頭の自由修飾語として
    let s = parse_ok("pa mai mi klama");
    assert!(s.contains("MAI_core \"mai\""), "{s}");
    // 融合数詞+mai(PA_seq 経路)
    parse_ok("pare mai mi klama");
    // 文中の項の後ろ(旧文法は裸 mai を語彙に持たず入力ごと拒否。
    // 隣接する数詞+mai は free に一括される。sumti+free の2要素に
    // 分かれるのは融合形 pa+pamai のケースで、こちらも free 一括に
    // なるため木が変わる — 後述の pa pamai ピンを参照)
    let s = parse_ok("mi viska pa mai");
    assert!(s.contains("mai_free (number"), "{s}");
    // 項と mai の間に別の free が挟まれば数詞は項のまま(隣接時のみ一括)
    let s = parse_ok("mi viska pa .ui mai");
    assert!(
        s.contains("sumti (number") && s.contains("MAI_core \"mai\""),
        "{s}"
    );
    // number の BOI 経路
    let s = parse_ok("pa boi mai");
    assert!(
        s.contains("BOI_core \"boi\"") && s.contains("MAI_core \"mai\""),
        "{s}"
    );
    // 疑問数詞 xo も number 経路で受理
    let s = parse_ok("xo mai");
    assert!(
        s.contains("PA_core \"xo\"") && s.contains("MAI_core \"mai\""),
        "{s}"
    );
    // 裸の mo'o も free として受理
    let s = parse_ok("mo'o");
    assert!(
        s.contains("mai_free (MAI_clause (MAI_core \"mo'o\"))"),
        "{s}"
    );
    // 裸の mai/mo'o 単独の free は意図的緩和(zantufa は mex_2 前置を
    // 要求するため拒否。第一枝 MAI_clause の帰結)
    parse_ok("mi klama mai");
    parse_ok("mi klama mo'o");
    // 段落序数 mo'o(selma'o MAI のもう1語)
    let s = parse_ok("pa mo'o mi klama");
    assert!(s.contains("MAI_core \"mo'o\""), "{s}");
    // 数詞項の後ろに置いた場合も free として受理
    parse_ok("mi pa mai klama");
    // 融合形は従来どおり
    let s = parse_ok("pamai");
    assert!(s.contains("MAI_core \"pamai\""), "{s}");
    // 融合形は pamai〜nomai のみ(MAI_core 固定)。"paremai" は MAI と
    // しては解釈されず、lujvo 形態として BRIVLA 経路で受理される
    // (v0.95 から不変。zantufa も数詞+mai の無空白連結として受理)
    let s = parse_ok("paremai");
    assert!(!s.contains("MAI_core"), "{s}");
    assert!(s.contains("BRIVLA_core \"paremai\""), "{s}");
    // brivla 解析の帰結として、本実装は後続の bridi とは繋がらない
    assert!(lojban::parse("paremai mi klama").is_err());
    // 旧受容入力で木が変わる融合形パターン: 旧解析は sumti(pa)+
    // free(pamai) の2要素だったが、隣接する数詞+MAI は free に
    // 一括される(受容は同じで木のみ変化)
    let s = parse_ok("mi viska pa pamai");
    assert!(
        s.contains(
            "mai_free (number (PA_clause (PA_core \"pa\"))) (MAI_clause (MAI_core \"pamai\"))"
        ),
        "{s}"
    );
    // 対象文(発話序数を含む全文)
    let s = parse_ok("pa mai .abu troci lo nu catlu lo cnita gi'e facki lo du'u .abu ma kau klama");
    assert!(s.contains("MAI_core \"mai\""), "{s}");
    // 非文は拒否
    assert!(lojban::parse("qqq").is_err());
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

#[test]
fn 対象文_bahe強調と_lahe参照() {
    // ni'o + la'e di'u + na'e + ba'e mutce(tanru 単位への強調前置)+ lo ka cizra
    let s = parse_ok("ni'o la'e di'u na'e ba'e mutce lo ka cizra");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("LAhE_core \"la'e\""), "{s}");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
    assert!(s.contains("NAhE_core \"na'e\""), "{s}");
    // ba'e は tanru_unit の前置(BAhE_clause ノード)
    assert!(
        s.contains("tanru_unit (BAhE_clause (BAhE_core \"ba'e\")"),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"mutce\""), "{s}");
    assert!(s.contains("NU_core \"ka\""), "{s}");
    // la'e の終端に ge'u は現れない(LUhU 専用)
    assert!(!s.contains("GEhU_core"), "{s}");
}

#[test]
fn bahe_tanru単位への前置() {
    // s_mark(na'e)と tanru 単位の間
    let s = parse_ok("mi na'e ba'e mutce lo ka cizra");
    assert!(s.contains("NAhE_core \"na'e\""), "{s}");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
    assert!(s.contains("BRIVLA_core \"mutce\""), "{s}");
    // tanru 第2単位への前置
    let s = parse_ok("mutce ba'e nandu");
    assert!(s.contains("BRIVLA_core \"mutce\""), "{s}");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
    assert!(s.contains("BRIVLA_core \"nandu\""), "{s}");
    // ba'e + 非 BRIVLA 単位(GOhA)
    let s = parse_ok("na'e ba'e go'i");
    assert!(s.contains("NAhE_core \"na'e\""), "{s}");
    assert!(
        s.contains("tanru_unit (BAhE_clause (BAhE_core \"ba'e\")"),
        "{s}"
    );
    assert!(s.contains("GOhA_core \"go'i\""), "{s}");
    // 連続の ba'e(free 経由で2語とも受理)
    let s = parse_ok("ba'e ba'e klama");
    assert_eq!(s.matches("BAhE_core").count(), 2, "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 強調なしの形は従来どおり(ba'e ノードなし)
    let s = parse_ok("mi na'e mutce");
    assert!(!s.contains("BAhE_core"), "{s}");
}

#[test]
fn zae_前借り語() {
    // za'e は BAhE 同 selma'o(free 経路)
    let s = parse_ok("mi za'e klama");
    assert!(s.contains("BAhE_core \"za'e\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 文頭の単独形も free(frees_s)経由の受理
    let s = parse_ok("za'e klama");
    assert!(s.contains("BAhE_core \"za'e\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // tanru 単位の先頭(tanru_unit 経路)でも受理
    let s = parse_ok("mutce za'e nandu");
    assert!(
        s.contains("tanru_unit (BAhE_clause (BAhE_core \"za'e\")"),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"nandu\""), "{s}");
}

#[test]
fn lahe_明示閉鎖_luu() {
    // LAhE の終端は lu'u(LUhU)。ge'u は受けない(CLL 6.7 / zantufa 準拠)
    let s = parse_ok("la'e di'u lu'u cu mutce");
    assert!(s.contains("LAhE_core \"la'e\""), "{s}");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
    assert!(s.contains("LUhU_core \"lu'u\""), "{s}");
    assert!(s.contains("CU_core \"cu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"mutce\""), "{s}");
    // h 表記(luhu)も同様
    let s = parse_ok("la'e di'u luhu cu mutce");
    assert!(s.contains("LUhU_core \"luhu\""), "{s}");
    // ge'u(GOI 終端)では閉鎖できない
    assert!(lojban::parse("la'e di'u ge'u cu mutce").is_err());
}

#[test]
fn lu引用の閉鎖ノードは_lihu() {
    // lu_quote の閉鎖は LIhU_clause(li'u)。LUhU は本来の語形 lu'u を持つ
    let s = parse_ok("lu mi klama li'u");
    assert!(s.contains("LIhU_clause (LIhU_core \"li'u\")"), "{s}");
    assert!(!s.contains("LUhU_core"), "{s}");
    let s = parse_ok("lu mi klama lihu");
    assert!(s.contains("LIhU_core \"lihu\""), "{s}");
}

#[test]
fn bae_free経路は従来どおり() {
    // 項後の自由修飾語としての ba'e(free 経路)は壊れない
    let s = parse_ok("mi ta'a ba'e do");
    assert!(s.contains("COI_core \"ta'a\""), "{s}");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
    assert!(s.contains("KOhA_core \"do\""), "{s}");
    // gihek_link 直後の ba'e(v0.92): PEG 順序選択により tanru_unit 前置ではなく
    // free 経路(bae_free)で先取りされる。暗黙仕様を意図的仕様として記録
    let s = parse_ok("mi klama gi'e ba'e cadzu");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(
        s.contains("bae_free (BAhE_clause (BAhE_core \"ba'e\"))"),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"cadzu\""), "{s}");
}

#[test]
fn 抽象_sio() {
    // si'o(概念抽象)は NU_core の語彙(v0.91 で追加。nu/ka と同じ抽象経路)。
    // 大文字形も受理(^case-insensitive の回帰ガード)
    let s = parse_ok("lo SI'O ri viska cu nandu");
    assert!(s.contains("NU_core \"SI'O\""), "{s}");
    // 項位置の抽象は tanru_unit 内の nu_form ノード
    let s = parse_ok("lo si'o ri viska cu nandu");
    assert!(s.contains("NU_core \"si'o\""), "{s}");
    assert!(s.contains("nu_form"), "{s}");
    // 項の代名詞が来る形も同様
    let s = parse_ok("lo si'o mi viska cu nandu");
    assert!(s.contains("NU_core \"si'o\""), "{s}");
    assert!(s.contains("nu_form"), "{s}");
    // kei による明示終端も受理
    let s = parse_ok("lo si'o do viska kei cu nandu");
    assert!(s.contains("NU_core \"si'o\""), "{s}");
    assert!(s.contains("KEI_clause"), "{s}");
    // 別形: 抽象節を selbri 側に置くと abstraction ノード
    let s = parse_ok("mi si'o do tavla cu nelci");
    assert!(s.contains("NU_core \"si'o\""), "{s}");
    assert!(s.contains("abstraction"), "{s}");
    // 対比: 従来語彙(nu / ka)は影響を受けない
    for w in ["nu", "ka"] {
        let s = parse_ok(&format!("lo {w} ri viska cu nandu"));
        assert!(s.contains(&format!("NU_core \"{w}\"")), "{s}");
    }
}

#[test]
fn 対象文_概念抽象sioを含む実文() {
    // zantufa が受理する実文(終端に半角スペース)。fa lo si'o … の
    // si'o が NU_core として解析されることを含め全体が受理される
    let s = parse_ok(".i ku'i la .alis. ca lo nu la ractu ca'a lebna lo junla lo kosta daski gi'e catlu jy gi'e di'a sutra cu spaji sa'irbi'o ki'u lo nu lindi pagre lo menli be .abu fa lo si'o ri pu no roi viska lo ractu poi ponse lo kosta daski .a lo junla poi ry ke'a dy ka'e lebna ");
    assert!(s.contains("I_core \"i\""), "{s}");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(s.contains("CAhA_core \"ca'a\""), "{s}");
    assert!(s.contains("FA_core \"fa\""), "{s}");
    assert!(s.contains("NU_core \"si'o\""), "{s}");
    assert!(
        s.contains("PU_core \"pu\"") && s.contains("ROI_core \"roi\""),
        "{s}"
    );
    assert!(s.contains("NOI_core \"poi\""), "{s}");
    assert!(s.contains("BY_core \"jy\""), "{s}");
    assert!(s.contains("BY_core \"ry\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sa'irbi'o\""), "{s}");
}

#[test]
fn 述語連鎖直後の感情標識() {
    // gihek_link の直後に自由修飾語(v0.92)。zantufa の (GIhA:gi'e UI:u'a) 相当
    let s = parse_ok("mi klama gi'e .u'a cadzu");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(
        s.contains("UI_core \".u'a\"") || s.contains("UI_core \"u'a\""),
        "{s}"
    );
    // 後続の述語は連鎖側の bridi_tail として解析される
    assert!(s.contains("BRIVLA_core \"cadzu\""), "{s}");
    // 3連鎖: 真ん中に UI なし・末尾に UI 付き
    let s = parse_ok("mi klama gi'e cadzu gi'e .u'a bajra");
    assert_eq!(s.matches("GIhA_core").count(), 2, "{s}");
    assert!(s.contains("UI_core \"u'a\""), "{s}");
    assert!(s.contains("BRIVLA_core \"bajra\""), "{s}");
    // NAI 否定形(ui_free の既存経路で受理)
    let s = parse_ok("mi klama gi'e .u'a nai cadzu");
    assert!(s.contains("UI_core \"u'a\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    // CAI 強度も同じ位置で受理
    let s = parse_ok("mi klama gi'e u'a sai cadzu");
    assert!(s.contains("CAI_core \"sai\""), "{s}");
    // 語境界ガードの回帰固定: free* 挿入により s_mark / BRIVLA 先頭の
    // 述語が free(SEI/SI/SU 等)に先取りされない
    let s = parse_ok("mi klama gi'e se prami do");
    assert!(s.contains("SE_core \"se\""), "{s}");
    assert!(s.contains("BRIVLA_core \"prami\""), "{s}");
    let s = parse_ok("mi klama gi'e sutra");
    assert!(s.contains("BRIVLA_core \"sutra\""), "{s}");
    // 連鎖先のない形は従来どおり拒否
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e .u'a").is_err());
    // 語形として不正な語は従来どおりエラー(否定系維持)
    assert!(LojbanParser::parse(Rule::text, "qqq").is_err());
}

#[test]
fn 対象文_gihe直後の感情標識を含む実文() {
    // zantufa が受理する実文。2箇所の gi'e 連鎖のうち後半が gi'e .u'a 形。
    // KE グループ / MOhI+FAhA / 関係節 / lujvo を含む
    let s = parse_ok(".i .abu bai lo nu kucli cu bajra pagre lo foldi gi'e jersi ry gi'e .u'a viska lo nu ry canci mo'i ne'i lo barda ke ractu kevna noi cnita lo spabi'u");
    assert_eq!(s.matches("GIhA_core \"gi'e\"").count(), 2, "{s}");
    assert!(s.contains("UI_core \"u'a\""), "{s}");
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(s.contains("BAI_core \"bai\""), "{s}");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("MOhI_core \"mo'i\""), "{s}");
    assert!(s.contains("FAhA_core \"ne'i\""), "{s}");
    assert!(s.contains("KE_core \"ke\""), "{s}");
    assert!(s.contains("NOI_core \"noi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"spabi'u\""), "{s}");
}

#[test]
fn kau_間接疑問マーカー() {
    // kau は UI セルマォ(CLL 19.10 / zantufa ツリーでも UI:kau)。
    // free 経路で受理される(v0.93 で UI_core に追加)
    let s = parse_ok("mi kau klama");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    assert!(s.contains("free"), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 項間・抽象内でも同じ経路
    parse_ok("mi kau tavla do");
    parse_ok("ma kau .abu bartu");
    let s = parse_ok("mi djuno lo du'u ma kau klama");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    // h 表記 kahu(UI_core 既存)と共存する
    parse_ok("mi kahu klama");
    // NAI 否定・CAI 強度の修飾形(ui_free の既存経路)
    let s = parse_ok("kau nai");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    let s = parse_ok("kau sai");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    assert!(s.contains("CAI_core \"sai\""), "{s}");
}

#[test]
fn rehu_序数頻度タグ() {
    // re'u は ROI セルマォ(roi + re'u。v0.93 で追加)。h 表記 rehu も同様
    // v0.99: quant_selbri ガードにより fragment(数詞+selbri の項)ではなく
    // 文のタグ読み(selbri の tense_marks 経路)で受理される。
    // interval_property の PA_seq 枝は atomic のため PA_core 子ノードは出ない
    let s = parse_ok("mi za'u re'u klama");
    assert!(s.contains("sentence"), "{s}");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    let s = parse_ok("so'i rehu klama");
    assert!(s.contains("ROI_core \"rehu\""), "{s}");
    // 数詞を挟む/連鎖する形(so'u roi 同様の複合タグ)
    for w in ["za'u ro re'u", "pa za'u re'u", "so'i re'u", "su'o re'u"] {
        let s = parse_ok(w);
        assert!(s.contains("ROI_core"), "{s}");
    }
    let s = parse_ok("za'u ro rehu");
    assert!(s.contains("ROI_core \"rehu\""), "{s}");
    // NAI 否定形(interval_property の (sp1 ~ NAI_clause)? 経由)
    let s = parse_ok("so'i re'u nai klama");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    // 裸 ROI の項後用法。z1 のみ整合の受容(z0/camxes は拒否)だが、
    // 既存の「mi roi klama」と同じ selbri 頭時制タグ経路で一貫
    let s = parse_ok("mi re'u klama");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    // interval_property 分岐書き換え(v0.93)の等価性ピン:
    // 無空白数詞連鎖(PA_seq)+ ROI の形(branch1)が従来どおり生きている
    let s = parse_ok("reso'i roi klama");
    assert!(s.contains("PA_seq \"reso'i\""), "{s}");
    assert!(s.contains("ROI_core \"roi\""), "{s}");
    let s = parse_ok("reso'i re'u klama");
    assert!(s.contains("PA_seq \"reso'i\""), "{s}");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    // 無空白結合形(so'uroi / zahurehu)は PA_seq ではなく音韻上の
    // brivla(fuhivla 形)として受理される(PA_word に ROI/TAhE 語は
    // 含まれないため)。v0.92 から不変の既存挙動
    let s = parse_ok("so'uroi klama");
    assert!(s.contains("BRIVLA_core \"so'uroi\""), "{s}");
    let s = parse_ok("zahurehu");
    assert!(s.contains("BRIVLA_core \"zahurehu\""), "{s}");
    // li 内の数詞(mex)は bare_number ガードの影響を受けない
    let s = parse_ok("li re no");
    assert!(s.contains("LI_core \"li\""), "{s}");
    assert!(s.contains("PA_core \"re\""), "{s}");
    assert!(s.contains("PA_core \"no\""), "{s}");
}

#[test]
fn 時制kuに描述が続く形は_fragment_として受理() {
    // [時制+明示ku] + 描述(+cu 無し)。zantufa は全文ではなく
    // fragment(terms: tag_term + sumti)として受理する。
    // tagged の「タグだけ項」枝(tense_tags ~ sp1 ~ KU_clause)により同経路を再現。
    // 描述の tanru が後続の brivla を吸収する点も zantufa と同じ
    let s = parse_ok("ba zi ku le gerku klama");
    assert!(s.contains("fragment"), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("LE_core \"le\""), "{s}");
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // cmevla 描述でも同様
    let s = parse_ok("ba zi ku la .alis. klama");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("CMEVLA_clause"), "{s}");
    // 対比: 主語が先にある場合は従来どおり文として受理
    // (tense_marks が「pu zi ku」まで消費し bridi_tail が続く)
    let s = parse_ok("mi pu zi ku klama");
    assert!(s.contains("PU_core \"pu\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 裸の ku だけの項は引き続き拒否(tagged は tense_tags 前置が必須)
    assert!(LojbanParser::parse(Rule::text, "mi ku klama").is_err());
    // タグだけ項枝 + BO(v0.95 の tense_mark BO 後置による副次的受容)。
    // BO を伴うタグだけ項も fragment(tagged) で受理する。
    // zantufa(z0/z1)は拒否する意図的緩和
    let s = parse_ok("pu bo ku");
    assert!(s.contains("fragment"), "{s}");
    assert!(s.contains("tagged"), "{s}");
    assert!(
        s.contains("BO_core \"bo\"") && s.contains("KU_core \"ku\""),
        "{s}"
    );
}

#[test]
fn タグと項の間の自由修飾語() {
    // tagged の時制枝はタグ〜sumti 間の自由修飾語を許容する
    // (ta'i ba'e ma …。zantufa は sumti 先頭語への BAhE 付帯として受理)
    let s = parse_ok("mi ta'i ba'e ma .abu bartu");
    assert!(s.contains("BAI_core \"ta'i\""), "{s}");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
    assert!(s.contains("KOhA_core \"ma\""), "{s}");
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"bartu\""), "{s}");
    // 自由修飾語なしの形は従来どおり
    parse_ok("mi ta'i do klama");
}

#[test]
fn 述語後のタグ付き項の後に裸述語は続かない() {
    // 「selbri + 数詞+TAhE タグ付き項 + 裸 selbri」は接続詞なしの
    // 二重述語となる非文。タグ付き項(re ta'e mi)は tail_terms として
    // 解釈され、後続の brivla に述語の役割が残らない。
    // zantufa(z0/z1)・camxes すべてが拒否することを参照突合済みで、
    // v0.92 も同一挙動のため受理拡張は行わない(lojban.pest tagged 注記)
    assert!(lojban::parse("mi ponse re ta'e mi klama").is_err());
    // 対比: 述語で終わる形 / タグ付き項が主語側にある形は受理される
    // (後者は参照系より緩い既存の受容。v0.92 から不変)
    parse_ok("mi ponse re ta'e mi");
    parse_ok("mi re ta'e mi klama");
}

#[test]
fn 対象文_kau_rehu_時制kuを含む実文() {
    // zantufa(z1 を除くビルド)/camxes が受理する実文。
    // 時制+明示ku / MOhI+FAhA / gi'e 連鎖 / 抽象 / ba'e 強調 /
    // kau(UI) / za'u re'u(数詞+ROI 複合タグ)を含む
    let s = parse_ok("ni'o ba zi ku la .alis. mo'i ne'i jersi ry gi'e no roi pensi lo du'u ta'i ba'e ma kau .abu ba za'u re'u bartu");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("PU_core \"ba\""), "{s}");
    assert!(s.contains("ZI_core \"zi\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("MOhI_core \"mo'i\""), "{s}");
    assert!(s.contains("FAhA_core \"ne'i\""), "{s}");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(
        s.contains("PA_seq \"no\"") && s.contains("ROI_core \"roi\""),
        "{s}"
    );
    assert!(s.contains("NU_core \"du'u\""), "{s}");
    assert!(s.contains("BAI_core \"ta'i\""), "{s}");
    assert!(s.contains("BAhE_core \"ba'e\""), "{s}");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(
        s.contains("PA_seq \"za'u\"") && s.contains("ROI_core \"re'u\""),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"bartu\""), "{s}");
}

#[test]
fn 対象文_裸時制連鎖を含む実文() {
    // zantufa が受理する実文。複数の MOhI+FAhA 連鎖を裸時制フラグメント
    // (tense_item)として受理する。ku は入力に存在しない(暗黙終端)
    let s = parse_ok("ni'o mo'i ni'a mo'i ni'a mo'i ni'a .i xu lo nu farlu cu no roi mulno .i");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    // MOhI+FAhA の組が3回出現する
    assert_eq!(s.matches("MOhI_core \"mo'i\"").count(), 3, "{s}");
    assert_eq!(s.matches("FAhA_core \"ni'a\"").count(), 3, "{s}");
    assert!(s.contains("UI_core \"xu\""), "{s}");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(
        s.contains("PA_seq \"no\"") && s.contains("ROI_core \"roi\""),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"farlu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"mulno\""), "{s}");
    // tense_item ノード配下に3組の入れ子が揃っていることの直接確認
    // (MOhI+FAhA が別々の item に散らばっていないことの回帰ガード)
    assert!(
        s.contains(
            "(tense_item (MOhI_clause (MOhI_core \"mo'i\")) (FAhA_clause (FAhA_core \"ni'a\")) \
             (MOhI_clause (MOhI_core \"mo'i\")) (FAhA_clause (FAhA_core \"ni'a\")) \
             (MOhI_clause (MOhI_core \"mo'i\")) (FAhA_clause (FAhA_core \"ni'a\")))"
        ),
        "{s}"
    );
}

#[test]
fn 裸時制連鎖のフラグメント() {
    // 2組以上の連鎖は tense_item 経由でのみ受理される
    parse_ok("mo'i ni'a mo'i ni'a");
    parse_ok("mo'i ni'a mo'i ni'a mo'i ni'a");
    parse_ok("pu ba");
    // 単一形・ku 付き形は従来どおり
    parse_ok("mo'i ni'a");
    parse_ok("mo'i ni'a ku");
    // ku 付き形は fragment(tagged) 経路が先行する(item 選択のシャドウ挙動ピン。
    // tense_item 側の tense_marks 末尾 KU 枝は実質予備)
    let s = parse_ok("pu ba ku");
    assert!(s.contains("fragment"), "{s}");
    assert!(!s.contains("tense_item"), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    // 連鎖が区切りのポーズを呑まない(前後の .i が両方残る)
    let s = parse_ok(".i pu ba .i");
    assert_eq!(s.matches("I_core \"i\"").count(), 2, "{s}");
    assert!(s.contains("tense_item"), "{s}");
    // 連鎖 + VAU(zantufa の fragment terms VAU_elidible 相当。v0.94 で受容)
    let s = parse_ok("mo'i ni'a vau");
    assert!(s.contains("tense_item"), "{s}");
    assert!(s.contains("VAU_core \"vau\""), "{s}");
    // naku 混在連鎖(zantufa は brigahi + tag_term の2項として受理するため
    // 受容。fragment の na_ku 部分確定を否定先読みで tense_item に譲る)
    let s = parse_ok("naku pu");
    assert!(s.contains("tense_item"), "{s}");
    assert!(s.contains("NAKU_joint \"naku\""), "{s}");
    assert!(s.contains("PU_core \"pu\""), "{s}");
    let s = parse_ok("na ku ba");
    assert!(s.contains("tense_item"), "{s}");
    assert!(
        s.contains("NA_core \"na\"") && s.contains("KU_core \"ku\""),
        "{s}"
    );
    assert!(s.contains("PU_core \"ba\""), "{s}");
    // VAU との組合せも同様
    parse_ok("naku pu vau");
    parse_ok("na ku ba vau");
    // 宙吊りタグ+BO フラグメント(v0.95 の tense_mark BO 後置による副次的受容)。
    // 接続先を持たない短スコープ結合で、zantufa(z0/z1)は拒否する意図的緩和
    // (「naku pu」ピンと同様の受容ピン)
    let s = parse_ok("pu bo");
    assert!(s.contains("tense_item"), "{s}");
    assert!(
        s.contains("PU_core \"pu\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
    let s = parse_ok("ni'i bo");
    assert!(s.contains("tense_item"), "{s}");
    assert!(
        s.contains("BAI_core \"ni'i\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
    let s = parse_ok("naku pu bo");
    assert!(s.contains("tense_item"), "{s}");
    assert!(
        s.contains("NAKU_joint \"naku\"") && s.contains("BO_core \"bo\""),
        "{s}"
    );
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn faha_残り5語の補完() {
    // CLL 10.12 の FAhA セルマォから欠落していた5語(bu'u/du'a/vu'a/ze'o/zo'i)を
    // FAhA_core に追加(v0.98)。変更前はこれらの語は FAhA 位置で完全未受理
    // (「mi jaurjanli bu'u lo lalxu」は bu'u 直後でエラー)。
    // 対象文: selbri 後の FAhA タグ付き項(zantufa の tag_term 相当)
    let s = parse_ok("ni'o ca ku .abu tirna lo nu da va jaurjanli bu'u lo lalxu");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("PU_core \"ca\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"tirna\""), "{s}");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("FAhA_core \"bu'u\""), "{s}");
    assert!(s.contains("LE_core \"lo\""), "{s}");
    assert!(s.contains("BRIVLA_core \"lalxu\""), "{s}");
    // selbri 前タグ(tagged 経路)。描述の tanru が後続の klama を貪欲吸収し、
    // 全体は fragment として受理される(zantufa z0 も同一の木。「ba zi ku
    // le gerku klama」ピンと同じ描述の貪欲吸収)
    let s = parse_ok("mi bu'u lo lalxu klama");
    assert!(s.contains("fragment"), "{s}");
    assert!(s.contains("FAhA_core \"bu'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"lalxu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // selbri 後タグ付き項(tail_terms 経路)
    let s = parse_ok("mi jaurjanli bu'u lo lalxu");
    assert!(s.contains("FAhA_core \"bu'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"jaurjanli\""), "{s}");
    // 残り4語の selbri 前タグ(selbri の tense_marks 経路)
    for w in ["du'a", "ze'o", "zo'i", "vu'a"] {
        let s = parse_ok(&format!("mi {w} klama"));
        assert!(s.contains(&format!("FAhA_core \"{w}\"")), "{s}");
    }
    // h 表記(ze'o のみ zeho。zeoho は誤りで zantufa も拒否することを実測済み)
    let s = parse_ok("mi buhu lo lalxu klama");
    assert!(s.contains("FAhA_core \"buhu\""), "{s}");
    for h in ["duha", "vuha", "zeho", "zohi"] {
        let s = parse_ok(&format!("mi {h} klama"));
        assert!(s.contains(&format!("FAhA_core \"{h}\"")), "{s}");
    }
    // MOhI+FAhA の組合せ
    let s = parse_ok("mi mo'i bu'u klama");
    assert!(s.contains("MOhI_core \"mo'i\""), "{s}");
    assert!(s.contains("FAhA_core \"bu'u\""), "{s}");
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 抽象内の数詞roi複合タグはquant_selbriに貪欲消費されない() {
    // v0.99: 抽象(du'u)内の inner sentence で quant_selbri が
    // 「za'u re'u sudga」を数詞+selbri の項として貪欲消費し、
    // bridi_tail の selbri が残らず拒否される問題を修正
    // (PEG の部分成功確定。v0.93 ギャップC「ba zi ku le gerku klama」と同型)。
    // bare_number と同じ ROI/TAhE/ZAhO 直前ガードを quant_selbri に追加し、
    // 数詞+ROI を複合タグ(interval_property)として selbri の
    // tense_marks 経路に譲る。zantufa z0 の term_2 !tag sumti ガード相当
    let s = parse_ok("mi li'a du'u ma kau za'u re'u sudga");
    assert!(s.contains("NU_core \"du'u\""), "{s}");
    assert!(s.contains("KOhA_core \"ma\""), "{s}");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    // 抽象内の PA+ROI 複合タグ構造(タグ読みのピン)
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    // BAI タグ(ta'i)付き項を含む対象文全体
    let s = parse_ok("ni'o lo pa moi preti cu li'a du'u ta'i ma kau za'u re'u sudga");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("LE_core \"lo\""), "{s}");
    assert!(
        s.contains("PA_core \"pa\"") && s.contains("MOI_core \"moi\""),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"preti\""), "{s}");
    assert!(s.contains("CU_core \"cu\""), "{s}");
    assert!(s.contains("UI_core \"li'a\""), "{s}");
    assert!(s.contains("NU_core \"du'u\""), "{s}");
    assert!(s.contains("BAI_core \"ta'i\""), "{s}");
    assert!(s.contains("UI_core \"kau\""), "{s}");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    // NAI 付きは v0.98 以前から受理(quant_selbri の selbri が nai を
    // 取り込めずタグ読みにフォールバックする経路)。挙動不変
    let s = parse_ok("mi li'a du'u ma kau za'u re'u nai sudga");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("NAI_core \"nai\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    // ガードの TAhE/ZAhO 枝のピン: 数詞+TAhE/ZAhO も複合タグ
    // (interval_property)としてタグ読みになる。
    // 注: z0 は ta'e を BAI 扱いにして数詞+TAhE 複合タグを持たないため
    // 「za'u ta'e sudga」を量化詞+selbri の項として読む既知差分の領域。
    // 当社は CLL 規範の interval_property 読みを優先する(v0.99 実装報告の
    // エッジ分類を参照)
    let s = parse_ok("mi li'a du'u ma kau za'u ta'e sudga");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("TAhE_core \"ta'e\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    let s = parse_ok("mi li'a du'u ma kau za'u pu'o sudga");
    assert!(s.contains("PA_seq \"za'u\""), "{s}");
    assert!(s.contains("ZAhO_core \"pu'o\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    // 既存維持: 数詞+MOI はガード外(tanru_unit の number+MOI 経路)
    parse_ok("lo re moi prenu cu barda");
    parse_ok("mi viska re moi prenu");
    // 既存維持: 数詞+ROI の単文。v0.98 は fragment(quant_selbri 項)経路
    // だったが、v0.99 からは z0 と同じ文のタグ読みで受理(木は変化)
    parse_ok("mi za'u re'u klama");
    // 既存維持: 直後が ROI/TAhE/ZAhO でない数詞+selbri は従来どおり
    // quant_selbri の項(ガードの非過剰遮断ピン)
    let s = parse_ok("pa prenu cu klama");
    assert!(s.contains("quant_selbri"), "{s}");
    // 数詞連鎖+selbri も quant_selbri 維持(連鎖の途中で遮断されない)
    let s = parse_ok("pa re prenu cu klama");
    assert!(s.contains("quant_selbri"), "{s}");
    assert!(
        s.contains("PA_core \"pa\"") && s.contains("PA_core \"re\""),
        "{s}"
    );
    // KU 後置形も quant_selbri 維持
    let s = parse_ok("pa prenu ku");
    assert!(s.contains("quant_selbri"), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    // 抽象内の空白区切り数詞連鎖+ROI も複合タグとして受理される。
    // 空白区切りのため interval_property の PA_clause 連鎖枝が選ばれ
    // (PA_seq 枝は無空白連結のみ)、PA_clause が並ぶ
    let s = parse_ok("mi li'a du'u ma kau za'u ro re'u sudga");
    assert!(s.contains("NU_core \"du'u\""), "{s}");
    assert!(
        s.contains("PA_core \"za'u\"") && s.contains("PA_core \"ro\""),
        "{s}"
    );
    assert!(s.contains("ROI_core \"re'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"sudga\""), "{s}");
    // z0 も拒否する形は引き続き拒否(数詞+ROI タグの後には selbri が
    // 続かず、裸の数詞+selbri 項もタグ読みに譲るため)
    assert!(lojban::parse("mi li'a du'u ma kau za'u sudga").is_err());
    // 数詞+ROI タグの直後の selbri を欠く形も拒否(z0 整合)
    assert!(lojban::parse("mi za'u re'u klama ku").is_err());
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 裸prenexと描述を項に取る前置スコープ() {
    // v0.100: prenex_sentence の2つのギャップを解消(zantufa z0/z1 準拠)。
    // ① prenex_term の選言の末尾に sumti を追加し、完全な sumti
    //   (描述 lo di'u preti 等)も前置スコープの項に取れる。
    //   単純形(PA_seq/PA_clause/KOhA_clause)を先に試すため既存の
    //   木形状は不変(「su'o da」は PA_seq+KOhA_clause の2項のまま)
    // ② zo'u 後の inner_sentence を任意化し、zo'u で閉じる裸 prenex
    //   (zo'u 後の bridi を省略したトピック風の形)を受理
    // 動機: 実文「ni'o lo di'u preti zo'u」(前述の文は質問だ)が
    //   エラーになった
    // 対象文: 裸 prenex。描述(lo di'u preti)が prenex の項となり
    //   zo'u で閉じる。desc(LE_clause)内の埋め込み sumti として
    //   KOhA_core "di'u"、sumti_tail 経由の selbri "preti" を持つ
    let s = parse_ok("ni'o lo di'u preti zo'u");
    assert!(s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("NIhO_core \"ni'o\""), "{s}");
    assert!(s.contains("desc"), "{s}");
    assert!(s.contains("LE_core \"lo\""), "{s}");
    assert!(s.contains("KOhA_core \"di'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"preti\""), "{s}");
    assert!(s.contains("ZOhU_core \"zo'u\""), "{s}");
    // prenex + bridi の形(zo'u 後に文が続く)
    let s = parse_ok("lo di'u preti zo'u mi jinvi");
    assert!(s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("ZOhU_core \"zo'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"jinvi\""), "{s}");
    // 所有形の描述を項に取る prenex + bridi
    let s = parse_ok("lo mi gerku zo'u mi klama");
    assert!(s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("KOhA_core \"mi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // ku 付きの描述
    let s = parse_ok("le nanmu ku zo'u mi klama");
    assert!(s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("LE_core \"le\""), "{s}");
    assert!(s.contains("KU_core \"ku\""), "{s}");
    assert!(s.contains("BRIVLA_core \"nanmu\""), "{s}");
    // 固有名詞の sumti
    let s = parse_ok("la alis. zo'u mi klama");
    assert!(s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("CMEVLA_core \"alis\""), "{s}");
    // 裸 prenex(zo'u で閉じる)。bridi を省略したトピック風の形
    for text in ["lo preti zo'u", "mi zo'u", "su'o da zo'u"] {
        let s = parse_ok(text);
        assert!(s.contains("prenex_sentence"), "{s}");
        assert!(s.contains("ZOhU_core \"zo'u\""), "{s}");
    }
    // 既存木不変ピン: 単純形の項は sumti で包まれず、prenex_sentence の
    // 直接の子であること(選言の順序により PA_seq/KOhA_clause が優先)。
    // sexpr 上で prenex_sentence の直後に PA_seq/KOhA_clause が並つことを
    // 部分一致で確認する
    let s = parse_ok("su'o da zo'u da klama");
    assert!(
        s.contains(
            "(prenex_sentence (PA_seq \"su'o\") (KOhA_clause (KOhA_core \"da\")) (ZOhU_clause"
        ),
        "{s}"
    );
    let s = parse_ok("mi zo'u mi klama");
    assert!(
        s.contains("(prenex_sentence (KOhA_clause (KOhA_core \"mi\")) (ZOhU_clause"),
        "{s}"
    );
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn prenex_sentence先試行順序交換の例外と境界() {
    // v0.100 レビュー対応: item/inner_sentence の sentence 先試行順序交換の
    // 挙動を固定するピンと、境界・入れ子文脈のカバレッジ
    // (文法本体は変更せず、実測の挙動をアサートする)
    // 例外(zeicompound 経路): zei_compound は word 経由(CMAVO_clause
    // フォールバック)で cmavo「zo'u」を吸収でき、!ZOhU_clause ガードは
    // tanru_unit の BRIVLA 枝にしか掛かっていない。そのため頂層 zo'u の
    // 直後に zei が続く形は sentence が先に一致し、旧順序と木が変わる
    // 形がある。zeicompound 側へのガード追加は単独「zo'u zei broda」の
    // 既存受理を損なうため行わない(実測: 旧順序(v0.99)では
    // mi zo'u zei broda は sentence 木、mi zo'u zei zei broda は
    // prenex 木だった)
    // mi zo'u zei broda: 旧順序(v0.99)でも prenex 必須内文の失敗により
    // sentence の zei_compound(「zo'u zei broda」を selbri として吸収)
    // で既に受理・同木。拒否となるのは内文任意化+旧順序の組合せの
    // バリアント固有で、リリース比較(v0.99→v0.100)では受理拡張ではない
    let s = parse_ok("mi zo'u zei broda");
    assert!(s.contains("sentence"), "{s}");
    assert!(!s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("zei_compound"), "{s}");
    assert!(s.contains("CMAVO_core \"zo'u\""), "{s}");
    assert!(s.contains("ZEI_core \"zei\""), "{s}");
    assert!(s.contains("BRIVLA_core \"broda\""), "{s}");
    // 単独の zei_compound(zo'u 先頭)は従来どおり受理(ガード不追加の根拠)
    let s = parse_ok("zo'u zei broda");
    assert!(s.contains("zei_compound"), "{s}");
    assert!(s.contains("CMAVO_core \"zo'u\""), "{s}");
    // mi zo'u zei zei broda: 旧順序=prenex(内文 zei zei broda)→ 新順序=
    // sentence(zei_compound「zo'u zei zei」+ tanru broda)の別木。
    // 受理は維持(退行なし)
    let s = parse_ok("mi zo'u zei zei broda");
    assert!(s.contains("sentence"), "{s}");
    assert!(!s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("zei_compound"), "{s}");
    assert!(s.contains("CMAVO_core \"zo'u\""), "{s}");
    assert!(s.contains("CMAVO_core \"zei\""), "{s}");
    assert!(s.contains("BRIVLA_core \"broda\""), "{s}");
    // 参考: bu_lerfu 経路(項位置)は旧順序でも prenex の内文が失敗して
    // sentence が選ばれるため順序非依存(実測: 旧順序でも受理・同木)。
    // bu_lerfu が term として「zo'u bu」を吸収する
    let s = parse_ok("mi zo'u bu broda");
    assert!(s.contains("sentence"), "{s}");
    assert!(!s.contains("prenex_sentence"), "{s}");
    assert!(s.contains("bu_lerfu"), "{s}");
    assert!(s.contains("CMAVO_core \"zo'u\""), "{s}");
    assert!(s.contains("BU_core \"bu\""), "{s}");
    // 量化描述の prenex 項は PA_seq が「ro」を先に確定するため
    // (PA_seq "ro")+(sumti desc …) の2項に分割される(項位置の
    // quant_desc 1ノードとの不整合。既存単純形優先の選言順序の副作用。
    // 受理自体は z0 整合)
    let s = parse_ok("ro lo ci gerku zo'u mi klama");
    assert!(
        s.contains("(prenex_sentence (PA_seq \"ro\") (sumti (desc"),
        "{s}"
    );
    assert!(s.contains("PA_core \"ci\""), "{s}");
    assert!(s.contains("BRIVLA_core \"gerku\""), "{s}");
    assert!(s.contains("ZOhU_core \"zo'u\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 拒否系の境界(単独 zo'u / 述語後の zo'u / zo'u 連続。z0/z1 も拒否)
    assert!(lojban::parse("zo'u").is_err());
    assert!(lojban::parse("mi klama zo'u").is_err());
    assert!(lojban::parse("mi zo'u zo'u").is_err());
    // 入れ子文脈の裸 prenex。gek 内と抽象内で受理
    // (z0/z1 はいずれも拒否する既知差分=拡張。内文の裸 prenex は
    // 頂層と同じく bridi 省略形として一貫して受理する)
    let s = parse_ok("ganai mi zo'u gi broda");
    assert!(s.contains("gek_sentence"), "{s}");
    assert!(
        s.contains("(prenex_sentence (KOhA_clause (KOhA_core \"mi\")) (ZOhU_clause"),
        "{s}"
    );
    assert!(s.contains("GI_core \"gi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"broda\""), "{s}");
    let s = parse_ok("mi nu lo preti zo'u kei klama");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("(prenex_sentence (sumti (desc"), "{s}");
    assert!(s.contains("KEI_core \"kei\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn mex接続詞演算子と前置形による量化sumti() {
    // v0.101: ユーザー報告「.i se ju no da mi tolprali lo nu troci」が
    // エラーになる問題を修正(zantufa z0/z1 は受理)。
    // z0 の解析: 「se ju」は文接続詞ではなく mex(数理表現)の演算子。
    // operator <- SE_clause operator(joik_ek(joik(JOI ju))) の前置形で
    // 演算子 se ju が被演算子 no を取り、quantifier(mex) + sumti_5(da)
    // の量化 sumti を成す。3つのギャップを解消:
    // ① mex_operator に (SE)? ~ mex_conn(A/JOI/JA系 + BIhI)を追加
    //   (CLL 16 の接続詞演算子。JOI_core は zantufa 準拠で ja/je/jo/ju を
    //   含むように拡張)
    // ② mex_operand に前置形 (SE)? ~ mex_conn ~ mex_operand を追加
    // ③ sumti_core に quant_sumti(mex + sumti_core)を追加
    // 対象文: quant_sumti(mex(SE_clause se + JOI_clause ju + number no)
    // + KOhA_clause da) の1項として解析される
    let s = parse_ok(".i se ju no da mi tolprali lo nu troci");
    assert!(s.contains("sentence"), "{s}");
    assert!(
        s.contains(
            "(quant_sumti (mex (SE_clause (SE_core \"se\")) (JOI_clause (JOI_core \"ju\")) \
             (number (PA_clause (PA_core \"no\")))) (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    assert!(s.contains("KOhA_core \"mi\""), "{s}");
    assert!(s.contains("BRIVLA_core \"tolprali\""), "{s}");
    assert!(s.contains("NU_core \"nu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"troci\""), "{s}");
    // 文頭でない形(述語の前項・後項・cu 文)
    let s = parse_ok("se ju no da klama");
    assert!(
        s.contains(
            "(quant_sumti (mex (SE_clause (SE_core \"se\")) (JOI_clause (JOI_core \"ju\")) \
             (number (PA_clause (PA_core \"no\")))) (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    let s = parse_ok("mi viska se ju no da");
    assert!(s.contains("BRIVLA_core \"viska\""), "{s}");
    assert!(
        s.contains(
            "(quant_sumti (mex (SE_clause (SE_core \"se\")) (JOI_clause (JOI_core \"ju\")) \
             (number (PA_clause (PA_core \"no\")))) (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    let s = parse_ok(".i se ju no da mi cu klama");
    assert!(s.contains("quant_sumti"), "{s}");
    assert!(s.contains("KOhA_core \"mi\""), "{s}");
    assert!(s.contains("CU_core \"cu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"klama\""), "{s}");
    // SE 変換 + A/JA 系(z0 は ja を JOI_clause として分類)
    let s = parse_ok(".i se ja no da klama");
    assert!(
        s.contains(
            "(mex (SE_clause (SE_core \"se\")) (JOI_clause (JOI_core \"ja\")) \
             (number (PA_clause (PA_core \"no\"))))"
        ),
        "{s}"
    );
    // SE 変換 + BIhI(区間演算子)
    let s = parse_ok(".i se bi'i no da klama");
    assert!(
        s.contains(
            "(mex (SE_clause (SE_core \"se\")) (BIhI_clause (BIhI_core \"bi'i\")) \
             (number (PA_clause (PA_core \"no\"))))"
        ),
        "{s}"
    );
    // NAI 付き joik 演算子
    let s = parse_ok(".i se ju nai no da klama");
    assert!(
        s.contains(
            "(mex (SE_clause (SE_core \"se\")) (JOI_clause (JOI_core \"ju\")) \
             (NAI_clause (NAI_core \"nai\")) (number (PA_clause (PA_core \"no\"))))"
        ),
        "{s}"
    );
    // SE なしの前置形(ju 単独の joik 演算子)
    let s = parse_ok(".i ju no da klama");
    assert!(s.contains("quant_sumti"), "{s}");
    // 中置形の接続詞演算子(li pa se ju re = 1 whether-or-not 2。z0 も受理)
    let s = parse_ok("li pa se ju re");
    assert!(s.contains("li_mex"), "{s}");
    assert!(s.contains("SE_clause"), "{s}");
    assert!(s.contains("JOI_core \"ju\""), "{s}");
    // 木変化ピン(v0.100→v0.101): li 内の接続詞は旧来
    // 「li_mex(mex(pa)) + ek_joik(joi) + sumti(number re)」の2項接続
    // だったが、mex_operator の接続詞枝により単一の
    // 「li_mex(mex(pa joi re))」(中置演算子)に変化した。
    // z0 の木(gerna_cipra js/zantufa-0.9999.js で実測)は
    // li_clause > mex > mex_1(operand(pa) + operator(joik_ek(joi))
    // + mex_1(operand(re))) の単一 mex 読みで、本実装の新木形と一致する
    let s = parse_ok("li pa joi re");
    assert!(
        s.contains(
            "(li_mex (LI_clause (LI_core \"li\")) (mex (number (PA_clause (PA_core \"pa\"))) \
             (JOI_clause (JOI_core \"joi\")) (number (PA_clause (PA_core \"re\")))))"
        ),
        "{s}"
    );
    // A 系の項接続詞も同様に単一 mex の中置演算子になる
    // (z0 は operator(joik_ek(ek(A_clause a))) の単一 mex 読み。実測)
    let s = parse_ok("li pa a re");
    assert!(
        s.contains(
            "(mex (number (PA_clause (PA_core \"pa\"))) (A_clause (A_core \"a\")) \
             (number (PA_clause (PA_core \"re\"))))"
        ),
        "{s}"
    );
    assert!(!s.contains("(sumti (number"), "{s}");
    // 多被演算子 mex による量化 sumti(quant_sumti 側の木変化)。
    // 「pa joi re da」は旧来 bare_number(pa) + ek_joik(joi) +
    // quant_selbri(re …) 等の複数項だったが、v0.101 からは
    // quant_sumti(mex(pa joi re) + KOhA(da)) の1項。
    // z0 の木は sumti_4(quantifier(mex(pa joi re)) + sumti_5(da)) の
    // 単一項読みで一致する(実測)
    let s = parse_ok("pa joi re da");
    assert!(
        s.contains(
            "(quant_sumti (mex (number (PA_clause (PA_core \"pa\"))) \
             (JOI_clause (JOI_core \"joi\")) (number (PA_clause (PA_core \"re\")))) \
             (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    // BO は演算子に付かない(z0 も .i se ju bo no da klama を拒否)。
    // BO を含めないのは項・文接続のスコープ短縮であって演算子の修飾ではないため
    assert!(lojban::parse(".i se ju bo no da klama").is_err());
    // 既存の BIhI 中置形の木は不変(mex_conn がサイレントのため)
    let s = parse_ok("li ci bi'i vo");
    assert!(
        s.contains(
            "(mex (number (PA_clause (PA_core \"ci\"))) (BIhI_clause (BIhI_core \"bi'i\")) \
             (number (PA_clause (PA_core \"vo\"))))"
        ),
        "{s}"
    );
    // 項接続への波及(z0 整合の受理拡張): JOI_core が JA 系を含むことで
    // ek_joik 経由の項論理接続も受理される(z0 は mi ja do / mi ju do を受理)
    let s = parse_ok("mi ja do klama");
    assert!(s.contains("JOI_core \"ja\""), "{s}");
    assert!(
        s.contains("KOhA_core \"mi\"") && s.contains("KOhA_core \"do\""),
        "{s}"
    );
    let s = parse_ok("mi ju do klama");
    assert!(s.contains("JOI_core \"ju\""), "{s}");
    // 既存の joi 項接続の木は不変
    let s = parse_ok("mi joi do klama");
    assert!(s.contains("JOI_core \"joi\""), "{s}");
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 量化sumtiへの木統合と既存不変ピン() {
    // v0.101: sumti_core に quant_sumti(mex + sumti_core)を追加したことに
    // よる木形状変化の固定と、既存規則(quant_desc/quant_selbri/prenex)の
    // 非変化ピン。
    // 木変化: 「数詞+KOhA」(no da / pa da / ro da 等)は旧来
    // bare_number + KOhA_clause の2項だったが、z0 の
    // quantifier(mex) + sumti_5 に合わせて quant_sumti 1項に統合される。
    // 受理自体は不変(旧経路でも受理されていた)で、木形のみの変化
    let s = parse_ok("no da klama");
    assert!(
        s.contains(
            "(quant_sumti (mex (number (PA_clause (PA_core \"no\")))) \
             (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    // 旧2項(bare_number + KOhA_clause)が sumti_core 直下に並ぶ形は消える
    assert!(!s.contains("(bare_number"), "{s}");
    let s = parse_ok("pa da klama");
    assert!(
        s.contains(
            "(quant_sumti (mex (number (PA_clause (PA_core \"pa\")))) \
             (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    // 述語の後ろの項も同様
    let s = parse_ok("mi klama no da");
    assert!(s.contains("quant_sumti"), "{s}");
    // 空白区切りの数詞連鎖は1つの number として mex に入る
    // (z0 の quantifier(number(pa re)) + sumti_5 と整合)
    let s = parse_ok("pa re da klama");
    assert!(
        s.contains(
            "(quant_sumti (mex (number (PA_clause (PA_core \"pa\")) (PA_clause (PA_core \"re\")))) \
             (KOhA_clause (KOhA_core \"da\")))"
        ),
        "{s}"
    );
    // 既存不変ピン1: 数詞+selbri は quant_selbri のまま
    // (quant_sumti は quant_selbri より後に試行される)
    let s = parse_ok("pa re prenu cu klama");
    assert!(s.contains("quant_selbri"), "{s}");
    assert!(!s.contains("quant_sumti"), "{s}");
    // 既存不変ピン2: 量化描述は quant_desc のまま
    let s = parse_ok("ro lo ci gerku cu klama");
    assert!(s.contains("quant_desc"), "{s}");
    assert!(!s.contains("quant_sumti"), "{s}");
    // 既存不変ピン3: prenex の項は PA_seq/KOhA_clause の2項のまま
    // (prenex_term の選言順序は変更していない)
    let s = parse_ok("su'o da zo'u da klama");
    assert!(
        s.contains(
            "(prenex_sentence (PA_seq \"su'o\") (KOhA_clause (KOhA_core \"da\")) (ZOhU_clause"
        ),
        "{s}"
    );
    // prenex 内文の項は従来どおり KOhA 単独
    let s = parse_ok("ro lo ci gerku zo'u mi klama");
    assert!(
        s.contains("(prenex_sentence (PA_seq \"ro\") (sumti (desc"),
        "{s}"
    );
    // 数詞+MOI/ROI は従来どおりガードで譲る(quant_sumti はガードを持たないが
    // sumti_core の後続枝が失敗するため影響しない)
    parse_ok("lo re moi prenu cu barda");
    parse_ok("mi za'u re'u klama");
    // 単独数詞は bare_number(サイレントのため木は number)のまま
    // (quant_sumti は後続 sumti を要求するため単独では失敗する)
    let s = parse_ok("mi viska pa");
    assert!(
        s.contains("(sumti (number (PA_clause (PA_core \"pa\"))))"),
        "{s}"
    );
    assert!(!s.contains("quant_sumti"), "{s}");
    // 否定系維持
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn bai_rai補完と_h変体() {
    // v0.102: BAI_core に ba'i/ci'o/rai と h 変体11語を追加。
    // 動機は zantufa(z0)が受理する「se rai」(traji 由来)のタグ付き項。
    // 対象文の木: lebna + terms[lo tajgai, tagged(se rai, lo ka junri simlu)]
    let s = parse_ok("lebna lo tajgai se rai lo ka junri simlu");
    assert!(s.contains("tagged"), "{s}");
    assert!(s.contains("SE_clause (SE_core \"se\")"), "{s}");
    assert!(s.contains("BAI_clause (BAI_core \"rai\")"), "{s}");
    assert!(s.contains("desc"), "{s}");
    // タグ付き項は tail_terms 内の第2項として現れる
    let tail = s.find("tail_terms").unwrap();
    let tagged = s.find("tagged").unwrap();
    assert!(tail < tagged, "{s}");

    // z0 プローブで受理を実測した追加語(ba'i/ci'o)
    parse_ok("lebna lo tajgai se ba'i lo ka junri simlu");
    parse_ok("lebna lo tajgai se ci'o lo ka junri simlu");
    // h 変体の代表(' ↔ h 規約。seho は z0 がタグ付き項位置で拒否するが規約整合で収録)
    parse_ok("lebna lo tajgai se bahi lo ka junri simlu");
    parse_ok("lebna lo tajgai se cuhu lo ka junri simlu");
    parse_ok("lebna lo tajgai se tahi lo ka junri simlu");
    parse_ok("lebna lo tajgai se ciho lo ka junri simlu");

    // 既存 BAI の受理維持(回帰確認)
    let s = parse_ok("lebna lo tajgai se ta'i lo ka junri simlu");
    assert!(s.contains("BAI_clause (BAI_core \"ta'i\")"), "{s}");
    parse_ok("lebna lo tajgai se bai lo ka junri simlu");
    parse_ok("lebna lo tajgai se mu'i lo ka junri simlu");

    // 非語彙は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn bai_追加語彙と_z0差分ピン() {
    // v0.102 続き: di'o/du'i/ga'a/te'i と h 変体9語、さらに ca'i と h 変体6語
    // (cahi/jahe/pahu/kahe/puha+zuhe(zu'e の機械変換))を追加。
    // いずれも z0 のタグ位置プローブで受理を実測済み(追加は z0 語彙に整合)。
    // 収録後の BAI_core は総数72語 = base 40語 + h 形32語(うち cihu は legacy)
    let s = parse_ok("lebna lo tajgai se di'o lo ka junri simlu");
    assert!(s.contains("BAI_clause (BAI_core \"di'o\")"), "{s}");
    parse_ok("lebna lo tajgai se du'i lo ka junri simlu");
    parse_ok("lebna lo tajgai se ga'a lo ka junri simlu");
    parse_ok("lebna lo tajgai se te'i lo ka junri simlu");
    // ca'i(by authority of)と h 形6語(cahi/jahe/pahu/kahe/puha/zuhe。z0 実測形。
    // ri'i/ka'a は機械変換形 rihi/kaha ではなく rihu/kahe を採用)
    let s = parse_ok("lebna lo tajgai se ca'i lo ka junri simlu");
    assert!(s.contains("BAI_clause (BAI_core \"ca'i\")"), "{s}");

    // h 変体の残りを網羅(bahi/cuhu/tahi/ciho は既存テスト、seho は下の差分ピン)
    for w in [
        "duho", "muhi", "nihi", "riha", "vaho", "rahi", "muhu", "kihu", "sihu", "rihu", "dehi",
        "diho", "duhi", "gaha", "tehi", "cahi", "jahe", "pahu", "kahe", "puha", "zuhe",
    ] {
        let s = parse_ok(&format!("lebna lo tajgai se {w} lo ka junri simlu"));
        assert!(s.contains(&format!("BAI_core \"{w}\"")), "{s}");
    }

    // z0 との受理差分(収録72語のタグ付き項位置プローブで実測): z0 は
    // se seho / se se'o の2語をこの位置でのみ拒否するが、' ↔ h 規約・
    // CLL 整合のため収録する(z0 は selbri 前タグ位置では両語を受理する)
    let s = parse_ok("lebna lo tajgai se seho lo ka junri simlu");
    assert!(s.contains("BAI_clause (BAI_core \"seho\")"), "{s}");
    // 差分ペアのもう一方(se'o は pre-existing)
    let s = parse_ok("lebna lo tajgai se se'o lo ka junri simlu");
    assert!(s.contains("BAI_clause (BAI_core \"se'o\")"), "{s}");

    // BAI + NAI 枝(tagged の (sp1 ~ NAI_clause)? )
    let s = parse_ok("lebna lo tajgai se rai nai lo ka junri simlu");
    assert!(
        s.contains("BAI_clause (BAI_core \"rai\")") && s.contains("NAI_clause (NAI_core \"nai\")"),
        "{s}"
    );

    // 文頭タグ位置: terms_full → term → tagged として解析される
    // (z0 もこの文形を受理することを実測済み)
    let s = parse_ok("se rai lo ka junri simlu cu se lebna");
    assert!(s.contains("tagged"), "{s}");
    assert!(s.contains("SE_clause (SE_core \"se\")"), "{s}");
    assert!(s.contains("BAI_clause (BAI_core \"rai\")"), "{s}");
    // タグ付き項は terms(文頭項列)内に現れる
    let terms = s.find("terms_full").unwrap();
    let tagged = s.find("tagged").unwrap();
    assert!(terms < tagged, "{s}");
    // 後続の cu と selbri が続く完全文
    assert!(s.contains("CU_clause (CU_core \"cu\")"), "{s}");
}

#[test]
fn tanru間の自由修飾語_対象文と受理() {
    // v0.103: tanru 単位間に自由修飾語を許容(tanru_post。z0 の
    // tanru_unit 後置 post_clause 相当)。動機はユーザー報告の実文:
    // 「.i .oi ta ca'o farlu ju'i cnita」— ju'i は tanru 単位間の呼格で、
    // cnita は tanru 継続単位(z0 も同じ読みで、呼格引数ではない)
    let s = parse_ok(".i .oi ta ca'o farlu ju'i cnita");
    assert!(s.contains("UI_core \"oi\""), "{s}");
    assert!(s.contains("KOhA_core \"ta\""), "{s}");
    assert!(s.contains("ZAhO_core \"ca'o\""), "{s}");
    // tanru 内に free(vocative ju'i) と単位 cnita が含まれる
    // (平準形: z0 は post_clause 内に右再帰ネストするが読みは同一)
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"farlu\"))) \
             (free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\"))))"
        ),
        "{s}"
    );

    // 単独の受理バリエーション(z0 実測: いずれも受理)
    let s = parse_ok("farlu ju'i cnita");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"farlu\"))) \
                    (free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) \
                    (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\"))))"
        ),
        "{s}"
    );
    parse_ok("mi klama ju'i cnita");
    // link(je)の前に free が挟まる形
    let s = parse_ok("farlu ju'i je cnita");
    assert!(
        s.contains(
            "(free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) \
                    (JA_clause (JA_core \"je\"))"
        ),
        "{s}"
    );
    // DOhU による明示閉鎖(z0 実測 ok)
    let s = parse_ok("farlu ju'i cnita dohu");
    assert!(s.contains("DOhU_clause (DOhU_core \"dohu\")"), "{s}");
    // 感情標識でも同じ経路(z0 実測: klama .ui cnita は受理)
    let s = parse_ok("klama .ui cnita");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))) \
             (free (free_unit (ui_free (UI_clause (UI_core \"ui\"))))) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\"))))"
        ),
        "{s}"
    );
    parse_ok("mi klama .ui cnita");
}

#[test]
fn tanru間の自由修飾語_既存木不変と_z0差分ピン() {
    // 既存木不変ピン1: 後続に単位 / link / DOhU が無い free は tanru_post が
    // 失敗して tail free にフォールバックする(「klama .ui」の木は v0.102 から不変)
    let s = parse_ok("klama .ui");
    // tanru は単位 klama のみ(free を取り込まない)
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))"),
        "{s}"
    );
    // free は tail_terms に留まる
    assert!(
        s.contains("tail_terms (free (free_unit (ui_free (UI_clause (UI_core \"ui\")))))"),
        "{s}"
    );

    // 複数 free のフォールバック(z0 実測 ok。z0 は free を post_clause 内に
    // 保持するが、本実装では tanru_post の frees_m が2個の free を消費した後
    // 継続グループが失敗し、文末の frees_m が2個の free を tail_terms 側に取る)
    let s = parse_ok("klama ju'i .ui");
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))"),
        "{s}"
    );
    assert!(
        s.contains(
            "tail_terms (free (free_unit (vocative (COI_clause (COI_core \"ju'i\")))) \
             (free_unit (ui_free (UI_clause (UI_core \"ui\")))))"
        ),
        "{s}"
    );

    // 既存木不変ピン2: 文頭の呼格 free + selbri(cnita farlu)のまま。
    // ju'i が tanru 側に取り込まれず、farlu は cnita の tanru 継続単位
    let s = parse_ok("ju'i cnita farlu");
    assert!(
        s.contains("(free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) (bridi_tail"),
        "{s}"
    );
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\"))) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"farlu\"))))"
        ),
        "{s}"
    );

    // z0 差分ピン(拒否維持): 裸 DOhU は free が前置されない限り拒否
    assert!(LojbanParser::parse(Rule::text, "klama dohu").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi klama dohu").is_err());
    // link 直後の free も拒否(z0 も拒否を実測)
    assert!(LojbanParser::parse(Rule::text, "farlu je ju'i cnita").is_err());

    // z0 交叉(実測)で受理を確認した追加形
    // vocative の明示閉鎖のみ(z0 も受理)
    let s = parse_ok("farlu ju'i dohu");
    assert!(s.contains("vocative"), "{s}");
    assert!(s.contains("DOhU_core \"dohu\""), "{s}");
    // vocative を挟んだ複数 free + 継続単位(z0 も受理)
    let s = parse_ok("farlu .ui ju'i cnita");
    assert!(
        s.contains(
            "(free (free_unit (ui_free (UI_clause (UI_core \"ui\")))) \
             (free_unit (vocative (COI_clause (COI_core \"ju'i\")))))"
        ),
        "{s}"
    );
    assert!(s.contains("BRIVLA_core \"cnita\""), "{s}");
    // 連続 vocative(z0 も受理)
    let s = parse_ok("farlu ju'i ju'i cnita");
    assert_eq!(s.matches("COI_core \"ju'i\"").count(), 2, "{s}");
    assert!(s.contains("BRIVLA_core \"cnita\""), "{s}");
    // vocative + 後続 free を伴う DOhU 閉鎖(z0 も受理)
    let s = parse_ok("farlu ju'i .ui dohu");
    assert!(s.contains("COI_core \"ju'i\""), "{s}");
    assert!(s.contains("DOhU_core \"dohu\""), "{s}");

    // 既知の z0 差分(過剰受容): vocative を含まない free + DOhU は z0 が
    // 拒否するが、tanru_post は語種を見ないため受理する。第3枝(DOhU)の
    // クラスと、第2枝の optional DOhU のクラスの両方を記録ピンとして保持
    // (grammar 側のコメント参照。受理側に倒した記録ピン)
    // 第3枝のクラス(free + DOhU。z0 実測 err)
    let s = parse_ok("farlu .ui dohu");
    assert!(s.contains("UI_core \"ui\""), "{s}");
    assert!(s.contains("DOhU_core \"dohu\""), "{s}");
    // 第2枝の optional DOhU のクラス(free + 単位 + DOhU。z0 実測 err)
    let s = parse_ok("farlu .ui cnita dohu");
    assert!(s.contains("UI_core \"ui\""), "{s}");
    assert!(s.contains("BRIVLA_core \"cnita\""), "{s}");
    assert!(s.contains("DOhU_clause (DOhU_core \"dohu\")"), "{s}");

    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn tanru単位のse_nae前置_対象文と受理() {
    // v0.104: tanru 単位に SE/NAhE を前置できる(tagji se klama 等)。
    // 動機はユーザー報告の実文: 「.i lo .abu xejni'a cu tai tagji se danre
    // lo jamfu ja'e lo nu carmi nandu fa lo nu kargau lo moklu」。
    // z0 実測スコープ: tanru 単位の前置は SE と NAhE のみで、
    // NA / JAhA は不可(selbri 直下の s_mark とスコープが異なる)
    let s = parse_ok(".i lo .abu xejni'a cu tai tagji se danre lo jamfu ja'e lo nu carmi nandu fa lo nu kargau lo moklu");
    // selbri の BAI 前置(tai。既存の selbri 経路)
    assert!(
        s.contains("(selbri (BAI_clause (BAI_core \"tai\")) (tanru"),
        "{s}"
    );
    // tanru 2単位目の SE 前置(v0.104 の新規経路)
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"tagji\"))) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"danre\"))))"),
        "{s}"
    );
    // terms(lo .abu xejni'a)と tail terms(lo jamfu + ja'e lo nu …)の構造
    assert!(s.contains("BY_core \"abu\""), "{s}");
    assert!(s.contains("BRIVLA_core \"xejni'a\""), "{s}");
    assert!(s.contains("BRIVLA_core \"jamfu\""), "{s}");
    assert!(
        s.contains("(tagged (BAI_clause (BAI_core \"ja'e\"))"),
        "{s}"
    );
    assert!(s.contains("nu_form (NU_clause (NU_core \"nu\"))"), "{s}");

    // 受理バリエーション(z0 実測: いずれも受理)
    let s = parse_ok("tagji se klama");
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"tagji\"))) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"klama\"))))"),
        "{s}"
    );
    let s = parse_ok("tagji na'e klama");
    assert!(
        s.contains(
            "(tanru_unit (NAhE_clause (NAhE_core \"na'e\")) \
                    (BRIVLA_clause (BRIVLA_core \"klama\")))"
        ),
        "{s}"
    );
    let s = parse_ok("tagji je'a klama");
    assert!(
        s.contains(
            "(tanru_unit (NAhE_clause (NAhE_core \"je'a\")) \
                    (BRIVLA_clause (BRIVLA_core \"klama\")))"
        ),
        "{s}"
    );
    // SE 語彙の他の語(SE_core = se/te/ve/xe。ve/xe は同経路で受理を実測、
    // z0 も受理)
    let s = parse_ok("tagji te klama");
    assert!(
        s.contains(
            "(tanru_unit (SE_clause (SE_core \"te\")) \
                    (BRIVLA_clause (BRIVLA_core \"klama\")))"
        ),
        "{s}"
    );
    // NAhE 語彙の他の語(NAhE_core = na'e/to'e/no'e/je'a。no'e は同経路で
    // 受理を実測、z0 も受理)
    let s = parse_ok("tagji to'e klama");
    assert!(
        s.contains(
            "(tanru_unit (NAhE_clause (NAhE_core \"to'e\")) \
                    (BRIVLA_clause (BRIVLA_core \"klama\")))"
        ),
        "{s}"
    );
    // 主語が先に来る形(項解析後に bridi_tail 側の tanru 単位として取る)
    let s = parse_ok("mi tagji se klama");
    assert!(
        s.contains("(terms_full (terms (term (sumti (KOhA_clause (KOhA_core \"mi\"))))) \
                    (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"tagji\"))) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\"))"),
        "{s}"
    );
    // NAhE+SE の重ね掛け(z0 実測 ok)
    let s = parse_ok("tagji na'e se klama");
    assert!(
        s.contains(
            "(tanru_unit (NAhE_clause (NAhE_core \"na'e\")) (SE_clause (SE_core \"se\")) \
                    (BRIVLA_clause (BRIVLA_core \"klama\")))"
        ),
        "{s}"
    );
    // tanru 継続の SE 前置(以前は2単位目に se を置けず拒否だった。z0 実測 ok)
    let s = parse_ok("klama se broda");
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"broda\"))))"),
        "{s}"
    );
    // tanru_link(je)経路の単位でも新前置が効く(z0 実測 ok)
    let s = parse_ok("broda je se broda");
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))) \
                    (JA_clause (JA_core \"je\")) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"broda\"))))"),
        "{s}"
    );
}

#[test]
fn tanru単位のse_nae前置_既存木不変と_z0差分ピン() {
    // z0 整合拒否ピン(z0 実測 err): tanru 単位の前置は SE/NAhE のみ。
    // NA / JAhA は selbri 直下の s_mark では取れるが、tanru 単位の前置としては不可
    assert!(LojbanParser::parse(Rule::text, "tagji na klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "tagji ja'a klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "tagji se ja'a klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "tagji ja'a se klama").is_err());
    // 順序固定(z0 スコープ): NAhE? ~ SE? のみで SE ~ NAhE は不可
    // (tagji se na'e klama は z0 も拒否を実測)
    assert!(LojbanParser::parse(Rule::text, "tagji se na'e klama").is_err());

    // 既存の selbri 直下 s_marks 経路は不変(単独 selbri の se/na'e は
    // 先行する s_marks が消費するため、tanru_unit の SE/NAhE 前置は出番がなく木も変わらない)
    let s = parse_ok("se danre");
    assert!(
        s.contains(
            "(selbri (s_mark (SE_clause (SE_core \"se\"))) \
                    (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"danre\")))))"
        ),
        "{s}"
    );
    let s = parse_ok("na'e danre");
    assert!(
        s.contains("(s_mark (NAhE_clause (NAhE_core \"na'e\")))"),
        "{s}"
    );

    // lujvo 語頭の se は SE 前置に読まれず BRIVLA 1語のまま(語境界により
    // SE_core が seltau / selma'o の語内 se に食み出さない)
    let s = parse_ok("mi seltau klama");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"seltau\"))) \
                    (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))"
        ),
        "{s}"
    );

    // BAhE 強調経路は不変(BAhE ~ sp1 ~ BRIVLA。SE/NAhE 前置は空のまま)
    let s = parse_ok("farlu ba'e cnita");
    assert!(
        s.contains(
            "(tanru_unit (BAhE_clause (BAhE_core \"ba'e\")) \
                    (BRIVLA_clause (BRIVLA_core \"cnita\")))"
        ),
        "{s}"
    );
    // BAhE ~ SE 組合せ(BAhE ~ sp1 ~ NAhE? ~ SE? ~ BRIVLA の順序どおり。
    // z0 実測 ok。ba'e na'e broda も同経路で受理を実測)
    let s = parse_ok("farlu ba'e se cnita");
    assert!(
        s.contains(
            "(tanru_unit (BAhE_clause (BAhE_core \"ba'e\")) (SE_clause (SE_core \"se\")) \
                    (BRIVLA_clause (BRIVLA_core \"cnita\")))"
        ),
        "{s}"
    );

    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 項が空のcu文_対象文と受理() {
    // v0.104: 項が空の cu 文(cu klama / cu pu klama 等)を受理する
    // (camxes terms-80 の terms 空許容相当。z0 も受理)。
    // terms_full に CU 先行の第2枝を追加し、CU_clause が terms_full の
    // 直接の子として現れる(第2枝経由の実木)
    let s = parse_ok("cu klama");
    assert!(
        s.contains("(terms_full (CU_clause (CU_core \"cu\")) \
                    (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\")))"),
        "{s}"
    );
    let s = parse_ok("cu pu klama");
    assert!(
        s.contains(
            "(terms_full (CU_clause (CU_core \"cu\")) \
                    (bridi_tail (selbri (PU_clause (PU_core \"pu\")) \
                    (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\")))"
        ),
        "{s}"
    );
    // tai は BAI_clause(z0 と同一の selbri 前置タグ読み)
    let s = parse_ok("cu tai klama");
    assert!(
        s.contains(
            "(bridi_tail (selbri (BAI_clause (BAI_core \"tai\")) \
                    (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\"))"
        ),
        "{s}"
    );
    let s = parse_ok("cu se danre");
    assert!(
        s.contains(
            "(terms_full (CU_clause (CU_core \"cu\")) \
                    (bridi_tail (selbri (s_mark (SE_clause (SE_core \"se\"))) \
                    (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"danre\"))))) \
                    (tail_terms \"\")))"
        ),
        "{s}"
    );
    let s = parse_ok("cu tagji se danre");
    assert!(
        s.contains("(terms_full (CU_clause (CU_core \"cu\")) \
                    (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"tagji\"))) \
                    (tanru_unit (SE_clause (SE_core \"se\")) (BRIVLA_clause (BRIVLA_core \"danre\"))))) \
                    (tail_terms \"\"))"),
        "{s}"
    );
}

#[test]
fn 項が空のcu文_既存木不変() {
    // 既存木不変ピン1: 項あり文は terms_full の第1枝のまま
    // (terms ~ CU_clause ~ bridi_tail。CU_clause は terms_full の子のまま)
    let s = parse_ok("mi cu se klama");
    assert!(
        s.contains(
            "(terms_full (terms (term (sumti (KOhA_clause (KOhA_core \"mi\"))))) \
                    (CU_clause (CU_core \"cu\")) \
                    (bridi_tail (selbri (s_mark (SE_clause (SE_core \"se\")))"
        ),
        "{s}"
    );

    // 既存木不変ピン2: 項無し文は従来どおり sentence の bridi_tail 枝を通る
    // (terms_full 第2枝に進まず CU_clause を出現させない)
    let s = parse_ok("klama");
    assert!(
        s.contains("(sentence (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\")))"),
        "{s}"
    );
    // selbri 前の時制も bridi_tail 枝のまま
    let s = parse_ok("pu klama");
    assert!(
        s.contains("(sentence (PU_clause (PU_core \"pu\")) \
                    (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) \
                    (tail_terms \"\")))"),
        "{s}"
    );

    // 第2枝の安全設計固定: bridi_tail は必須で、cu 単独 / cu mi のような
    // 述語を欠く形は拒否する(z0 も拒否を実測)
    assert!(LojbanParser::parse(Rule::text, "cu").is_err());
    assert!(LojbanParser::parse(Rule::text, "cu mi").is_err());
}

#[test]
fn fai_と_faha_fa_a_補完_対象文と受理() {
    // v0.105: FA に fai(公式 FA・不定格)を、FAhA に fa'a/faha
    // (v0.98 補完の取りこぼし)を追加。
    // ユーザー報告の対象文(z0 実測 ok)が受理される
    let s = parse_ok(
        ".i .abu mutce gleki lo nu facki lo du'u lo cnebo cu jai frili \
                      fai lo nu krobi'o fa'a ro da tai tu'a lo since",
    );
    // fai は FA_clause の tagged 項(jai frili の不定格項)
    assert!(
        s.contains(
            "(tagged (FA_clause (FA_core \"fai\")) \
                     (sumti (desc (LE_clause (LE_core \"lo\")) \
                     (selbri (tanru (tanru_unit (nu_form (NU_clause (NU_core \"nu\")) \
                     (sentence (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"krobi'o\")))))"
        ),
        "{s}"
    );
    // fa'a は tense_tags(tense_mark の FAhA_clause 枝)経由の tagged 項
    // (tense_tags は silent のため木には FAhA_clause が直接現れる)
    assert!(
        s.contains(
            "(tagged (FAhA_clause (FAhA_core \"fa'a\")) \
                     (sumti (quant_sumti (mex (number (PA_clause (PA_core \"ro\")))) \
                     (KOhA_clause (KOhA_core \"da\")))))"
        ),
        "{s}"
    );
    // tai(BAI)項・tu'a 抽象項も従来どおり
    assert!(
        s.contains(
            "(tagged (BAI_clause (BAI_core \"tai\")) (sumti (KOhA_clause (KOhA_core \"tu'a\"))))"
        ),
        "{s}"
    );

    // fai: 述語の前の項位置(z0 実測 ok)
    let s = parse_ok("mi fai lo zarci klama");
    assert!(
        s.contains("(term (tagged (FA_clause (FA_core \"fai\")) (sumti (desc (LE_clause (LE_core \"lo\")) \
                    (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))) \
                    (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))))))"),
        "{s}"
    );

    // fai: selbri の後ろの tail 位置(z0 実測 ok)
    let s = parse_ok("klama fai lo zarci");
    assert!(
        s.contains("(tail_terms (term (tagged (FA_clause (FA_core \"fai\")) (sumti (desc (LE_clause (LE_core \"lo\")) \
                    (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))))))))))"),
        "{s}"
    );

    // fa'a: 方位タグ付き項(z0 実測 ok)
    let s = parse_ok("mi fa'a lo zdani klama");
    assert!(
        s.contains("(term (tagged (FAhA_clause (FAhA_core \"fa'a\")) (sumti (desc (LE_clause (LE_core \"lo\")) \
                    (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zdani\"))) \
                    (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))))))"),
        "{s}"
    );

    // h 変体 faha も同一経路(z0 実測 ok)
    let s = parse_ok("mi faha lo zdani klama");
    assert!(s.contains("(FAhA_clause (FAhA_core \"faha\"))"), "{s}");

    // fa'a: tail 位置のタグ付き項(z0 実測 ok)
    let s = parse_ok("mi klama fa'a ro da");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (FAhA_clause (FAhA_core \"fa'a\")) \
                     (sumti (quant_sumti (mex (number (PA_clause (PA_core \"ro\")))) \
                     (KOhA_clause (KOhA_core \"da\")))))))"
        ),
        "{s}"
    );

    // 連続する fai 項(z0 実測 ok。fragment: mi + fai 項 + fai 項。
    // 第2 fai 項の描述が後続 klama を tanru として吸収する)
    let s = parse_ok("mi fai lo zarci fai lo zdani klama");
    assert_eq!(s.matches("(FA_clause (FA_core \"fai\"))").count(), 2, "{s}");
    assert!(
        s.contains(
            "(term (tagged (FA_clause (FA_core \"fai\")) \
                     (sumti (desc (LE_clause (LE_core \"lo\")) \
                     (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zdani\"))) \
                     (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))))))))))"
        ),
        "{s}"
    );

    // 文頭の fai 項(z0 実測 ok。fragment: tagged 項のみ。描述が
    // 後続 klama を tanru として吸収する)
    let s = parse_ok("fai lo zarci klama");
    assert!(
        s.contains(
            "(fragment (terms (term (tagged (FA_clause (FA_core \"fai\")) \
                     (sumti (desc (LE_clause (LE_core \"lo\")) \
                     (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))) \
                     (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))))))))))"
        ),
        "{s}"
    );
}

#[test]
fn fai_と_faha_fa_a_補完_既存不変と_z0差分ピン() {
    // z0 整合拒否ピン: va'a/vaha は VUhU の単項演算子(加法逆元。除算は fe'i)
    // であり FAhA ではない。z0 も拒否を実測のため収録しない
    assert!(LojbanParser::parse(Rule::text, "mi va'a lo zdani klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi vaha lo zdani klama").is_err());

    // 既存 FA の受理と木は不変(v0.104 までの fa〜fu)
    let s = parse_ok("mi klama fa lo zarci");
    assert!(
        s.contains("(tail_terms (term (tagged (FA_clause (FA_core \"fa\")) (sumti (desc (LE_clause (LE_core \"lo\")) \
                    (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))))))))))"),
        "{s}"
    );

    // 既存 FAhA の受理と木は不変(v0.98 補完の ca'u 等)
    let s = parse_ok("mi klama ca'u lo zdani");
    assert!(
        s.contains("(tail_terms (term (tagged (FAhA_clause (FAhA_core \"ca'u\")) (sumti (desc (LE_clause (LE_core \"lo\")) \
                    (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zdani\"))))))))))"),
        "{s}"
    );

    // fi'a(不定スロットの疑問格)は既存どおり FA_core で受理
    let s = parse_ok("mi fi'a lo zarci klama");
    assert!(s.contains("(FA_clause (FA_core \"fi'a\"))"), "{s}");

    // fai を含む複合(tail 位置に fai 項と fa'a 項を連続。z0 実測 ok)
    let s = parse_ok("mi klama fai lo zdani fa'a lo zarci");
    assert!(s.contains("(FA_clause (FA_core \"fai\"))"), "{s}");
    assert!(s.contains("(FAhA_clause (FAhA_core \"fa'a\"))"), "{s}");

    // FA+NAI(v0.108): tagged の FA 枝が (sp1 ~ NAI_clause)? を取るように
    // なり fai nai / fa nai を受理(z0/z1 実測 ok。maftufa は拒否の参照分裂
    // だが z0 整合優先。かつては拒否ピンだったが GAP 解消で受理に変更)
    let s = parse_ok("fai nai lo zarci klama");
    assert!(s.contains("(NAI_clause (NAI_core \"nai\"))"), "{s}");
    let s = parse_ok("fa nai mi klama");
    assert!(s.contains("(NAI_clause (NAI_core \"nai\"))"), "{s}");

    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn レタル語は_tanru単位のbrivlaでない_v0_106回帰ピン() {
    // 回帰(v0.105 ユーザー報告): BRIVLA_core が母音始まりのレタル語
    // 「abu」を fuhivla 扱いで brivla 誤マッチし、「lo mentu .abu sanli」の
    // 描述が tanru(mentu abu sanli) を吸収していた。その結果 sentence 経路
    // の bridi_tail が空になり、gi'e 連鎖を伴う対象文が全文エラーに
    // なっていた。v0.106 で tanru_unit の BRIVLA 枝に BY_core 否定先読みを
    // 追加し、レタル語を tanru 単位の brivla としない。
    // z0 実測の参照源は zantufa-0.9999.js(オーケストレーター実測)。実装側の
    // 当初実測は maftufa-1.9999.js だったが、tester が z0/z1(zantufa-0.9999.js /
    // zantufa-1.9999.js)で再検証済み・結果同一(唯一の差は既知 GAP lo byklesi ku)。

    // z0 整合の新規拒否ピン: レタル語単独は描述の brivla にならない。
    // z0 実測(zantufa-0.9999.js)err。v0.105 までは誤受理していた
    // (拒否への変化=意図的修正)
    assert!(LojbanParser::parse(Rule::text, "lo abu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo ebu ku").is_err());

    // 境界ピン: ybu/by の語形は旧版から拒否で不変(z0 実測 err、zantufa-0.9999.js)
    assert!(LojbanParser::parse(Rule::text, "lo ybu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo by ku").is_err());

    // 個別拒否ピン: ibu/obu/ubu も同経路(z0 実測 err。v0.105 までは誤受理)
    assert!(LojbanParser::parse(Rule::text, "lo ibu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo obu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo ubu ku").is_err());

    // selbri 前置も不可: se abu は SE 変換+レタル語の brivla にならない
    // (z0 実測 err、zantufa-0.9999.js。v0.105 までは se abu を brivla 誤受理)
    assert!(LojbanParser::parse(Rule::text, "se abu broda").is_err());

    // 木変化ピン: desc が .abu を tanru に吸収しなくなった新木
    // (旧: fragment 内の desc(lo, tanru(mentu, abu, sanli)))
    let s = parse_ok("lo mentu .abu sanli");
    assert_eq!(
        s,
        "(text (content (item (sentence (terms_full (terms (term (sumti (desc (LE_clause (LE_core \
         \"lo\")) (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"mentu\")))))))) (term \
         (sumti (BY_clause (BY_core \"abu\"))))) (bridi_tail (selbri (tanru (tanru_unit \
         (BRIVLA_clause (BRIVLA_core \"sanli\"))))) (tail_terms \"\")))))))"
    );

    // 木変化ピン: fragment → sentence への変化。注: sentence の tense_marks
    // が ze'a を先に消費する既存設計(v0.94 系)のため、木は z0 の
    // tagged(ZI ze'a, …) 項読みではなく ZI_clause 読みになる
    // (v0.93/94 の既知クラス)
    let s = parse_ok("ze'a lo mentu .abu sanli");
    assert_eq!(
        s,
        "(text (content (item (sentence (ZI_clause (ZI_core \"ze'a\")) (terms_full (terms (term \
         (sumti (desc (LE_clause (LE_core \"lo\")) (selbri (tanru (tanru_unit (BRIVLA_clause \
         (BRIVLA_core \"mentu\")))))))) (term (sumti (BY_clause (BY_core \"abu\"))))) \
         (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"sanli\"))))) \
         (tail_terms \"\")))))))"
    );

    // 木変化ピン: レタル語列の文。旧(v0.105): sentence の bridi_tail が
    // tanru(BRIVLA abu, ebu, ibu) に誤読 → 新: fragment の terms 3項
    // (各 BY_clause)に変化。受理は維持(z0 実測 ok、zantufa-0.9999.js)
    let s = parse_ok(".abu .ebu .ibu");
    assert_eq!(
        s,
        "(text (content (item (fragment (terms (term (sumti (BY_clause (BY_core \"abu\")))) \
         (term (sumti (BY_clause (BY_core \"ebu\")))) (term (sumti (BY_clause (BY_core \
         \"ibu\")))))))))"
    );

    // 木変化クラスの記録: 「pa abu broda」型。旧(v0.105): fragment の
    // quant_selbri(number pa, selbri(tanru(abu, broda)) の数詞+selbri 誤読)→
    // 新: sentence の quant_sumti(mex pa + BY abu)+ selbri broda。受理は維持で、
    // z0 の quantifier+sumti 読みに近づく方向(z0 実測 ok、zantufa-0.9999.js)
    let s = parse_ok("pa abu broda");
    assert_eq!(
        s,
        "(text (content (item (sentence (terms_full (terms (term (sumti (quant_sumti (mex \
         (number (PA_clause (PA_core \"pa\")))) (BY_clause (BY_core \"abu\")))))) \
         (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))))) \
         (tail_terms \"\")))))))"
    );

    // 受理ピン: ba'e abu broda。ba'e(BAhE 強調)は自由修飾語(bae_free)読みで
    // abu を tanru 単位にしない(z0 実測 ok、zantufa-0.9999.js)。
    // 木は v0.105 から不変(free + term(BY abu) + selbri broda)
    let s = parse_ok("ba'e abu broda");
    assert!(
        s.contains("(free (free_unit (bae_free (BAhE_clause (BAhE_core \"ba'e\")))))"),
        "{s}"
    );
    assert!(
        s.contains("(term (sumti (BY_clause (BY_core \"abu\"))))"),
        "{s}"
    );
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"broda\"))"), "{s}");

    // 既存 OVER 差分の記録(pre-existing、本変更と無関係): 母音始まり fu'ivla。
    // fuhivla 規則が母音開始を許すため「aburobu」を fu'ivla 扱いで受理するが、
    // z0 は拒否を実測(lo aburobu ku = err、zantufa-0.9999.js)。
    // 本実装は受理のまま実態どおりピン(z0 実測ソース: zantufa-0.9999.js)
    let s = parse_ok("lo aburobu ku");
    assert!(
        s.contains(
            "(desc (LE_clause (LE_core \"lo\")) (selbri (tanru (tanru_unit \
             (BRIVLA_clause (BRIVLA_core \"aburobu\"))))) (KU_clause (KU_core \"ku\")))"
        ),
        "{s}"
    );

    // ユーザー報告の対象文(z0 実測 ok、zantufa-0.9999.js)。木形状差異は上記の既知クラスに同じ
    let s = parse_ok(
        "ni'o ze'a lo mentu .abu sanli gi'e catlu lo zdani gi'e pensi \
         lo du'u .ei ba zi zukte ma kau .i",
    );
    assert!(
        s.contains("(sep (NIhO_clause (NIhO_core \"ni'o\")))"),
        "{s}"
    );
    assert!(
        s.contains("(sentence (ZI_clause (ZI_core \"ze'a\")) (terms_full"),
        "{s}"
    );
    assert!(
        s.contains(
            "(term (sumti (desc (LE_clause (LE_core \"lo\")) (selbri (tanru (tanru_unit \
             (BRIVLA_clause (BRIVLA_core \"mentu\")))))))) (term (sumti (BY_clause (BY_core \
             \"abu\"))))"
        ),
        "{s}"
    );
    assert_eq!(
        s.matches("(GIhA_clause (GIhA_core \"gi'e\"))").count(),
        2,
        "{s}"
    );
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"sanli\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"catlu\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"zdani\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"pensi\"))"), "{s}");
    assert!(s.contains("(NU_clause (NU_core \"du'u\"))"), "{s}");
    assert!(s.contains("(UI_clause (UI_core \"ei\"))"), "{s}");
    assert!(s.contains("(PU_clause (PU_core \"ba\"))"), "{s}");
    assert!(s.contains("(ZI_clause (ZI_core \"zi\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"zukte\"))"), "{s}");
    assert!(s.contains("(KOhA_clause (KOhA_core \"ma\"))"), "{s}");
    assert!(s.contains("(UI_clause (UI_core \"kau\"))"), "{s}");
    assert!(s.contains("(I_clause (I_core \"i\"))"), "{s}");

    // ni'o / ze'a を外した形も z0 整合で受理(zantufa-0.9999.js 実測 ok)
    parse_ok(".abu sanli gi'e catlu lo zdani gi'e pensi lo du'u .ei ba zi zukte ma kau");

    // 既存不変: 通常文・gi'e 連鎖は従来どおり
    let s = parse_ok("mi klama fa lo zarci");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"zarci\"))"), "{s}");
    let s = parse_ok("mi sanli gi'e catlu lo zdani");
    assert_eq!(
        s.matches("(GIhA_clause (GIhA_core \"gi'e\"))").count(),
        1,
        "{s}"
    );

    // 旧既知 GAP の解消(v0.110): レタル接頭融合語(byklesi 型)。
    // z0 実測(zantufa-0.9999.js)では by+brivla の無ポーズ隣接により
    // lerfu_string + BOI(省略) + selbri の読みで受理される。本実装は
    // tanru_unit の BY_prefix 枝(BY_prefix + BRIVLA_clause)で受理する
    // (木形状差異 = 既知クラス)。下記のレタル接頭融合語テストを参照
    assert!(LojbanParser::parse(Rule::text, "lo byklesi ku").is_ok());

    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

// =====================================================================
// 項の後に先接続文(gek)が続く形(v0.107)
// sentence の第3代替 gek_after_terms(camxes sentence-50 相当)。
// ユーザ報告の解析エラーの回帰ピン。z0(zantufa-0.9999.js)実測と突き合わせ済み
// =====================================================================

#[test]
fn 項の後の先接続文_ユーザ報告文() {
    // ユーザ報告の対象文。v0.106 では err(fragment が ca 項だけ食って
    // gek 以降が宙に浮き text の EOI が満たされなかった)。z0 実測 ok
    let s = parse_ok(".i ca lo nu .abu cu za'u re'u mipri catlu kei ge la finpe selfu ba'o cliva gi lo drata cu zutse lo loldi ne'a lo vorme gi'e bebna catlu fa'a lo tsani");
    // gek_after_terms は silent のため GA/GI と内側 sentence が
    // sentence の直接の子として平準化される
    assert!(s.contains("(GA_clause (GA_core \"ge\"))"), "{s}");
    assert!(s.contains("(GI_clause (GI_core \"gi\"))"), "{s}");
    // ca は sentence の tense_marks 経路で先取りされる
    // (z0 は「ca lo nu … kei」全体をタグ付き項に取る既知クラス差異)
    assert!(s.contains("(PU_clause (PU_core \"ca\"))"), "{s}");
    // 抽象項 lo nu .abu cu za'u re'u mipri catlu kei
    assert!(s.contains("(NU_clause (NU_core \"nu\"))"), "{s}");
    assert!(s.contains("(KEI_clause (KEI_core \"kei\"))"), "{s}");
    assert!(s.contains("(ROI_clause (ROI_core \"re'u\"))"), "{s}");
    // gek 直後の内側文1: la finpe selfu ba'o cliva
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"selfu\"))"), "{s}");
    assert!(s.contains("(ZAhO_clause (ZAhO_core \"ba'o\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"cliva\"))"), "{s}");
    // gi 直後の内側文2: lo drata cu zutse lo loldi ne'a lo vorme
    // gi'e bebna catlu fa'a lo tsani
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"zutse\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"vorme\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"tsani\"))"), "{s}");
    // 内側文2の gi'e 連鎖と fa'a タグは従来経路
    assert!(s.contains("(GIhA_clause (GIhA_core \"gi'e\"))"), "{s}");
    assert!(s.contains("(FAhA_clause (FAhA_core \"fa'a\"))"), "{s}");
}

#[test]
fn 項の後の先接続文_z0実測プローブ() {
    // タグ付き項+gek(v0.106 までのギャップ本体)
    let s = parse_ok("ca lo nu .abu cu za'u re'u mipri catlu kei ge mi cliva gi mi cadzu");
    assert!(s.contains("(GA_clause (GA_core \"ge\"))"), "{s}");
    assert!(s.contains("(GI_clause (GI_core \"gi\"))"), "{s}");
    assert!(s.contains("(PU_clause (PU_core \"ca\"))"), "{s}");
    assert_eq!(
        s.matches("(KOhA_clause (KOhA_core \"mi\"))").count(),
        2,
        "{s}"
    );
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"cliva\"))"), "{s}");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"cadzu\"))"), "{s}");

    // 項+ga
    let s = parse_ok("mi ga broda gi broda");
    assert!(s.contains("(GA_clause (GA_core \"ga\"))"), "{s}");
    // 項+cu+ge
    let s = parse_ok("mi cu ge broda gi broda");
    assert!(s.contains("(CU_clause (CU_core \"cu\"))"), "{s}");
    assert!(s.contains("(GA_clause (GA_core \"ge\"))"), "{s}");
    // 量化項+ge
    parse_ok("da ge broda gi broda");
    // 項+裸タグ+ge。z0 は裸タグ(pu 等)を terms 内の tag_term として取るが、
    // 本実装は gek_after_terms の tense_marks? スロットで受ける
    parse_ok("mi pu ge broda gi broda");
    // 裸タグ+ge(項なし)/ cu 前置も z0 実測 ok で受理
    parse_ok("pu ge broda gi broda");
    parse_ok("cu ge broda gi broda");
}

#[test]
fn 項の後の先接続文_既存不変ピン() {
    // 裸 gek は従来どおり gek_sentence 経路(item/inner_sentence では
    // gek_sentence が先に試行されるため gek_after_terms に奪われない)
    let s = parse_ok("ge mi cliva gi mi cadzu");
    assert!(
        s.contains("(gek_sentence (GA_clause (GA_core \"ge\")) (sentence"),
        "{s}"
    );
    assert!(s.contains("(GI_clause (GI_core \"gi\"))"), "{s}");

    // gek を含まないタグ付き項の通常文は従来どおり sentence 経路
    // (gek_after_terms を経ないため木は不変)
    let s = parse_ok("ca lo nu .abu cu za'u re'u mipri catlu kei mi cliva");
    assert!(!s.contains("GA_clause"), "{s}");
    assert!(s.contains("(PU_clause (PU_core \"ca\"))"), "{s}");
    assert!(
        s.contains(
            "(bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"cliva\")))))"
        ),
        "{s}"
    );

    // 項+selbri の通常文は従来どおり terms_full の bridi_tail 経路
    let s = parse_ok("mi klama");
    assert!(s.contains("(terms_full (terms (term (sumti (KOhA_clause (KOhA_core \"mi\"))))) (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))) (tail_terms \"\")))"), "{s}");
}

#[test]
fn 項の後の先接続文_拒否ピン() {
    // gek の後に文が無い形。z0 も拒否する(zantufa-0.9999.js 実測 err)。
    // fragment(mi)が項を消費した後 text の EOI が満たされず全体を拒否
    assert!(LojbanParser::parse(Rule::text, "mi ge").is_err());
    // 裸 ku はタグとして取れない(tense_marks はタグ付きでしか KU を取らない。
    // z0 実測 err と一致)
    assert!(LojbanParser::parse(Rule::text, "mi ku ge broda gi broda").is_err());
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

// ===== v0.108: バッチ1 GAP 解消(語彙 ji/va'u + NAI 後置)の回帰テスト =====

#[test]
fn ji_接続詞疑問の項接続_v0_108() {
    // ji(接続詞疑問「〜かそれとも〜か?」)を JOI_core に収録。
    // z0 実測(zantufa-0.9999.js): 「mi ji do klama」は joik_ek(JOI_clause ji)
    // の項接続で受理。JOI 終端に ji を含める zantufa 準拠(v0.101 の
    // ja/je/jo/ju 追加と同クラスの語彙追加)
    let s = parse_ok("mi ji do klama");
    assert!(s.contains("(JOI_clause (JOI_core \"ji\"))"), "{s}");
    // 項接続は sumti の繰り返し(ek_joik 経路)で構築される
    assert!(
        s.contains(
            "(sumti (KOhA_clause (KOhA_core \"mi\")) \
             (JOI_clause (JOI_core \"ji\")) \
             (sumti (KOhA_clause (KOhA_core \"do\"))))"
        ),
        "{s}"
    );

    // 3 形とも z0/z1/maftufa 実測 ok(gap_tracker の GAP_接続詞疑問_ji と同一入力)
    parse_ok("mi ji do klama lo zdani");
    parse_ok("do ji mi broda");

    // NAI との組合せ(ji nai)も JOI 既存語と同型
    let s = parse_ok("mi ji nai do klama");
    assert!(s.contains("(JOI_clause (JOI_core \"ji\"))"), "{s}");
    assert!(s.contains("(NAI_clause (NAI_core \"nai\"))"), "{s}");
}

#[test]
fn ji_既存_joi_不変ピン_v0_108() {
    // 既存 JOI 語の項接続と木は不変(ji 追加は選言の末尾追加のみ)
    let s = parse_ok("mi joi do klama");
    assert!(s.contains("(JOI_clause (JOI_core \"joi\"))"), "{s}");
    let s = parse_ok("mi ja do klama");
    assert!(s.contains("(JOI_clause (JOI_core \"ja\"))"), "{s}");

    // ji は JA_core(文接続・gek 頭部)には入れていない。z0 実測 err と一致
    assert!(LojbanParser::parse(Rule::text, "mi ji broda gi broda").is_err());
    // JA_core の既存語形と tanru 接続経路は不変
    let s = parse_ok("mi klama je cadzu");
    assert!(s.contains("(JA_clause (JA_core \"je\"))"), "{s}");

    // 演算子位置(mex_conn)でも ji を受理(v0.101 の ek_joik 共用経路)
    let s = parse_ok("li re su'i ji ci");
    assert!(s.contains("(JOI_clause (JOI_core \"ji\"))"), "{s}");
}

#[test]
fn vau_単独タグと_h変体_v0_108() {
    // va'u(〜のおかげで)を BAI_core に追加。z0 実測(zantufa-0.9999.js):
    // 「va'u mi klama」は tag_term(BAI_clause + sumti mi) + bridi_tail(klama)。
    // 本実装は文頭タグを tense_marks が先取りする既知クラス(mu'i 等と同一の
    // 木形状差異で、受理・読みは同一)
    let s = parse_ok("va'u mi klama");
    assert!(
        s.contains("(sentence (BAI_clause (BAI_core \"va'u\"))"),
        "{s}"
    );

    // h 変体(' ↔ h 規約)。z0/z1/maftufa 実測 ok
    let s = parse_ok("vahu mi klama");
    assert!(s.contains("(BAI_clause (BAI_core \"vahu\"))"), "{s}");

    // 尾項位置(2語形タグ)
    let s = parse_ok("mi klama va'u lo zdani");
    assert!(s.contains("(BAI_clause (BAI_core \"va'u\"))"), "{s}");
    let s = parse_ok("mi klama vahu lo zdani");
    assert!(
        s.contains("(tagged (BAI_clause (BAI_core \"vahu\"))"),
        "{s}"
    );

    // NAI 後置は BAI 枝の既存形式(tense_mark の BAI 枝内 (sp1 ~ NAI_clause)?)
    let s = parse_ok("va'u nai mi klama");
    assert!(s.contains("(BAI_clause (BAI_core \"va'u\"))"), "{s}");
    assert!(s.contains("(NAI_clause (NAI_core \"nai\"))"), "{s}");
}

#[test]
fn se_vau_2語形と_結合形の揺れ解消_v0_108() {
    // se va'u は tagged の SE+BAI 2語形経路。z0 実測:
    // tag_term(SE_clause? + BAI_clause va'u + sumti) 相当で読みは同一
    let s = parse_ok("se va'u mi klama");
    assert!(
        s.contains(
            "(term (tagged (SE_clause (SE_core \"se\")) \
             (BAI_clause (BAI_core \"va'u\")) \
             (sumti (KOhA_clause (KOhA_core \"mi\")))))"
        ),
        "{s}"
    );

    // 尾項位置+描述
    let s = parse_ok("mi klama se va'u lo nu broda");
    assert!(
        s.contains(
            "(tagged (SE_clause (SE_core \"se\")) \
             (BAI_clause (BAI_core \"va'u\")) \
             (sumti (desc (LE_clause (LE_core \"lo\"))"
        ),
        "{s}"
    );

    // 結合形 seva'u/sevahu(SEBAI_joint)の既存経路は不変
    // (BAI_core に va'u を追加しても結合形が優先される = 語形が長いSEBAI_joint
    //  を選択肢に置く既存順序の維持)
    let s = parse_ok("seva'u do mi klama");
    assert!(s.contains("SEBAI_joint \"seva'u\""), "{s}");
    assert!(!s.contains("(SE_clause (SE_core \"se\"))"), "{s}");
    let s = parse_ok("sevahu do mi klama");
    assert!(s.contains("SEBAI_joint \"sevahu\""), "{s}");
}

#[test]
fn fa_nai_不定格の否定_v0_108() {
    // tagged の FA 枝に (sp1 ~ NAI_clause)? を追加(fa/fe/fi/fo/fu/fi'a/fai 共通)。
    // z0 実測(zantufa-0.9999.js): 「fa nai mi klama」は tag_term 内の FA_clause に
    // post_clause(free(UI_clause nai)) が付いた形 —— zantufa は nai を UI 自由修飾語
    // として読む。本実装は BAI 枝と同型の NAI 後置(タグ否定の CLL 読み)で
    // 受理集合を整合させる(木形状は既知の読み差異クラス)。
    // maftufa は fa nai 形を拒否する(参照分裂。z0 整合優先で受理)
    let s = parse_ok("fa nai mi klama");
    assert!(
        s.contains(
            "(term (tagged (FA_clause (FA_core \"fa\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (sumti (KOhA_clause (KOhA_core \"mi\")))))"
        ),
        "{s}"
    );

    let s = parse_ok("fai nai lo zarci klama");
    assert!(
        s.contains(
            "(tagged (FA_clause (FA_core \"fai\")) \
             (NAI_clause (NAI_core \"nai\"))"
        ),
        "{s}"
    );

    // 尾項位置(スロットの否定)も受理
    parse_ok("mi klama fe nai");
    parse_ok("mi klama fai nai");
}

#[test]
fn fa_nai_裸タグ項_v0_108() {
    // z0 の tag_term は tag + (sumti | KU_elidible) で、sumti を伴わない
    // 裸タグ項を受理する(実測: mi klama fa / mi klama fa nai / fa / fa nai が
    // z0/z1 ok)。FA 枝の sumti を省略可能にし同経路を再現。
    // sumti の省略はオプション化のため既存入力の木は不変
    let s = parse_ok("mi klama fa nai");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (FA_clause (FA_core \"fa\")) \
             (NAI_clause (NAI_core \"nai\")))))"
        ),
        "{s}"
    );

    // 裸 fa の発話フラグメント(z0 実測 ok)
    let s = parse_ok("fa");
    assert!(
        s.contains("(fragment (terms (term (tagged (FA_clause (FA_core \"fa\"))))))"),
        "{s}"
    );
    let s = parse_ok("fa nai");
    assert!(
        s.contains("(FA_clause (FA_core \"fa\")) (NAI_clause (NAI_core \"nai\"))"),
        "{s}"
    );

    // 裸タグ項の直後に描述を続ける形(z0 実測 ok。この形は z0 も
    // tag+sumti の同束読みで、受理集合は一致)
    parse_ok("mi klama fa nai lo zarci");
}

#[test]
fn fa_既存不変_v0_108() {
    // FA 枝の NAI 後置・sumti 省略化はオプション追加のみで既存の木は不変
    let s = parse_ok("mi klama fa lo zarci");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (FA_clause (FA_core \"fa\")) \
             (sumti (desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))))))))))"
        ),
        "{s}"
    );
    let s = parse_ok("mi fi'a lo zarci klama");
    assert!(s.contains("(FA_clause (FA_core \"fi'a\"))"), "{s}");

    // FA+FAhA 連続の既存木も不変
    let s = parse_ok("mi klama fai lo zdani fa'a lo zarci");
    assert!(s.contains("(FA_clause (FA_core \"fai\"))"), "{s}");
    assert!(s.contains("(FAhA_clause (FAhA_core \"fa'a\"))"), "{s}");

    // 裸 fa(NAI 無し)も z0 同様の裸タグ項として受理(z0 実測 ok)
    parse_ok("mi klama fa");
}

#[test]
fn 時制タグの_nai_v0_108() {
    // tense_mark(全タグ共通)に (sp1 ~ NAI_clause)? を後置。
    // z0/z1/maftufa 実測 ok(gap_tracker の GAP_時制タグの_nai と同一入力)。
    // z0 実測(zantufa-0.9999.js): 「mi pu nai klama」の nai はタグ直後の
    // post_clause(free(UI_clause nai))。本実装は selbri 内 tense_marks の
    // NAI_clause(「過去ではない」のタグ否定読み)で受理集合を整合させる
    let s = parse_ok("mi pu nai klama");
    assert!(
        s.contains(
            "(selbri (PU_clause (PU_core \"pu\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))"
        ),
        "{s}"
    );

    parse_ok("mi ba nai klama");
    let s = parse_ok("mi ca nai klama");
    assert!(
        s.contains("(PU_clause (PU_core \"ca\")) (NAI_clause (NAI_core \"nai\"))"),
        "{s}"
    );

    // 時制連鎖+BAI の組合せ(z0 実測 ok)
    let s = parse_ok("mi pu nai bai klama");
    assert!(
        s.contains(
            "(PU_clause (PU_core \"pu\")) (NAI_clause (NAI_core \"nai\")) \
             (BAI_clause (BAI_core \"bai\"))"
        ),
        "{s}"
    );

    // タグ付き項位置(時制タグ+NAI+sumti。z0 実測 ok)
    let s = parse_ok("mi ba nai lo zdani klama");
    assert!(
        s.contains(
            "(tagged (PU_clause (PU_core \"ba\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (sumti (desc (LE_clause (LE_core \"lo\"))"
        ),
        "{s}"
    );
}

#[test]
fn 時制タグ_nai_既存不変ピン_v0_108() {
    // NAI を含まない既存入力の木は不変(オプション後置のみ)
    let s = parse_ok("mi pu klama");
    assert!(
        s.contains(
            "(selbri (PU_clause (PU_core \"pu\")) \
             (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))"
        ),
        "{s}"
    );
    assert!(!s.contains("NAI_clause"), "{s}");

    // BAI 枝の既存 in-branch NAI 後置(bai nai)は不変
    let s = parse_ok("mi klama bai nai lo zdani");
    assert!(
        s.contains("(BAI_clause (BAI_core \"bai\")) (NAI_clause (NAI_core \"nai\"))"),
        "{s}"
    );

    // 文頭 BAI+NAI(v0.108 前から tense_mark の BAI 枝で受理)
    let s = parse_ok("va'u nai mi klama");
    assert!(s.contains("(BAI_clause (BAI_core \"va'u\"))"), "{s}");
    assert!(s.contains("(NAI_clause (NAI_core \"nai\"))"), "{s}");

    // 意図的拡張の既存ピン(タグ+BO)も不変
    let s = parse_ok("mi pu bo klama");
    assert!(
        s.contains("(PU_clause (PU_core \"pu\")) (BO_clause (BO_core \"bo\"))"),
        "{s}"
    );
}

// ===== v0.108 レビュー対応: tense_mark 後置の排他化とタグ契約の KU 半分 =====

#[test]
fn 時制タグ_nai_bo_排他_v0_108() {
    // tense_mark の後置は NAI と BO の排他選択(sp1 ~ (NAI | BO))。
    // 組合せの「pu nai bo」は z0/z1/maftufa とも拒否する実測のため
    // 拒否ピン(v0.108 初版は受理していた z0 非整合の新規 OVER を解消)
    assert!(LojbanParser::parse(Rule::text, "mi pu nai bo klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi pu nai bo ge broda gi broda").is_err());
    // 逆順の「pu bo nai」も z0 実測 err と一致
    assert!(LojbanParser::parse(Rule::text, "mi pu bo nai klama").is_err());

    // 単独の「pu bo」は z0 は拒否するが本パーサーの意図的拡張(受容優先)。
    // 排他化後も BO 枝が生きていることの確認(木は不変)
    let s = parse_ok("mi pu bo klama");
    assert!(
        s.contains("(PU_clause (PU_core \"pu\")) (BO_clause (BO_core \"bo\"))"),
        "{s}"
    );

    // 単独の「pu nai」は不変
    let s = parse_ok("mi pu nai klama");
    assert!(
        s.contains("(PU_clause (PU_core \"pu\")) (NAI_clause (NAI_core \"nai\"))"),
        "{s}"
    );
}

#[test]
fn fa_と_bai_の_タグ契約_v0_108() {
    // z0 の tag_term 契約 = tag + (sumti | KU_elidible) の KU 半分(v0.108
    // レビュー)。FA 枝に (sp1 ~ KU_clause)? を追加し、「mi klama fa ku」等の
    // 明示 ku 閉鎖を受理(z0/z1/maf 実測 ok)。sumti と KU は排他
    let s = parse_ok("mi klama fa ku");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (FA_clause (FA_core \"fa\")) \
             (KU_clause (KU_core \"ku\")))))"
        ),
        "{s}"
    );
    // 裸 fa ku のフラグメント
    let s = parse_ok("fa ku");
    assert!(
        s.contains(
            "(fragment (terms (term (tagged (FA_clause (FA_core \"fa\")) \
             (KU_clause (KU_core \"ku\"))))))"
        ),
        "{s}"
    );
    // fa ku 項 + do 項(スロットに do が当たる読み)
    let s = parse_ok("mi klama fa ku do");
    assert!(s.contains("(KU_clause (KU_core \"ku\"))"), "{s}");
    assert!(s.contains("(KOhA_clause (KOhA_core \"do\"))"), "{s}");
    // NAI 後置 + KU
    let s = parse_ok("mi klama fa nai ku");
    assert!(
        s.contains(
            "(tagged (FA_clause (FA_core \"fa\")) (NAI_clause (NAI_core \"nai\")) \
             (KU_clause (KU_core \"ku\")))"
        ),
        "{s}"
    );
    // fi'a でも KU 閉鎖は同経路(z0/z1/maf 実測 ok)
    parse_ok("mi klama fa fi'a ku");

    // BAI 枝の sumti 省略化(裸 BAI 項)+ KU。「mi klama va'u」は
    // z0/z1/maf 実測 ok(裸 tag_term。tag … KU_elidible)
    let s = parse_ok("mi klama va'u");
    assert!(
        s.contains("(tail_terms (term (tagged (BAI_clause (BAI_core \"va'u\")))))"),
        "{s}"
    );
    // SE 変換 + KU(se va'u ku は v0.108 初版では拒否だった z0 整合の受理拡張)
    let s = parse_ok("mi klama se va'u ku");
    assert!(
        s.contains(
            "(tagged (SE_clause (SE_core \"se\")) (BAI_clause (BAI_core \"va'u\")) \
             (KU_clause (KU_core \"ku\")))"
        ),
        "{s}"
    );
    parse_ok("se va'u ku mi klama");

    // sumti+ku の二重閉鎖は z0/z1/maf とも拒否する実測のため拒否ピン
    // (tag 契約は (sumti | KU_elidible) の排他)
    assert!(LojbanParser::parse(Rule::text, "mi klama fa mi ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi klama va'u mi ku").is_err());
    // sumti 直後の ku 余りは二重 selbri 非文として z0 も拒否(実測)
    assert!(LojbanParser::parse(Rule::text, "mi klama ni'i klama").is_err());

    // 過剰受理チェック(既存形との受理の z0 一致): 裸 BAI の直後が項でも
    // タグが sumti を貪欲に取る(z0 の tag+sumti と同束の読み)
    let s = parse_ok("mi klama va'u do");
    assert!(
        s.contains(
            "(tagged (BAI_clause (BAI_core \"va'u\")) \
             (sumti (KOhA_clause (KOhA_core \"do\"))))"
        ),
        "{s}"
    );
    parse_ok("mi klama bai do");
    parse_ok("mi klama bai ku");
}

#[test]
fn fa_と_bai_の_タグ契約_木不変_v0_108() {
    // sumti あり・nai 無しの既存入力の木は不変(KU 追加・sumti 省略化は
    // オプション化のため)
    let s = parse_ok("mi klama fa lo zarci");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (FA_clause (FA_core \"fa\")) \
             (sumti (desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zarci\"))))))))))"
        ),
        "{s}"
    );
    let s = parse_ok("mi klama va'u lo zdani");
    assert!(
        s.contains(
            "(tagged (BAI_clause (BAI_core \"va'u\")) \
             (sumti (desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zdani\"))))))))"
        ),
        "{s}"
    );
    let s = parse_ok("se va'u mi klama");
    assert!(
        s.contains(
            "(term (tagged (SE_clause (SE_core \"se\")) \
             (BAI_clause (BAI_core \"va'u\")) \
             (sumti (KOhA_clause (KOhA_core \"mi\")))))"
        ),
        "{s}"
    );
    let s = parse_ok("fa nai mi klama");
    assert!(
        s.contains(
            "(term (tagged (FA_clause (FA_core \"fa\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (sumti (KOhA_clause (KOhA_core \"mi\")))))"
        ),
        "{s}"
    );

    // 経路変更の木不変: 「va'u ku」「va'u nai ku」は従来 tagged の
    // tense_tags+KU 枝が受けていたが、BAI 枝の KU 追加で本枝に移動する。
    // tense_tags/tense_mark は silent のため tagged の子の並びは同一
    // (exact-tree ピン)
    let s = parse_ok("mi klama va'u ku");
    assert!(
        s.contains(
            "(tail_terms (term (tagged (BAI_clause (BAI_core \"va'u\")) \
             (KU_clause (KU_core \"ku\")))))"
        ),
        "{s}"
    );
    let s = parse_ok("mi klama va'u nai ku");
    assert!(
        s.contains(
            "(tagged (BAI_clause (BAI_core \"va'u\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (KU_clause (KU_core \"ku\")))"
        ),
        "{s}"
    );
    // selbri 前置タグ(ni'i bo 等)が裸 BAI 項に食われないことの確認
    // (tagged BAI 枝の selbri 否定先読みガード)
    let s = parse_ok("mi ni'i bo klama");
    assert!(
        s.contains(
            "(bridi_tail (selbri (BAI_clause (BAI_core \"ni'i\")) \
             (BO_clause (BO_core \"bo\")) \
             (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))"
        ),
        "{s}"
    );
}

#[test]
fn 裸タグ項_追ピン_v0_108() {
    // 項位置の裸 FA タグ(z0/z1/maf 実測 ok)。tagged(FA) 項 + bridi_tail
    let s = parse_ok("mi fa klama");
    assert!(
        s.contains(
            "(terms (term (sumti (KOhA_clause (KOhA_core \"mi\")))) \
             (term (tagged (FA_clause (FA_core \"fa\"))))) \
             (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))))"
        ),
        "{s}"
    );

    // mex 演算子の ji+nai(v0.101 の mex_conn 経路。z0/z1/maf 実測 ok)
    let s = parse_ok("li pa ji nai re");
    assert!(
        s.contains(
            "(mex (number (PA_clause (PA_core \"pa\"))) \
             (JOI_clause (JOI_core \"ji\")) \
             (NAI_clause (NAI_core \"nai\")) \
             (number (PA_clause (PA_core \"re\"))))"
        ),
        "{s}"
    );

    // 裸タグ+selbri の文(z0/z1/maf 実測 ok)。実木は fragment ではなく
    // sentence(terms の tagged 項 + bridi_tail)として構築される
    let s = parse_ok("fa broda");
    assert!(
        s.contains(
            "(terms (term (tagged (FA_clause (FA_core \"fa\"))))) \
             (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))))) \
             (tail_terms \"\"))"
        ),
        "{s}"
    );
}

#[test]
fn 裸tanru_bo接続_v0_109() {
    // GAP_裸tanru_BO接続(v0.109 解消)。tanru_link に BO 単独の選択肢を追加。
    // z0 実測(zantufa-0.9999.js): 「mi klama bo cadzu」は selbri_6 =
    // tanru_unit (BO tanru_unit)* の tanru 接続(gihek/bridi_tail ではない)。
    // 本実装は tanru 繰り返しの平準形(z0 は selbri_6 ノード。既知クラスの
    // 木形状差異で、受理・読みは同一)
    let s = parse_ok("mi klama bo cadzu");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))) \
             (BO_clause (BO_core \"bo\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"cadzu\"))))"
        ),
        "{s}"
    );

    // BO 連鎖(z0 も selbri_6 の繰り返しで受理)
    let s = parse_ok("mi klama bo cadzu bo bajra");
    assert_eq!(s.matches("BO_core \"bo\"").count(), 2, "{s}");
    assert!(s.contains("BRIVLA_core \"bajra\""), "{s}");

    // 描述内の tanru BO(z0 実測 ok)
    let s = parse_ok("mi viska lo broda bo brode ku");
    assert!(
        s.contains("(BO_clause (BO_core \"bo\"))")
            && s.contains("BRIVLA_core \"broda\"")
            && s.contains("BRIVLA_core \"brode\""),
        "{s}"
    );
    parse_ok("lo broda bo brode ku barda");

    // gihek との組合せ(z0 実測 ok): 2個目の bridi_tail の selbri が BO で継続
    parse_ok("mi broda gi'e brode bo brodi");
    // co_tail 内の tanru BO(z0 実測 ok)
    parse_ok("mi klama co cadzu bo bajra");
    // 項接続の BO は ek_joik の既存経路(不変)
    parse_ok("mi joi bo do klama");

    // NAhE + BO は z0 も拒否(s_marks の後の tanru は単位で始まる必要がある)
    assert!(LojbanParser::parse(Rule::text, "mi na'e bo broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "na'e bo broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "to'e bo broda").is_err());
}

#[test]
fn gihekによるjoi接続_v0_109() {
    // GAP_JOIによるselbri接続(v0.109 解消)。gihek_link に gihek_joik
    // (JOI/BIhI)を追加。z0 実測: gihek は NA? SE? GIhA のみだが、zantufa の
    // joik は総称接続詞(JOI+JA系+BIhI)を selbri_4/selbri_5 の接続で取るため
    // 「mi broda joi brode」が受理される。本実装は camxes 系 gihek 拡張に倣い
    // bridi_tail 連結部で受ける(木形状差異 = 既知クラス: z0 は selbri_4 の
    // joik で2つの selbri を結ぶが、本実装は gihek_link で2つの bridi_tail を
    // 結ぶ。受理・読みは同一)
    let s = parse_ok("mi broda joi brode");
    assert!(
        s.contains(
            "(bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))))) \
             (tail_terms \"\") (JOI_clause (JOI_core \"joi\")) \
             (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"brode\"))))) \
             (tail_terms \"\")))"
        ),
        "{s}"
    );

    // JOI 系の各語(gap_tracker GAP_JOIによるselbri接続 と同一入力。
    // いずれも z0/z1/maftufa 実測 ok)
    for input in [
        "mi broda joi brode",
        "mi broda jo'e brode",
        "mi broda fa'u brode",
        "mi broda ku'a brode",
        "mi broda johu brode",
        "mi broda jo'u brode",
        // 接続詞疑問 ji(v0.108 で JOI_core 収録。副次効果で緑化)
        "mi broda ji brode",
    ] {
        parse_ok(input);
    }

    // NAI 後置 / BO 後置 / SE 変換(z0/z1/maf 実測 ok。CLL 14 の
    // 接続詞 NAI・短スコープ BO・SE 変換)
    parse_ok("mi broda joi nai brode");
    parse_ok("mi broda joi bo brode");
    parse_ok("mi broda joi nai bo brode");
    parse_ok("mi broda se joi brode");
    // BIhI(GAhO 両端 / NAI 後置 / SE 前置。z0/z1/maf 実測 ok)
    parse_ok("mi broda bi'i brode");
    // BIhI_core の残り語形 mi'i(単独形。z0/z1/maf 実測 ok)
    parse_ok("mi broda mi'i brode");
    parse_ok("mi broda ga'o bi'i ke'i brode");
    parse_ok("mi broda ke'i bi'o ga'o brode");
    parse_ok("mi broda bi'i nai brode");
    parse_ok("mi broda se bi'i brode");
    // 連鎖と混合(z0 実測 ok)
    parse_ok("mi broda joi brode ji brodi");
    parse_ok("mi broda ji brode joi brodi");
    parse_ok("mi broda joi brode gi'e brodi");
    parse_ok("mi broda joi brode joi brodi");
    parse_ok("mi broda joi brode bo brodi");
    parse_ok("mi broda joi brode co brodi");
    parse_ok("mi broda jo'e brode gi'e brodi");
    parse_ok("mi broda ku'a brode bo brodi");
    // 連結詞直後の自由修飾語(gihek_free。z0 実測 ok)
    parse_ok("mi broda joi brode .ui brodi");
    parse_ok("mi broda joi .ui brode");
    parse_ok("mi broda joi brode vau");

    // A 系は z0/z1/maf とも拒否(ek_joik との差分。項接続の A は別位置)
    assert!(LojbanParser::parse(Rule::text, "mi broda e brode").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi broda a brode").is_err());
    // JA 単独の裸形は tanru_link の JA 経路が先に消費されるため gihek には
    // 到達しない(「mi broda je brode」は tanru の木のまま = 下の不変ピン)
    // vocative は連結詞の直後に置けない(z0/z1/maf 実測 err。gihek_free)
    assert!(LojbanParser::parse(Rule::text, "mi broda joi ju'i brode").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi broda je ju'i brode").is_err());
    // DOhU 明示閉鎖の vocative は受理(z0/z1/maf 実測 ok)
    parse_ok("mi broda joi ju'i dohu brode");
    // gi は GIhA ではない(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "mi broda joi brode gi brodi").is_err());
    // 後続の ku は宙に浮く(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "mi broda je brode ku brodi").is_err());
}

#[test]
fn gihek拡張_既存不変ピン_v0_109() {
    // GIhA 経路の木は不変(gihek_link の第1枝を維持)
    let s = parse_ok("mi klama gi'e cadzu");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    assert!(s.contains("(GIhA_clause (GIhA_core \"gi'e\"))"), "{s}");
    // guhek(gu'e … gi …)経路も不変
    let s = parse_ok("mi gu'e broda gi broda");
    assert!(
        s.contains("GUhA_core \"gu'e\"") && s.contains("GI_core \"gi\""),
        "{s}"
    );
    // 項接続(ek_joik)の木は不変
    let s = parse_ok("mi joi do klama");
    assert!(s.contains("JOI_core \"joi\""), "{s}");
    // JA の tanru 接続の木は不変(je は gihek に到達しない)
    let s = parse_ok("mi broda je brode");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))) \
             (JA_clause (JA_core \"je\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"brode\"))))"
        ),
        "{s}"
    );
    // GIhA+BO(v0.95)の木は不変
    let s = parse_ok("mi nelci gi'e bo citka");
    assert!(s.contains("GIhA_core") && s.contains("BO_core"), "{s}");
    // gihek 直後の ba'e(v0.92)の木は不変
    let s = parse_ok("mi klama gi'e ba'e cadzu");
    assert!(s.contains("GIhA_core \"gi'e\""), "{s}");
    // 連結詞直後の感情標識(v0.92)。v0.109 で free* → gihek_free(silent)に
    // 置換したため、旧実装の free ラッパは消失する(非 silent の free_unit は
    // gihek_free_unit の参照経路で残り、bridi_tail の直接の子に平準化される。
    // tail free 側は tail_terms > free > free_unit と free ラッパが残る
    // 既知クラスの木形状差異)。新形状の木ピン(v0.109 レビュー対応)
    let s = parse_ok("mi klama gi'e .ui cadzu");
    assert!(
        s.contains(
            "(GIhA_clause (GIhA_core \"gi'e\")) \
             (free_unit (ui_free (UI_clause (UI_core \"ui\")))) \
             (bridi_tail (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"cadzu\")))))"
        ),
        "{s}"
    );
    // free ラッパのみ消失(gihek_free/gihek_free_unit は silent)
    assert!(!s.contains("(free ("), "{s}");
    // 裸 vocative は連結詞の直後に置けない(z0/z1/maf 実測 err。
    // v0.92 からの過剰受容を v0.109 で gihek_free により解消)
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e ju'i cadzu").is_err());
    // DOhU 明示閉鎖なら受理(z0/z1/maf 実測 ok)
    parse_ok("mi klama gi'e ju'i dohu cadzu");
}

#[test]
fn jai_se変換タグ_v0_109() {
    // GAP_jai_se変換タグ(v0.109 解消)。JAI 枝に (SE | NAhE)+ 変換タグの
    // 選択肢を追加。z0 実測(zantufa-0.9999.js): 「mi jai se gau broda」は
    // tanru_unit_1 = JAI_clause(tag) tanru_unit_1 の tag が SE+BAI を取る形
    // (JAI ノード内に [JAI jai][SE se][BAI gau] と平準化)。
    // 本実装は SE/NAhE を JAI の直下の兄弟ノードに平準化する(既知クラスの
    // 木形状差異で、受理・読みは同一)
    let s = parse_ok("mi jai se gau broda");
    assert!(
        s.contains(
            "(tanru_unit (JAI_clause (JAI_core \"jai\")) \
             (SE_clause (SE_core \"se\")) \
             (BAI_clause (BAI_core \"gau\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))))"
        ),
        "{s}"
    );
    // 尾項+描述(gap_tracker GAP_jai_se変換タグ と同一入力。z0/z1/maf 実測 ok)
    parse_ok("mi jai se gau klama lo zdani");

    // 変換タグの前置は SE/NAhE の連続(z0 実測: se gau / na'e gau /
    // se na'e gau / na'e se gau / se pu / se ta'i は受理)
    parse_ok("mi jai se ta'i broda");
    parse_ok("mi jai na'e gau broda");
    parse_ok("mi jai se na'e gau broda");
    parse_ok("mi jai na'e se gau broda");
    parse_ok("mi jai se pu broda");
    // co との組合せ(z0 実測 ok)
    parse_ok("mi jai se gau broda co brodi");

    // z0 も拒否する形(実測 err): NA/JAhA は変換タグの前置になれない。
    // s_marks 全体ではなく (SE | NAhE)+ とする理由
    assert!(LojbanParser::parse(Rule::text, "mi jai na gau broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi jai ja'a gau broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi jai se ja'a gau broda").is_err());
    // GA はタグではない(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "mi jai se go broda").is_err());
    // タグだけのフラグメントは z0 も拒否(z0=0/z1=1/maf=1 の参照分裂のため
    // gap_tracker には含まれない。z0 整合で拒否を維持)
    assert!(LojbanParser::parse(Rule::text, "jai se gau").is_err());

    // 既存木不変: SE 省略形と SE+BRIVLA 形は第1/第2枝が先に一致して木は不変
    let s = parse_ok("mi jai frili");
    assert!(
        s.contains(
            "(tanru_unit (JAI_clause (JAI_core \"jai\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"frili\"))))"
        ),
        "{s}"
    );
    let s = parse_ok("mi jai gau broda");
    assert!(
        s.contains(
            "(tanru_unit (JAI_clause (JAI_core \"jai\")) \
             (BAI_clause (BAI_core \"gau\")) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\"))))"
        ),
        "{s}"
    );
    // 「jai se broda」は tanru_unit の SE 前置 brivla 経路(z0 も同じ木:
    // SE は tanru_unit_1 側)
    let s = parse_ok("mi jai se broda");
    assert!(
        s.contains(
            "(tanru_unit (JAI_clause (JAI_core \"jai\")) \
             (tanru_unit (SE_clause (SE_core \"se\")) \
             (BRIVLA_clause (BRIVLA_core \"broda\"))))"
        ),
        "{s}"
    );
}

#[test]
fn free後のco転換selbri継続_v0_109() {
    // GAP_free後のco転換selbri継続(v0.109 解消)。tanru_post に co_post 枝を
    // 追加。z0 実測(zantufa-0.9999.js): 「farlu ju'i co cnita」は
    // post_clause(free ju'i, selbri co cnita, DOhU elided)。
    // 本実装は tanru 繰り返しの平準形(CO_clause と tanru が tanru の直接の
    // 子として現れる。v0.103 と同じ既知クラスの木形状差異で、受理・読みは同一)
    let s = parse_ok("farlu ju'i co cnita");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"farlu\"))) \
             (free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) \
             (CO_clause (CO_core \"co\")) \
             (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\")))))"
        ),
        "{s}"
    );
    // gap_tracker と同一入力の残り2形(z0/z1/maf 実測 ok)
    let s = parse_ok(".oi ta ca'o farlu ju'i co cnita");
    assert!(
        s.contains("CO_core \"co\"") && s.contains("BRIVLA_core \"cnita\""),
        "{s}"
    );
    parse_ok("mi farlu ju'i co cnita");

    // 複数 free の後の co(z0 実測 ok)
    parse_ok("farlu ju'i .ui co cnita");
    // 感情標識の後の co(z0 実測 ok。既存の tail free + co_tail 経路から
    // tanru_post の co 継続に変わる新規受理)
    parse_ok("mi klama .ui co broda");
    // co 後の SE 前置(z0 実測 ok)
    parse_ok("farlu ju'i co se cnita");
    // NAhE は tanru_unit の前置で受理(z0 実測 ok)
    parse_ok("farlu ju'i co na'e cnita");
    // 抽象の継続(z0 実測 ok)
    parse_ok("farlu ju'i co nu broda kei");
    // co 連鎖(z0 実測 ok)
    parse_ok("farlu ju'i co cnita co brodi");
    // 連鎖途中の co+SE(z0 実測 ok)
    parse_ok("farlu ju'i co cnita co se brodi");
    // co 後の JA tanru 連接(z0 実測 ok)
    parse_ok("farlu ju'i co brode je brodi");
    // co 後の BO 連接(z0 実測 ok)
    parse_ok("farlu ju'i co broda bo brodi");
    // 継続後の gihek(z0 実測 ok)
    parse_ok("farlu ju'i co cnita gi'e brodi");
    // 継続後の free は tail free へ(z0 実測 ok)
    parse_ok("farlu ju'i co cnita .ui");

    // z0 も拒否する形(実測 err)
    // s_marks 全体ではなく SE 前置まで: JAhA/NA は co の直後に置けない
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co ja'a cnita").is_err());
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co na cnita").is_err());
    // DOhU 後置は不可(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co cnita dohu").is_err());
    // ku 後置は不可(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co cnita ku").is_err());
    // co 後の gek/gi は不可(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co broda gi broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "farlu ju'i co cnita gi broda").is_err());
    // dangling な co(z0 実測 err)
    assert!(LojbanParser::parse(Rule::text, "mi klama ju'i co").is_err());

    // 既存木不変: co を含まない free 継続と、free で終わる入力は
    // v0.103 の木のまま(フォールバック設計の維持)
    let s = parse_ok("farlu ju'i cnita");
    assert!(
        s.contains(
            "(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"farlu\"))) \
             (free (free_unit (vocative (COI_clause (COI_core \"ju'i\"))))) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"cnita\"))))"
        ),
        "{s}"
    );
    let s = parse_ok("klama .ui");
    assert!(
        s.contains("(tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\"))))"),
        "{s}"
    );
    assert!(
        s.contains("tail_terms (free (free_unit (ui_free (UI_clause (UI_core \"ui\")))))"),
        "{s}"
    );
    // 通常の co_tail(selbri 直下)は free を伴わないため不変
    let s = parse_ok("mi klama co broda");
    assert!(
        s.contains(
            "(selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"klama\")))) \
             (CO_clause (CO_core \"co\"))"
        ),
        "{s}"
    );
}

// =====================================================================
// v0.110: バッチ3 GAP 解消(レタル接頭融合語 / VUhO 関係節共有 /
// zoi 区切り語のピリオド正規化 / 項レベル ke グループ / gihek の
// (NA? SE?) 前置 / 呼格+sumti 継続)
// =====================================================================

#[test]
fn レタル接頭融合語_v0_110() {
    // GAP_レタル接頭lujvo_byklesi(v0.110 解消)。
    // z0 実測(zantufa-0.9999.js): 「lo byklesi ku」は brivla ではなく、
    // by+brivla の無ポーズ隣接(cmavo の post_word = pause /
    // !nucleus lojban_word 経路)により、sumti_tail 内で
    // lerfu_string(by) + BOI(省略) + selbri(klesi) として解析される
    // (maftufa も同読み。形態論的な単一 brivla ではない)。
    // 本実装は無ポーズ隣接を一般にサポートしないため、融合語トークンを
    // tanru_unit の BY_prefix 枝(BY_prefix + BRIVLA_clause)で受理する
    // (木形状差異 = 既知クラス。z0 は lerfu 項+selbri の2要素、
    // 本実装は tanru_unit 1要素。受理・読みは同一)
    let s = parse_ok("lo byklesi ku");
    assert!(
        s.contains(
            "(desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BY_prefix \"by\") \
             (BRIVLA_clause (BRIVLA_core \"klesi\"))))) \
             (KU_clause (KU_core \"ku\")))"
        ),
        "{s}"
    );
    // gap_tracker GAP_レタル接頭lujvo_byklesi と同一入力(z0/z1/maf 実測 ok)
    let s = parse_ok("mi byklesi");
    assert!(
        s.contains(
            "(bridi_tail (selbri (tanru (tanru_unit (BY_prefix \"by\") \
             (BRIVLA_clause (BRIVLA_core \"klesi\"))))) (tail_terms \"\"))"
        ),
        "{s}"
    );
    parse_ok("lo cyklesi ku");
    parse_ok("mi klama lo byklesi ku");
    // CVCy_lujvo を後続に取る形(z0 実測 ok。jamynai = jam+y+nai)
    parse_ok("lo dyjamynai ku");
    // tanru 継続・be 連結(z0 実測 ok)
    parse_ok("lo byklesi broda");
    parse_ok("mi byklesi broda");
    parse_ok("lo byklesi be do ku");
    parse_ok("lo byklesi je broda ku");
    // 大文字小文字(z0 実測 ok)
    parse_ok("lo BYklesi ku");
    // 全子音レタル(z0 実測 ok: by/cy/dy/fy/gy/jy/ky/ly/my/ny/py/ry/sy/ty/vy/xy/zy)
    for l in [
        "by", "cy", "dy", "fy", "gy", "jy", "ky", "ly", "my", "ny", "py", "ry", "sy", "ty", "vy",
        "xy", "zy",
    ] {
        parse_ok(&format!("lo {l}klesi ku"));
    }

    // 過剰受理チェック(z0 交叉・実測):
    // - 母音レタルは前置しない(「lo abu ku」は z0 も拒否。v0.106 の
    //   BY_core ガードと整合。既存の拒否ピンは不変)
    assert!(LojbanParser::parse(Rule::text, "lo abu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo ebu ku").is_err());
    // - 後続が brivla でない融合語は z0 も拒否(bu'u は FAhA cmavo、
    //   ku は KU cmavo のため selbri が始まらない)
    assert!(LojbanParser::parse(Rule::text, "lo cybu'u ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo byku ku").is_err());
    // - 既存 lujvo の受理は不変: BRIVLA 枝を先に試すため、融合語トークンが
    //   有効な brivla である場合は既存経路が優先される
    let s = parse_ok("lo klama ku");
    assert!(s.contains("(BRIVLA_clause (BRIVLA_core \"klama\"))"), "{s}");
    // byfyklesi / bycyklesi 型の多レタル接頭は z0 が受理するが(by+fy+klesi 等
    // の lerfu_string 連鎖)、本実装は単一レタル接頭のみのため拒否
    // (サブセット。残差記録)
    assert!(LojbanParser::parse(Rule::text, "lo byfyklesi ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo bycyklesi ku").is_err());
    // - CVCV 4字/CVCVV 5字の stress なし短形残部は !(cvcv_short_tail) ガードで
    //   排除(BRIVLA_core がこの短形を fuhivla/lujvo に誤マッチする実装上の
    //   性質の防波。レタル接頭×残部語形マトリクス 52 行+拡張 15 行で
    //   z0/z1/maf 交叉実測、短形残部は全形が参照 3 種とも拒否):
    //   「lo bynaku ku」「lo bykuku ku」「lo bynunu ku」「lo byzozo ku」
    //   「lo bypapa ku」「lo bydada ku」「lo cykuku ku」「lo zykuku ku」
    //   「mi bykuku」「lo bykukai ku」等は z0 実測 err(ガードなしの旧実装は
    //   fuhivla 誤読の新規 OVER)で z0 整合の拒否(誤読受容より誠実な拒否が正。
    //   v0.106 の lo abu ku 処置と同型の判断。parent 81a54c2 で全形 err の
    //   確認済み=本変更で新規に導入した受理を取り消す退行なしの縮小)
    for s in [
        "lo bynaku ku",
        "lo bykuku ku",
        "lo bynunu ku",
        "lo byzozo ku",
        "lo bypapa ku",
        "lo bydada ku",
        "lo cykuku ku",
        "lo zykuku ku",
        "mi bykuku",
        "lo bykukai ku",
        "lo bykule ku",
        "lo bykula ku",
        "lo bykubu ku",
        "lo byvovu ku",
        "lo bykuku broda",
        "mi bykuku broda",
    ] {
        assert!(LojbanParser::parse(Rule::text, s).is_err(), "{s}");
    }
    //   ガードは 5 字以上の有効 brivla を壊さない(word_boundary 付き短形の
    //   み排除。実測ピン):
    //   - gismu 残部(klesi 5字 CVCCV 等は1子音開始のため短形に不一致)
    //   - 「kukla」(CVCCV gismu 形)「kuklama」以外の正規 lujvo 残部
    //     (nunkapi / kulesi… は z0 実測で kukla は受理、kulesi は拒否。
    //     現行ガードは両方とも不干渉=現状維持)
    //   - CVCy lujvo(kukybu'e / jamynai)
    parse_ok("lo bykukla ku");
    parse_ok("lo bynunkapi ku");
    parse_ok("lo bykukybu'e ku");
    parse_ok("lo bynaselci ku");
    parse_ok("lo byduduki ku");
    //   残差(記録。次バッチ候補):
    //   - 6字以上の無記名 CVCVCV 連鎖残部(「lo bykukula ku」「lo bykatane ku」
    //     型)は z0 が拒否するが本実装は fuhivla 誤マッチで受理(既存の
    //     BRIVLA_core 緩さの残存。接頭なし「lo kukula ku」は pre-existing
    //     OVER。一括排除は z0 が受理する 6 字形(byduduki 型)を壊すため
    //     形態論レベルの stress 判定が要る)
    parse_ok("lo bykukula ku");
    //   - z0 ok / 本実装 err の項分離読み残差: 「lo bydudu ku」「mi bydudu」
    //     「lo bydudu klama」は z0 では dudu が GOhA cmavo(du)の無ポーズ
    //     隣接の項分離読み(融合語読みではない)で受理される。本実装は
    //     項分離読み未対応のため拒否(短形 CVCV の融合語読みを取らない
    //     z0 と同じ拒否対象。読み経路の追加が次バッチ課題)
    assert!(LojbanParser::parse(Rule::text, "lo bydudu ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi bydudu").is_err());
    assert!(LojbanParser::parse(Rule::text, "lo bydudu klama").is_err());
    // - 形態論ストレステスト(z0 交叉・実測)で確認した既知クラス:
    //   brivla+レタル接頭融合語の無ポーズ隣接(tanru 2単位目・gihek 後・
    //   selbri 直後)は z0 が拒否するが本実装は受容(受容優先の既知クラス。
    //   v0.95 タグ+BO / v0.107 mi pu bo ge と同型。非正規字形で実害なし)
    parse_ok("mi klama byklesi");
    parse_ok("mi klesi byklesi");
    parse_ok("mi klama gi'e byklesi");
    //   受理一致で読みが異なる形: 「mi suta byklesi」は z0 では su(消去)+ta
    //   (KOhA 項)への形態論分割の読み(su_clause が intro に現れる)、本実装は
    //   tanru(suta, by+klesi)。受理一致の既知読み差異クラス
    parse_ok("mi suta byklesi");
    //   z0 ok / 本実装 err の残差(GAP 方向。次バッチ候補として記録):
    //   関係節が続く形(z0 は sumti_tail の埋め込み lerfu 項+selbri+relative
    //   の読み)、レタル+タグ cmavo の隣接(z0 は by 項+BAI/PU タグ+selbri の
    //   読み)は項分離読みの実装が要るため未対応
    assert!(LojbanParser::parse(Rule::text, "lo byklesi noi broda ku").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi byta'e klama").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi byca klama").is_err());
    drop(s);
    // 通常のレタル項(空白あり)は既存経路のまま(BY_clause)
    let s = parse_ok("lo by klesi ku");
    assert!(s.contains("(BY_clause (BY_core \"by\"))"), "{s}");
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn vuho後の関係節共有_v0_110() {
    // GAP_VUhO後の関係節共有(v0.110 解消)。
    // z0 実測(zantufa-0.9999.js): sumti = sumti_1 (VUhO_clause relative_clauses)?
    // の形で、vu'o の直後は relative_clauses(noi/poi/voi または GOI)のみ。
    // 「mi viska lo broda vu'o noi mi klama」は sumti の
    // (VUhO relative_clauses)? スロットで受理される。
    // なお z0 は「vu'o + sumti」の項連結を持たない(「vu'o mi」は z0 実測 err)
    // が、本実装の既存の VUhO 項連結枝は pre-existing の受理のため維持
    // (既知 OVER 差分。ok→err 回帰の防止)
    let s = parse_ok("mi viska lo broda vu'o noi mi klama");
    assert!(
        s.contains(
            "(desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\")))))) \
             (VUhO_clause (VUhO_core \"vu'o\")) \
             (relative_clauses (relative_clause (NOI_clause (NOI_core \"noi\"))"
        ),
        "{s}"
    );
    // gap_tracker GAP_VUhO後の関係節共有 と同一入力(z0/z1/maf 実測 ok)
    parse_ok("mi viska lo broda vuho noi mi klama");
    // poi / voi 形(z0 実測 ok)
    parse_ok("mi viska lo broda vu'o poi mi klama");
    parse_ok("mi viska lo broda vu'o voi mi klama");
    // GOI 形の関係節(z0 実測 ok)
    parse_ok("mi viska lo broda vu'o pe mi");
    // 既存の VUhO 項連結は不変(z0 は拒否の既知 OVER 差分。受理維持)
    parse_ok("mi viska lo broda vu'o mi");
    // 連結後の関係節共有は z0 も拒否(単一スロット)
    assert!(LojbanParser::parse(
        Rule::text,
        "mi viska lo broda vu'o noi mi klama vu'o poi brode"
    )
    .is_err());
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 項レベルのkeグループ_v0_110() {
    // GAP_ke_group内の項(v0.110 解消)。
    // z0 実測(zantufa-0.9999.js): term_2 = KE_clause term+ KEhE_elidible の
    // 項グループで、「mi klama ke lo zdani broda ke'e」は tail_terms 内の
    // term(KE + [lo zdani broda(描述、selbri は tanru zdani+broda)] + ke'e)
    // として受理される。既存の tanru_unit 側 ke_group(KE selbri KEhE?)は
    // 「ke broda ke'e」型の selbri グループを先に処理するため木は不変
    let s = parse_ok("mi klama ke lo zdani broda ke'e");
    assert!(
        s.contains(
            "(term (KE_clause (KE_core \"ke\")) \
             (term (sumti (desc (LE_clause (LE_core \"lo\")) \
             (selbri (tanru (tanru_unit (BRIVLA_clause (BRIVLA_core \"zdani\"))) \
             (tanru_unit (BRIVLA_clause (BRIVLA_core \"broda\")))))))) \
             (KEhE_clause (KEhE_core \"ke'e\")))"
        ),
        "{s}"
    );
    // KEhE 省略(z0 実測 ok)
    parse_ok("mi klama ke lo zdani broda");
    // 複数項(z0 実測 ok)
    parse_ok("mi klama ke lo zdani lo broda ke'e");
    // KOhA 項(z0 実測 ok)
    parse_ok("mi klama ke do ke'e");
    // 明示 ku 付き描述(z0 実測 ok)
    parse_ok("mi klama ke lo zdani ku ke'e");
    // 過剰受理チェック(z0 交叉・実測): グループ内が sumti で始まらない形、
    // および ke'e の後に続く形は z0 も拒否
    assert!(LojbanParser::parse(Rule::text, "mi klama ke mi broda ke'e").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi klama ke lo zdani broda ke'e brode").is_err());
    // 既存の tanru_unit 側 ke_group(selbri グループ)の木は不変
    let s = parse_ok("mi klama ke broda ke'e");
    assert!(
        s.contains("(tanru_unit (ke_group (KE_clause (KE_core \"ke\")) (selbri"),
        "{s}"
    );
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn gihekの_na_se前置_v0_110() {
    // GAP(gihek の (NA? SE?) 前置。v0.110 解消)。
    // z0 実測(zantufa-0.9999.js): gihek = NA? SE? GIhA、joik = GAhO? NA? SE?
    // JOI GAhO? で前置スロットを持つ。GIhA 形の z0 の木は「na」を前の
    // bridi_tail の tail_terms の裸 NA 項(na ku の KU 省略形)として取るが、
    // 本実装は gihek_link の前置スロットで受ける(木形状差異 = 既知クラス。
    // 受理・読みは同一)。JOI/BIhI 形は z0 も selbri_4 の joik(NA? SE? JOI)
    // で前置を取る
    let s = parse_ok("mi broda na gi'e brode");
    assert!(
        s.contains("(NA_clause (NA_core \"na\")) (GIhA_clause (GIhA_core \"gi'e\"))"),
        "{s}"
    );
    parse_ok("mi broda se gi'e brode");
    parse_ok("mi broda na se gi'e brode");
    parse_ok("mi broda na gi'a brode");
    parse_ok("mi broda se gi'a brode");
    // JOI/BIhI 枝への (NA? SE?) 前置(z0/z1/maf 実測 ok)
    parse_ok("mi broda na joi brode");
    parse_ok("mi broda na se joi brode");
    parse_ok("mi broda na bi'i brode");
    parse_ok("mi broda na se bi'i brode");
    // GAhO 両端との組合せ(z0 実測 ok)
    parse_ok("mi broda ga'o na bi'i ke'i brode");
    // 連鎖(z0 実測 ok)
    parse_ok("mi broda na gi'e brode gi'a brodi");
    // NAI 後置との組合せ(z0 実測 ok。nai は既存の後置スロットで受ける)
    parse_ok("mi broda na gi'e nai brode");

    // 含めないもの(z0 交叉・実測):
    // - NAI 前置(nai joi / nai gi'a)は z0/z1 のみ受理・maftufa は拒否の
    //   緩受理のため含めない(既知クラス維持)
    assert!(LojbanParser::parse(Rule::text, "mi broda nai joi brode").is_err());
    // - SE と NA の逆順は z0 も拒否(前置の順序は NA→SE 固定)
    assert!(LojbanParser::parse(Rule::text, "mi broda se na gi'e brode").is_err());
    // - NA 前置+NAI(na nai gi'e)は z0 が na 項+nai(UI free)+gihek の別読み
    //   で受理するが、本実装は未対応のまま(残差記録)
    assert!(LojbanParser::parse(Rule::text, "mi broda na nai gi'e brode").is_err());
    // - 前置だけの宙吊り接続は拒否(z0 も拒否)
    assert!(LojbanParser::parse(Rule::text, "mi broda na gi'e").is_err());
    // 既存不変: 前置なし GIhA の木は不変
    let s = parse_ok("mi klama gi'e cadzu");
    assert!(s.contains("(GIhA_clause (GIhA_core \"gi'e\"))"), "{s}");
    // 既存不変: na ku 項(明示 ku)は tail_terms 側のまま
    let s = parse_ok("mi broda na ku gi'e brode");
    assert!(s.contains("KU_clause"), "{s}");
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

#[test]
fn 呼格と_sumti引数の継続_v0_110() {
    // GAP(呼格+sumti 継続。v0.110 解消)。
    // z0 実測(zantufa-0.9999.js): free の第3枝は vocative sumti? DOhU_elidible
    // で、sumti は代名詞も取れる。「farlu ju'i do cnita」は
    // free(vocative ju'i, sumti do, DOhU 省略) + selbri cnita の tanru 継続、
    // 「mi klama gi'e ju'i do cadzu」は gihek の post_clause の free で受理。
    // 注意(z0 交叉・実測): 連結部の cmevla 引数形
    // 「gi'e ju'i la alen. cadzu」は z0/z1/maf とも拒否のため、
    // 連結部では KOhA 引数のみ許容(既存の拒否と整合)
    let s = parse_ok("farlu ju'i do cnita");
    assert!(
        s.contains(
            "(vocative (COI_clause (COI_core \"ju'i\")) \
             (KOhA_clause (KOhA_core \"do\")))"
        ),
        "{s}"
    );
    // gap_tracker の対象文と同型(z0/z1/maf 実測 ok)
    parse_ok("mi klama gi'e ju'i do cadzu");
    parse_ok("mi klama gi'e ju'i mi cadzu");
    parse_ok("mi klama gi'e ju'i ti cadzu");
    // NAI 後置+KOhA 引数(z0 実測 ok)
    parse_ok("mi klama gi'e ju'i nai do cadzu");
    parse_ok("mi klama ju'i nai do");
    // DOhU 明示閉鎖との併用(z0 実測 ok)
    parse_ok("mi klama gi'e ju'i do dohu cadzu");
    // 尾項位置の呼格+sumti(z0 実測 ok)
    parse_ok("mi klama ju'i do");
    // tanru 継続(z0 実測 ok)
    parse_ok("mi klama ju'i do cadzu");

    // 既存の拒否は不変(z0 交叉・実測):
    // - 裸 vocative(引数なし)は連結部で拒否
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e ju'i cadzu").is_err());
    // - cmevla 引数の DOhU 省略形は連結部で拒否(z0/z1/maf 実測 err)
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e ju'i la alen. cadzu").is_err());
    // - 連結後の bridi_tail が無い形は拒否(z0 も拒否)
    assert!(LojbanParser::parse(Rule::text, "mi klama gi'e ju'i do").is_err());
    // 既存の受理は不変: DOhU 明示閉鎖形
    parse_ok("mi klama gi'e ju'i dohu cadzu");
    // cmevla 引数の vocative は tanru 継続位置では従来どおり受理
    parse_ok("farlu ju'i la alen. cnita");
    // 語形不正は引き続きエラー
    assert!(lojban::parse("qqq").is_err());
}

// ---------------------------------------------------------------------
// v0.111: KOhA の ce'u(ラムダ変数)/ zi'o(消去項)と CLL 標準 UI の欠落語。
// ユーザー報告文「.i sy mintu lo purdykurji lo ka ma kau tarmi ce'u .i clani
// kurfa gi'e plita gi'e se kojna lo xance jo'u lo jamfu」の失敗点
// (lo ka ma kau tarmi ce'u → KOhA_core に ce'u が無い)の回帰。
// z0/z1/maf 交叉は全形 ok(STATUS v0.111 参照)
// ---------------------------------------------------------------------
#[test]
fn ユーザー報告文_ka_内_ceu() {
    // 報告文全体の受理
    parse_ok(
        ".i sy mintu lo purdykurji lo ka ma kau tarmi ce'u .i clani kurfa \
         gi'e plita gi'e se kojna lo xance jo'u lo jamfu",
    );
    // 報告の失敗点の単文
    let s = parse_ok("sy mintu lo purdykurji lo ka ma kau tarmi ce'u");
    assert!(s.contains("KOhA_core \"ce'u\""), "{s}");
    // 項位置の ce'u(KOhA_core としての木)
    let s = parse_ok("mi klama ce'u");
    assert!(s.contains("KOhA_clause (KOhA_core \"ce'u\")"), "{s}");
    // 項位置の zi'o
    let s = parse_ok("mi viska zi'o");
    assert!(s.contains("KOhA_clause (KOhA_core \"zi'o\")"), "{s}");
    // 主語位置・質問詞併用(z0 実測 ok)
    parse_ok("ma klama ce'u");
    parse_ok("lo ka ma kau tarmi ce'u cu se nelci mi");
}

#[test]
fn ceu_zio_の_h_変体() {
    // アポストロフィ ↔ h 規約(z0/z1/maf 実測 ok)
    let s = parse_ok("mi klama cehu");
    assert!(s.contains("KOhA_core \"cehu\""), "{s}");
    let s = parse_ok("mi viska ziho");
    assert!(s.contains("KOhA_core \"ziho\""), "{s}");
    // 語境界: cehu は cehau 等の前置に誤マッチしない(word_boundary ガード)
    assert!(LojbanParser::parse(Rule::text, "mi klama cehau").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi klama cehuho").is_err());
}

#[test]
fn ceu_zio_の組合せ() {
    // 連続項(z0/z1/maf 実測 ok)
    parse_ok("mi klama ce'u zi'o");
    // 抽象内のラムダ変数(ka + kau 疑問詞)
    parse_ok("mi djica lo ka ma kau tarmi ce'u");
    // zi'o は GOhA/その他の項と同じ項位置で受理、既存 KOhA は不変
    parse_ok("mi klama zo'e");
    parse_ok("mi klama zu'i");
    let s = parse_ok("mi klama ri");
    assert!(s.contains("KOhA_core \"ri\""), "{s}");
}

#[test]
fn ui_標準語彙の欠落語() {
    // CLL 標準 UI の欠落語(dai/ca'e/ke'u/pau 等。z0/z1/maf 実測 ok)
    for w in [
        "a'i", "a'o", "ca'e", "dai", "e'a", "io", "ju'a", "ke'u", "le'o", "li'o", "o'e", "pau",
        "pa'e", "ra'u", "ro'a", "ro'o", "se'a", "si'a", "ta'u", "ti'e", "to'u", "va'i", "vu'e",
    ] {
        let input = format!("mi klama {w}");
        let s = parse_ok(&input);
        assert!(s.contains(&format!("UI_core \"{w}\"")), "{input}: {s}");
    }
    // h 変体(アポストロフィ ↔ h 規約)
    for w in [
        "ahi", "aho", "cahe", "eha", "juha", "kehu", "leho", "liho", "ohe", "pahe", "rahu", "roha",
        "roho", "seha", "siha", "tahu", "tihe", "tohu", "vahi", "vuhe",
    ] {
        let input = format!("mi klama {w}");
        let s = parse_ok(&input);
        assert!(s.contains(&format!("UI_core \"{w}\"")), "{input}: {s}");
    }
}

#[test]
fn ui_新語の統語位置() {
    // 文頭自由修飾語(z0 実測 ok)
    parse_ok("a'o mi klama");
    parse_ok("ca'e mi klama");
    parse_ok("dai mi klama");
    // tanru 単位位置(z0 実測 ok)
    parse_ok("mi klama ca'e broda");
    parse_ok("mi broda dai brode");
    parse_ok("mi pau broda");
    // 質問詞 dai(xu+UI の重畳。z0 実測 ok)
    parse_ok("xu dai do klama");
    // UI+CAI 強度・NAI 後置(ui_free の既存契約)
    parse_ok("mi klama dai ru'e");
    parse_ok("mi klama pau nai");
    // 感情強度との併用
    parse_ok("mi nelci do .u'u dai");
}

#[test]
fn ui_新語と既存語の選択不変() {
    // 既存 UI 語は引き続き同形で受理される(選択順序の不変確認)
    parse_ok("mi gleki ui");
    let s = parse_ok("xu do djica");
    assert!(s.contains("UI_core \"xu\""), "{s}");
    // ki'e は COI(呼格)、se'o は BAI としての既存収録を維持
    let s = parse_ok("mi klama ki'e");
    assert!(s.contains("COI_core \"ki'e\""), "{s}");
    parse_ok("mi klama se'o");
    // 語境界: 新語の前置誤マッチ防止。
    // pahu は BAI_core(pa'u)の有効語形だが、この統語位置では BAI タグ項が
    // 成立しない(tagged の BAI 枝は裸確定時に直後 selbri を禁ずるガードで
    // 「pahu broda」の selbri が残るため)ため拒否となる。tahuho は tahu の
    // 後が語内文字 h で word_boundary が成立しない語形不正
    assert!(LojbanParser::parse(Rule::text, "mi klama pahu broda").is_err());
    assert!(LojbanParser::parse(Rule::text, "mi klama tahuho").is_err());
}
