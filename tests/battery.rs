//! バッテリー掃引(v0.27〜v0.61)で収集した実文形式の統合回帰スイート。
//!
//! 各バッテリーで「受理される」ことが確認済みの文を恒久化し、
//! 文法変更時の実用性リグレッションを捕捉する。

use lojban::parse;

fn assert_ok(sentences: &[&str]) {
    for s in sentences {
        assert!(parse(s).is_ok(), "バッテリー文が解析できなくなった: {s:?}");
    }
}

/// バッテリー#1(v0.27): 対話・質問・タグの基礎文
#[test]
fn battery1_対話と基礎() {
    assert_ok(&[
        "xu do tavla fi la lojban.",
        "mi na kakne lo ka tavla",
        "ko'a pu zi je'a citka",
        "ti du le cukta poi mi viska",
        "do djica tu'a ma",
        "mi bilga la'e di'u",
        "ko na darxi mi",
        "ei do klama",
        ".ii do gunta mi",
        "lo nu do klama cu se jalge lo nu mi gleki",
        "a'u do xamgu tadni",
        "mu'o mi'e la alis.",
        "fi'i do klama",
        "a'unai",
        "mi'a pu ze'a cadzu",
        "ko curmi ro da",
        "di'a gunka",
        "mi pu prami do ca lo cabdei",
        "le nanmu ku goi ko'a klama",
    ]);
}

/// バッテリー#3(v0.35): 抽象・先接続・項の補強
#[test]
fn battery3_抽象と先接続() {
    assert_ok(&[
        "xu ganai broda gi brode",
        "pe'i ganai mi klama gi mi cadzu",
        "mi kakne lo ka bajra su'o da",
        "le mu prenu cu simxu lo ka prami",
        "ko'a cusku fi'o jetnu fe'u lo du'u broda",
        "pu ze'a lo nu mi tadni kei mi morji",
        "mi klama ca lo nu do cadzu kei ku",
        "ro prenu cu prami su'o da",
        "xu lo gerku cu batci do",
        "ma pu cau do zdani",
        "do se slabu mi",
        "la djan. cu pamoi ro lo troci",
        "mi bilga lo ka co'a gunka",
        "mi troci lo ka ganai broda gi brode",
    ]);
}

/// バッテリー#4/#7(v0.36/v0.49): 引用・入れ子・混成長文
#[test]
fn battery4_引用と混成() {
    assert_ok(&[
        "lo se cusku be lu do drani li'u cu xamgu",
        "zo broda zo broda",
        "xu do pu viska lu le mlatu cu cadzu li'u",
        "ma cusku bau la lojban. lu coi li'u",
        "li vei re su'i ci ve'o su'i vo du li mu",
        "ganai lo gerku poi mi viska ke'a ku'o cu batci fi'o spaji fe'u do gi mi cusku lu xamgu li'u",
        "mi pu ze'a ca le cabdei troci lo ka su'o da zo'u da se prami",
        "le nanmu noi pu zi je'a tavla bau la lojban. cu se jimpe mi",
        "xu do djica tu'a nu mi na'e roroi klama kei pe'i sai",
        "mi'a pu mo'i ca'u ze'i vofli vau .i ba ku mi'a co'a renro",
        "ganai lo nu mi gleki kei fa'u lo nu do gleki cu fasnu gi mi smaji",
    ]);
}

/// バッテリー#5/#8(v0.39/v0.61): 数理表現
#[test]
fn battery5_数理() {
    assert_ok(&[
        "li re su'i ci pi'i vo du li pa no no",
        "li vei ny su'i pa ve'o pi'i xa du li xa",
        "pa ki'o re du li pa re",
        "li ji'i mu du li mu",
        "le pamoi gerku cu klama",
        "mi pamoi lo'i prenu",
        "lo re moi be lo ci gerku cu barda",
        "li xo pu cusku",
        "li renono su'i ci du li reno ci",
        "ma klama li pa",
        "li re na'u zmadu ci su'i vo du li xa",
        "li mo'e ti pi'i re du li za'u",
        "li re su'i ci naku du li mu",
        "li pa fi'u re du li pimu",
        "li va'a pa du li ni'u pa",
    ]);
}

/// バッテリー#6(v0.48): 呼格・談話標識・結合形
#[test]
fn battery6_呼格と談話標識() {
    assert_ok(&[
        "je'e mi'e la alis.",
        "la frank. la alis. prami",
        "po'o lo gerku cu batci",
        "la'edi'u cu xamgu",
        "roroi mi cadzu",
        "mi na'e roroi klama",
        "ji'anai mi gleki",
        "ru'anai do drani",
        "ba'e do viska mi",
    ]);
}
