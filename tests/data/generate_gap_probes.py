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
    参照先規則 1 個から委譲取得。PA_core -> PA_word)。"""
    cores = {}
    for name, body in bodies.items():
        if not name.endswith("_core"):
            continue
        words = re.findall(r'\^"([^"]+)"', body)
        if not words:
            refs = re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\b", body)
            refs = [r for r in refs if r in bodies and r != name]
            if len(refs) == 1:
                words = re.findall(r'\^"([^"]+)"', bodies[refs[0]])
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
    "UI": FREE_TEMPLATES,
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
    # レタル接頭 lujvo(v0.106 既知 GAP)
    "lo byklesi ku",
    "lo cyklesi ku",
    "mi byklesi",
    "mi klama lo byklesi ku",
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
    # v0.109 残差: 呼格+sumti 引数の継続形(z0/z1/maf は受理するが ours err)。
    # z0 実測では vocative は sumti を引数に取れるため「ju'i do」全体が
    # vocative 系 free として連結詞の直後に置ける。本実装の vocative_arg は
    # CMEVLA/desc のみで KOhA を取れず、gihek_free 側の vocative_closed は
    # DOhU 必須のため DOhU 省略形が届かない。
    # なお連結後に cmevla 引数を取る「mi klama gi'e ju'i la alen. cadzu」は
    # z0/z1/maf とも拒否(参照一致)で、残差は KOhA 引数の DOhU 省略形に限られる
    "mi klama gi'e ju'i do cadzu",
    "farlu ju'i do cnita",
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
