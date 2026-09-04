#!/usr/bin/env python3
"""GAP 掃引用プローブ行列の生成器。

src/grammar/lojban.pest から全 `*_core` 語彙リストを機械抽出し(coverage_doc.rs
の抽出手法の Python 版)、クラスごとに代表統語位置へ流し込んだプローブ文
(1行=1文)を tests/data/gap_probes.txt に出力する。

プローブの分類:
  - タグ系(BAI/FAhA/PU/ROI/TAhE/ZAhO/ZI/...): タグ4位置テンプレート
  - 項語彙(KOhA/PA/BY/GOhA): 項3位置テンプレート
  - 自由修飾語(UI/CAI/MAI/BAhE/...): free 前・後テンプレート
  - 抽象詞(NU)/接続詞(A/JA/JOI/GIhA/...)等: クラス固有テンプレート
  - 構造プローブ(語彙でなく構造の差分。既知 GAP 候補と OVER 記録用)

出力は語彙の抽出順(文法ファイル上の出現順)で安定しており、
行番号 = プローブ番号として run_gap_sweep.sh の比較表(CSV)と対応する。

再実行: python3 tests/data/generate_gap_probes.py
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PEST = ROOT / "src" / "grammar" / "lojban.pest"
OUT = Path(__file__).resolve().parent / "gap_probes.txt"


def strip_comments(src: str) -> str:
    return re.sub(r"//[^\n]*", "", src)


def load_rule_bodies(src: str) -> dict:
    """規則名 -> 本体文字列(ブレンス対応)。"""
    bodies = {}
    for m in re.finditer(r"^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*[@_]?\s*\{", src, re.M):
        name = m.group(1)
        depth = 1
        i = m.end()
        while i < len(src) and depth:
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
            i += 1
        bodies[name] = src[m.end() : i - 1]
    return bodies


def extract_cores(bodies: dict) -> dict:
    """*_core クラスの語彙リスト(本体に ^"..." リテラルが無い場合は
    参照先規則から委譲取得。PA_core -> PA_word)。"""
    cores = {}
    for name, body in bodies.items():
        if not name.endswith("_core"):
            continue
        words = re.findall(r'\^"([^"]+)"', body)
        if not words:
            refs = re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b", body)
            refs = [r for r in refs if r in bodies and r != name]
            # 委譲先は ^"..." リテラルを持つ規則(語彙本体)を選ぶ。
            # PA_core = @{ PA_word ~ &word_boundary } のように語境界などの
            # 補助規則が混ざる場合は無視する(v0.112 まで len==1 の条件で
            # PA_core の委譲が落ちて PA 語彙プローブが生成されていなかった)
            for ref in refs:
                words = re.findall(r'\^"([^"]+)"', bodies[ref])
                if words:
                    break
        cores[name] = words
    return cores


# ---------------------------------------------------------------- templates
# タグ位置(時制・BAI 系)。4 代表位置。
TAG_TEMPLATES = [
    "mi {W} klama",            # selbri 前タグ
    "mi {W} lo zdani klama",   # タグ+描述 項
    "{W} lo zdani klama",      # 文頭タグ+項
    "mi klama {W} lo zdani",   # selbri 後タグ+項
]
# 項語彙。3 代表位置。
SUMTI_TEMPLATES = ["{W} klama", "mi {W} klama", "mi viska {W}"]
# 自由修飾語。2 代表位置。
FREE_TEMPLATES = ["{W} mi klama", "mi klama {W}"]

TEMPLATES = {
    # タグ系
    "BAI": TAG_TEMPLATES,
    "FAhA": TAG_TEMPLATES,
    "PU": TAG_TEMPLATES,
    "CAhA": TAG_TEMPLATES,
    "CUhE": TAG_TEMPLATES,
    "ZI": TAG_TEMPLATES,
    "ZAhO": TAG_TEMPLATES,
    "TAhE": TAG_TEMPLATES,
    "ROI": TAG_TEMPLATES,
    "VA": TAG_TEMPLATES,
    "ZEhA": TAG_TEMPLATES,
    "VEhA": TAG_TEMPLATES,
    "VIhA": TAG_TEMPLATES,
    "MOhI": TAG_TEMPLATES,
    "KI": TAG_TEMPLATES,
    "FEhE": ["mi ze'i {W} roi klama", *TAG_TEMPLATES[:1]],
    "FA": ["{W} mi klama lo zdani", "mi klama {W} lo zarci"],
    # 項語彙
    "KOhA": SUMTI_TEMPLATES,
    "PA": SUMTI_TEMPLATES,
    "BY": ["{W} klama", "mi viska {W}"],
    "GOhA": ["mi {W}", "ti {W} lo zdani"],
    "LAhE": ["mi viska {W} di'u", "mi viska {W} lo broda ku"],
    "LE": ["mi viska {W} broda ku", "{W} broda cu barda"],
    # 抽象詞
    "NU": ["mi troci lo {W} klama ku", "lo {W} broda cu nandu"],
    # 接続詞
    "A": ["mi {W} do klama", "li pa {W} re du vo"],
    "JA": ["mi broda {W} brode"],
    "JOI": ["mi {W} do klama", "mi broda {W} brode", "li pa {W} re du vo"],
    "GIhA": ["mi broda {W} brode"],
    "GUhA": ["{W} broda gi brode"],
    "GA": ["{W} mi klama gi do broda", "{W} broda gi brode"],
    "GI": ["ga mi klama {W} do broda"],
    "BIhI": ["li pa {W} re du vo"],
    # 転換・me 系
    "SE": ["mi {W} klama lo zdani", "mi {W} broda"],
    "ME": ["ti {W} do broda", "mi {W} lo ci gerku klama"],
    "MOI": ["mi re {W}", "lo broda cu {W} re"],
    "JAI": ["mi {W} gau broda", "mi {W} broda"],
    # mex 演算系
    "VUhU": ["li pa {W} ci du xa", "li {W} pa du re"],
    "NAhU": ["li {W} su'i pa du re"],
    "FIhU": ["li pa {W} re du pamu"],
    "BIhE": ["li re su'i pa {W} pi'i ci du xa"],
    "PEhO": ["li {W} su'i pa ku'e re du re"],
    "MAhO": ["li {W} su'i du re"],
    "MOhE": ["li {W} pa su'i re du ci"],
    "VEI": ["li {W} pa su'i re ve'o su'i ci du xa"],
    "VEhO": ["li vei pa su'i re {W} su'i ci du xa"],
    "KUhE": ["li pe'o su'i pa {W} re du re"],
    "TEhU": ["mi me lo broda {W} klama"],
    "XI": ["mi viska lo broda {W} re", "li pa {W} re du re"],
    # 自由修飾語系
    # UI は UI+NAI の無空白結合形(dainai 等)も掃引対象。
    # 文頭形({W}nai mi klama)は z0/z1/maf 全 ok・ours 拒否の GAP 候補に、
    # 文末形(mi klama {W}nai)は ref 全 ok だが ours は fuhivla 緩さの
    # tanru 誤読で偶然 ok(v0.111 STATUS 次バッチ課題の記録行)
    "UI": ["{W} mi klama", "mi klama {W}", "mi klama {W}nai", "{W}nai mi klama"],
    "CAI": ["mi u'i {W} broda"],
    "MAI": ["mi broda pa {W}"],
    "BAhE": ["mi {W} klama"],
    "FUhE": FREE_TEMPLATES,
    "FUhO": FREE_TEMPLATES,
    "DAhO": ["mi klama {W}"],
    "SOI": ["mi klama {W} vo'a vo'e"],
    "SEhU": ["mi klama lo zdani soi vo'a vo'e {W}"],
    "SEI": ["mi klama {W} do zvati lo zdani", "{W} mi klama"],
    "Y": FREE_TEMPLATES,
    "SI": ["mi {W} klama"],
    "SU": ["mi klama lo zdani {W}"],
    # 文区切り
    "I": ["mi klama {W} do klama"],
    "NIhO": ["mi klama {W} do klama"],
    # 否定・肯定
    "NA": ["mi {W} klama"],
    "JAhA": ["mi {W} klama"],
    "NAhE": ["mi {W} klama", "lo {W} broda cu barda"],
    "NAI": ["mi pu {W} klama", "mi broda ja {W} brode"],
    # 項修飾・終端詞
    "NOI": ["lo broda {W} broda cu barda", "lo broda {W} mi viska ke'a ku'o barda"],
    "GOI": ["lo broda {W} mi cu barda", "mi {W} ko'a klama"],
    "CO": ["mi klama lo zdani {W} broda"],
    "COI": ["{W} la .alis. mi klama", "{W} do broda"],
    "KE": ["mi klama {W} broda je brode"],
    "KEhE": ["mi klama ke broda je brode {W}"],
    "KEI": ["mi troci lo nu broda {W}"],
    "KU": ["lo broda {W} klama"],
    "KUhO": ["lo broda poi mi viska ke'a {W} barda"],
    "GEhU": ["mi viska la'e lo broda {W} ku"],
    "LUhU": ["mi viska la'e lo broda {W}"],
    "LI": ["li pa du {W} re"],
    "LIhU": ["mi cusku lu broda {W}"],
    "LOhU": ["mi cusku {W} broda li'u"],
    "LU": ["mi cusku {W} broda li'u"],
    "LEhU": ["mi cusku zo broda {W}"],
    "ZO": ["mi cusku {W} broda"],
    "ZOI": ["mi cusku {W} gy. broda .gy"],
    "TO": ["to mi klama {W}"],
    "TOI": ["to mi klama {W}"],
    "DOhU": ["doi la .alis. {W} mi klama"],
    "FAhO": ["mi klama {W}"],
    "VUhO": ["mi viska lo broda {W} noi mi klama"],
    "VAU": ["mi klama {W}"],
    "BO": ["mi klama {W} cadzu", "mi pu {W} klama"],
    "BOI": ["li re {W} su'i ci du xa"],
    "BE": ["mi viska lo klama {W} lo zdani ku"],
    "BEI": ["mi viska lo klama be lo zdani {W} lo zarci be'o ku"],
    "BEhO": ["mi viska lo klama be lo zdani {W} ku"],
    "CEhE": ["mi klama {W} do"],
    "PEhE": ["mi {W} je do klama"],
    "NUhI": ["mi dukse {W} lo broda lo brode"],
    "NUhU": ["mi dukse nu'i lo broda lo brode {W}"],
    "BU": ["mi cusku zo {W}"],
}

# 構造プローブ(語彙でなく構造の差分)。
# 上段: 既知 GAP 候補 / 下段: OVER 既知差分(ours ok / z0 err)の記録用。
STRUCTURAL_PROBES = [
    # レタル接頭融合語(v0.110 解消+同時実施の形態論ストレステスト)。
    # 受理系: 残部が 5 字以上の有効 brivla(gismu 形 CVCCV / 正規 lujvo /
    # CVCy lujvo)。拒否系: CVCV 4字/CVCVV 5字の stress なし短形残部
    # (z0/z1/maf 3 種とも拒否の実測。本実装は !(cvcv_short_tail) ガードで
    # fuhivla 誤読を排除)。
    "lo byklesi ku",
    "lo cyklesi ku",
    "mi byklesi",
    "mi klama lo byklesi ku",
    "lo byklama ku",
    "lo bykukla ku",
    "lo bynunkapi ku",
    "lo bynaselci ku",
    "lo byduduki ku",
    "lo bykukybu'e ku",
    "lo dyjamynai ku",
    "lo byklaniku ku",
    "lo BYklesi ku",
    # 初子音結合(CCVCV)gismu 残部の受理系(bybroda / bycmalu / byzdani 型。
    # レタル接頭×残部語形マトリクスの裏取り。z0/z1/maf 実測 ok)
    "lo bybroda ku",
    "lo bycmalu ku",
    "lo byzdani ku",
    "lo cybroda ku",
    "mi bybroda",
    "lo bydjudi ku",
    "lo bysligu ku",
    # 短形残部の排除(z0 整合の拒否。parent 81a54c2 で全形 err の確認済み)
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
    "lo bykubu ku",
    "lo byvovu ku",
    "lo bykuku broda",
    "lo abu ku",
    "lo ebu ku",
    "lo cybu'u ku",
    "lo byku ku",
    # v0.110 残差(6字以上の無記名 CVCVCV 連鎖残部。z0 は拒否するが
    # BRIVLA_core の既存 fuhivla 緩さで受理が残る。一括排除は z0 が受理する
    # 6 字形(byduduki 型)を壊すため形態論レベルの stress 判定が要る。
    # 接頭なし 3 行は pre-existing OVER)
    "lo bykukula ku",
    "lo kuku ku",
    "lo kukai ku",
    "mi kuku",
    # v0.110 残差(項分離読み。z0 は cmavo 無ポーズ隣接の項読みで受理するが
    # 本実装は未対応)
    "mi bynaku klama",
    "lo bydudu ku",
    "mi bydudu",
    "lo bydudu klama",
    "lo bysukai ku",
    # v0.110 残差(多レタル接頭。z0 は lerfu_string 連鎖で受理)
    "lo byfyklesi ku",
    "lo bycyklesi ku",
    # v0.110 残差(関係節が続くレタル接頭。z0 は埋め込み lerfu 項+selbri+
    # relative の読み)
    "lo byklesi noi broda ku",
    # v0.110 残差(レタル+タグ cmavo の隣接。z0 は by 項+タグ+selbri の読み)
    "mi byta'e klama",
    "mi byca klama",
    # v0.110: brivla+レタル接頭融合語の無ポーズ隣接(z0 は拒否するが本実装は
    # 受容優先の既知クラスとして受理。tanru 2単位目・gihek 後)
    "mi klama byklesi",
    "mi klesi byklesi",
    "mi klama gi'e byklesi",
    # v0.110: 受理一致で読みが異なる形(z0 は su 消去+ta 項の読み)
    "mi suta byklesi",
    # FA + NAI(v0.105 既知 GAP)
    "fa nai mi klama",
    "mi klama fa nai",
    "fai nai mi klama",
    "mi klama fai nai",
    "fe nai mi klama",
    # 接続詞疑問 ji(v0.101 既知 GAP)
    "mi ji do klama",
    "mi ji do klama lo zdani",
    "do ji mi broda",
    "li pa ji nai re",
    # 単独 va'u / se va'u(SEBAI_joint の注記既知 GAP)
    "va'u mi klama",
    "se va'u mi klama",
    "mi klama se va'u lo nu broda",
    # タグ契約の KU 半分と裸タグ項(v0.108 レビュー。FA/BAI の tag +
    # (sumti | KU_elidible) 契約)
    "mi klama fa ku",
    "fa ku",
    "mi klama fa ku do",
    "mi klama fa nai ku",
    "mi klama fa fi'a ku",
    "mi klama va'u",
    "mi klama se va'u ku",
    "se va'u ku mi klama",
    "mi fa klama",
    "fa broda",
    # 裸 BAI の過剰受理チェック(タグが直後の項を貪欲に取る既存形との整合)
    "mi klama va'u do",
    "mi klama bai do",
    # sumti+ku の二重閉鎖(全参照 err を想定の整合確認)
    "mi klama fa mi ku",
    "mi klama va'u mi ku",
    # free 後の co 継続(v0.103 既知 GAP → v0.109 解消)
    "farlu ju'i co cnita",
    ".oi ta ca'o farlu ju'i co cnita",
    "mi farlu ju'i co cnita",
    # v0.109: co 継続の受理スコープ(z0 交叉実測)
    "farlu ju'i co se cnita",
    "farlu ju'i co na'e cnita",
    "farlu ju'i co nu broda kei",
    "farlu ju'i co cnita co brodi",
    "farlu ju'i co cnita gi'e brodi",
    "mi klama .ui co broda",
    "farlu ju'i co ja'a cnita",
    "farlu ju'i co na cnita",
    "farlu ju'i co cnita dohu",
    "farlu ju'i co cnita ku",
    "farlu ju'i co broda gi broda",
    # 裸 tanru BO 接続(v0.95 既知 GAP → v0.109 解消)
    "mi klama bo cadzu",
    # v0.109: 裸 BO の受理スコープ(z0 交叉実測)
    "mi klama bo cadzu bo bajra",
    "mi broda bo brode",
    "mi viska lo broda bo brode ku",
    "mi broda gi'e brode bo brodi",
    "mi klama co cadzu bo bajra",
    "mi na'e bo broda",
    "na'e bo broda",
    "to'e bo broda",
    # jai + se 変換タグ
    "jai se gau",
    "mi jai se gau broda",
    "mi jai gau broda",
    "mi jai se gau klama lo zdani",
    # v0.109: JAI+SE/NAhE 変換タグの受理スコープ(z0 交叉実測)
    "mi jai se ta'i broda",
    "mi jai na'e gau broda",
    "mi jai se na'e gau broda",
    "mi jai na'e se gau broda",
    "mi jai se pu broda",
    "mi jai se gau broda co brodi",
    "mi jai na gau broda",
    "mi jai ja'a gau broda",
    "mi jai se ja'a gau broda",
    "mi jai se go broda",
    # v0.109: bridi_tail 連結部の JOI/BIhI 接続(z0 交叉実測)
    "mi broda joi brode",
    "mi broda jo'e brode",
    "mi broda fa'u brode",
    "mi broda ku'a brode",
    "mi broda johu brode",
    "mi broda jo'u brode",
    "mi broda ji brode",
    "mi broda joi nai brode",
    "mi broda joi bo brode",
    "mi broda se joi brode",
    "mi broda bi'i brode",
    "mi broda ga'o bi'i ke'i brode",
    "mi broda bi'i nai brode",
    "mi broda se bi'i brode",
    "mi broda joi brode gi'e brodi",
    "mi broda joi brode joi brodi",
    "mi broda joi brode co brodi",
    "mi broda joi brode .ui brodi",
    "mi broda joi ju'i dohu brode",
    # 【v0.110 で解消。下記は v0.109 時点の記録】
    # v0.109 残差: 呼格+sumti 引数の継続形(z0/z1/maf は受理するが ours err)。
    # z0 実測では vocative は sumti を引数に取れるため「ju'i do」全体が
    # vocative 系 free として連結詞の直後に置ける。本実装の vocative_arg は
    # CMEVLA/desc のみで KOhA を取れず、gihek_free 側の vocative_closed は
    # DOhU 必須のため DOhU 省略形が届かない。
    # なお連結後に cmevla 引数を取る「mi klama gi'e ju'i la alen. cadzu」は
    # z0/z1/maf とも拒否(参照一致)で、残差は KOhA 引数の DOhU 省略形に限られる
    "mi klama gi'e ju'i do cadzu",
    "farlu ju'i do cnita",
    # 【v0.110 で解消。下記は v0.109 時点の記録】
    # v0.109 残差(GAP 候補): gihek の (NA? SE?) 前置。zantufa の gihek は
    # NA? SE? GIhA、joik は GAhO? NA? SE? JOI GAhO? で前置スロットを持つため
    # 参照 3 種は受理するが、本実装は gihek_link に前置スロットがない
    # (v0.109 で gihek_joik の JOI/BIhI 枝に SE 前置のみ実装済みで
    # GIhA 枝との非対称)。実装は次バッチ課題。
    # 注: 単独形「na joi」「na se joi」は z0 が拒否(z1/maf ok)のため
    # 単独プローブは参照分裂になる。連結部のスロットを直接測る文脈付き形で記録
    "mi broda na joi brode",
    "mi broda na se joi brode",
    "mi broda na gi'e brode",
    "mi broda se gi'e brode",
    "mi broda na gi'a brode",
    "mi broda se gi'a brode",
    "mi broda e brode",
    "mi broda a brode",
    "mi broda joi ju'i brode",
    "mi broda je ju'i brode",
    "mi klama gi'e ju'i cadzu",
    "mi broda joi brode gi brodi",
    "mi broda je brode ku brodi",
    # v0.109: 描述内 selbri の JOI 接続(z0 は selbri_4 の joik で受理するが
    # 本実装は gihek 経路のため未対応の残差。既知差分として記録)
    "mi viska lo broda joi brode ku",
    "mi viska lo broda bi'i brode ku",
    # mex + mai(free <- mex_2 MAI の mex 全体形)
    "mi broda vei ny su'i pa mai",
    "mi broda li pa mai",
    # 逆参照(ri/ra/ru)
    "mi klama ri",
    "mi klama ra",
    "mi klama ru",
    "mi viska lo broda .i mi viska ri",
    # 抽象詞の語彙確認(li'i/su'u/je'i/ni)
    "lo li'i broda cu nandu",
    "lo su'u broda cu nandu",
    "lo je'i broda cu nandu",
    "mi klama lo ni broda ku",
    # 尾部形 quantifier+sumti(v0.97 既知 GAP)
    "lo pa mi gerku ku barda",
    "lo pa le gerku ku barda",
    # tag + BO(v0.95 で受容済み・OVER 側のピン止め)
    "mi pu bo klama",
    "mi pu bo ge broda gi broda",
    # OVER 既知差分の記録(ours ok / z0 err)
    "mi caku klama",
    "caku mi klama",
    "li pa du re",
    "lo aburobu ku",
    "mi klama lo broda be'o",
    # その他の境界形の確認
    "mi klama ke lo zdani broda ke'e",
    "ganai mi broda gi mi klama",
    "mi ge broda gi brode",
    "xu do klama",
    "pe'i mi klama",
    "lo broda pe mi cu barda",
    "li vei ny su'i pa ve'o du re",
    "mi klama vi lo zdani",
    # v0.111: KOhA ce'u/zi'o(ラムダ変数・消去項)と CLL 標準 UI 欠落語。
    # ユーザー報告文(ka 抽象内 ce'u の失敗)の回帰アンカー
    ".i sy mintu lo purdykurji lo ka ma kau tarmi ce'u .i clani kurfa gi'e plita gi'e se kojna lo xance jo'u lo jamfu",
    "sy mintu lo purdykurji lo ka ma kau tarmi ce'u",
    "lo ka ma kau tarmi ce'u cu se nelci mi",
    "mi djica lo ka ma kau tarmi ce'u",
    "mi klama ce'u",
    "mi klama zi'o",
    "mi klama ce'u zi'o",
    "ma klama ce'u",
    "mi klama cehu",
    "mi viska ziho",
    # v0.111 UI 新語の位置別(z0/z1/maf 実測 ok)
    "a'o mi klama",
    "ca'e mi klama",
    "xu dai do klama",
    "mi nelci do .u'u dai",
    "mi klama ca'e broda",
    "mi broda dai brode",
    "mi pau broda",
    "mi klama dai ru'e",
    # v0.114: tu'a の LAhE 移設(KOhA_core → LAhE_core)。
    # zantufa-0.9999.js の LAhE 形態論は tu'a(tuha)を含み、sumti_5 の
    # LAhE 枝(LAhE relative_clauses? sumti LUhU_elidible)で後続 sumti が必須。
    # 上段: 裸 tu'a は参照 3 種とも拒否(KOhA 読みだった頃の OVER 3 行
    # tu'a klama / mi tu'a klama / mi viska tu'a を含む。z0 整合の意図的縮小)。
    # 下段: タグが LAhE 項を束ねる形を含む受理ピン(z0 の木と同型)。
    # tuha は h 変体('↔h 規約。z0/z1/maf 実測受理)
    "tu'a klama",
    "mi tu'a klama",
    "mi viska tu'a",
    "mi djica tu'a",
    "tai tu'a",
    "tu'a zo'u broda",
    "mi cu tu'a lo since",
    "tuha klama",
    "tai tu'a lo since",
    "mi djica tu'a do",
    "mi djica tu'a do lu'u",
    "tu'a do",
    "tu'a lo broda cu brode",
    "tu'a mi klama",
    "mi klama tu'a lo zdani",
    "mi klama tu'a lo zdani broda",
    "lu'e tu'a lo si'o broda",
    "tu'a do zo'u broda",
    "tuha do",
    "mi djica tuha do",
    # v0.114: 不閉鎖 to の実測記録。z0 の free 第5枝は
    # TO_clause text TOI_elidible で text が入力の残りを吸収し、TOI は常に
    # 省略可・閉鎖として機能しない(text が toi を word として先に消費)。
    # 本実装の to_quote(TO_clause ~ word* ~ TOI?)と挙動一致
    # (不閉鎖 to が後続の .i と次文まで吸われる点も含めて参照 3 種と整合)。
    # 意味論上の既知クラス:「to」は閉じられない限り発話の残りを引用する
    "mi to klama .i do broda",
    "mi to klama toi do broda",
    "mi to klama",
    "mi to klama toi",
    "to broda",
    "to broda toi",
    "mi viska do to do klama toi",
    "mi to klama toi broda",
    "mi to klama .i do broda toi",
    # v0.114 記録(既存 GAP 候補。tu'a 移設とは無関係の pre-existing):
    # 呼格+LAhE sumti 引数。z0 の free 第3枝は vocative sumti? DOhU_elidible
    # で sumti に LAhE 形(la'e/lu'e/tu'a)も取れるが、本実装の vocative_arg /
    # vocative_koha は CMEVLA/desc/KOhA のみで LAhE 形が届かない
    "ju'i tu'a do",
    "farlu ju'i tu'a do cnita",
    "mi klama gi'e ju'i tu'a do cadzu",
    # v0.114 記録(既存 GAP 候補。mex operand の LAhE。z0 の operand は
    # (LAhE_clause / NAhE BO_clause) mex LUhU_elidible を取る。la'e/lu'e でも
    # 同様の既存 GAP。tu'a 移設後に lahe_sumti 経路の補完候補として記録)
    "li la'e pa su'i re lohO",
    "li tu'a pa su'i re lohO",
    # v0.116: NA/NAhE/JAhA 直後の CAI は UI 読みの free(z0 は NA の直後に
    # post_clause の free を取り、sai/cu'i 等の CAI スケールを UI 語彙に包含)。
    # UI_core に sai/cu'i/cuhi を収録+UINAI_joint に sainai/cu'inai/cuhinai を
    # 収録+s_marks 直後に (sp1 ~ !BAhE_clause ~ frees_mid)? スロットを追加。
    # BAhE は z0 が後続 tanru_unit の前置に束ねるためスロットから除外。
    # NA 6 種×CAI 5 種×CAhA 有無の全 60 形は z0/z1/maf 実測で受理を確認
    # (na'e/to'e/no'e/je'a + CAI + ka'e のみ z1 が拒否の参照分裂)。
    # 下段は報告文(gi'e 接続第2枝の na sai ka'e muvdu)
    "mi na sai muvdu",
    "mi na sai ka'e muvdu",
    "mi na'e sai muvdu",
    "mi na'e sai ka'e muvdu",
    "mi to'e cu'i ka'e muvdu",
    "mi no'e cai muvdu",
    "mi je'a ru'e ka'e muvdu",
    "mi ja'a sa'e muvdu",
    "mi sai cai muvdu",
    "mi cu'i sai muvdu",
    "mi na sai nai muvdu",
    "mi klama gi'e na sai ka'e muvdu",
    "sainai mi klama",
    "mi klama sainai",
    "cu'inai mi klama",
    "mi klama cuhinai",
    "sai nai mi klama",
    "mi na'e ba'e mutce",
    ".i lo se kecti cu sligau lo rebla tai lo nu tolgei kei gi'e na sai ka'e muvdu",
]


def build_probes() -> list:
    src = PEST.read_text(encoding="utf-8")
    bodies = load_rule_bodies(strip_comments(src))
    cores = extract_cores(bodies)
    probes = []

    # 語彙プローブ: 文法ファイル上の出現順でクラスを巡回
    for full_name in cores:
        base = full_name[: -len("_core")]
        words = cores[full_name]
        templates = TEMPLATES.get(base)
        if not templates or not words:
            # CMAVO/CMEVLA/BRIVLA は語形の汎用フォールバック(語彙リストでない)ため対象外
            continue
        for tmpl in templates:
            for w in words:
                probes.append(tmpl.format(W=w))

    # 構造プローブ
    probes.extend(STRUCTURAL_PROBES)
    return probes


def main() -> None:
    probes = build_probes()
    # 重複除去(連続維持・順序維持)
    seen = set()
    uniq = []
    for p in probes:
        if p and p not in seen:
            seen.add(p)
            uniq.append(p)
    OUT.write_text("\n".join(uniq) + "\n", encoding="utf-8")
    print(f"wrote {len(uniq)} probes -> {OUT}")


if __name__ == "__main__":
    main()
