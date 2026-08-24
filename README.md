English | [日本語](README.md)

# lojban — ロジバン PEG パーサー(Rust)

ロジバン(人工言語 Lojban)のテキストを解析する PEG パーサーです。
パーサーエンジンには [pest](https://pest.rs) を使用し、文法は
guskant 氏の **zantufa-1.9999.peg** を Rust/pest 向けに移植したものを
`src/grammar/lojban.pest` に定義しています。

## 機能

- **形態論**: zantufa 由来の完全な語形認識
  - cmavo(機能語)/ brivla(内容語: gismu・lujvo・fu'ivla)/ cmevla(固有名詞)
  - 音節構造・二重母音・子音連結の合法性検証(zantufa の子音規則を移植)
  - ストレス表記(大文字母音、例: `GERku`)対応
  - rafsi 分解による lujvo 判定(slinkuhi 対策含む)
- **統語論**(コアサブセット)
  - 文(sentence): 項(terms)+ `cu` + 述語(bridi_tail)
  - sumti: 代名詞(KOhA)/ 冠詞句(le lo la…)/ 量化描述(`ro lo remna`)/ 数詞
    / 文字参照(BY `xy` `abu` 等。項と数式の被演算子)
    / 接続(`e` `a` `o` `u` + `nai`、`joi` 系)
  - selbri: tanru(名詞句修飾)/ `na` 否定 / `ja'a` 肯定 / `na'e to'e` スケール反転
    / `se te ve xe` 変換 / `co` 逆順 / tanru 接続(`melbi je cmalu`、`je bo`)
    / 述語連鎖(`gi'e` `gi'a` …、`gi'e bo`)/ JAI 変換(`jai gau …`)
  - `be … bei … be'o` 項連結(linked sumti)
  - 抽象(`nu ka ni zu'o …`、SE 変換 `se du'u` / 結合形 `sedu'u`)
    / `ke … ke'e` グルーピング
  - 引用(`lu … li'u` 文引用 / `zo` 単語引用 = sumti。`lo'u … le'u` 誤文引用 = 自由修飾語)
  - 関係節(`poi / noi`)/ 所有(`pe / po / goi`)
  - 自由修飾語(感情標識 `ui` 等(+強度 `cai sai ru'e cu'i`)、談話標識
    `ku'i` `ja'o` `po'o` `da'i` `je'u` 等、`xu` 疑問、`sei` 挿入、
    `to … toi` 注釈、`soi … vo'a vo'e` 入れ替え、発話序数 `pamai`、
    添字 `xi re`、`da'o` 等。
    連鎖(`mu'o ge'e coi`)や項・述語間の挿入(`xu do su'a djica`)も許容)
  - **zei 複合語**(`zdani zei sinxa`): 完全な語を lujvo 相当に連結
  - 呼格(`coi …`)/ 文連結(`.i` `ni'o`、`.ije` `.ijanai` `.ibo` `.ijebo`
    等の結合表記、`.i bo` グルーピングを含む)
  - 先接続詞(`ge … gi`: 項と文の接続。NAI 結合形 `ganai … ginai` と
    分離形 `ga nai … gi nai` を含む)
  - **時制・相**(PU `pu ca ba` / CAhA `ka'e ca'a …` / ZAhO `co'a ca'o ba'o …`
    / ZI `zi za zu` / VA `vi va vu` / TAhE `ta'e di'i na'o ru'i` /
    疑問 `cu'e`)と **空間間隔**(VEhA `ve'i ne'i le zdani` / VIhA `vi'a ca'u`)
  - **空間・移動時制**(FAhA 方位 `ca'u ti'a zu'a ga'u ni'a …`、
    MOhI 移動指定 `mo'i ca'u`)、**時制間隔**(ZEhA `pu bi'o ba` / `ca bi'i ba`)、
    **sumti を取る時制タグ**
    (`mi ca le cabdei cu klama` / `vi ne'i le zdani` / 期間 `ze'a lo cacra`)
  - **項の補強**: LAhE 参照(`la'e di'u` / `lu'e le cukta`)、
    `naku`(NA KU)による項位置の否定、KOhA 補完(`mi'a` `ma'a` `do'o` `di'u`
    `tu'a` `dei` 等)、記述詞 `lo'e` / `le'e`
  - 数量詞+述語の項(`pa prenu cu klama`)、描述内数量詞(`le ci gerku`)
  - `me` 述語(+省略可の `me'u`)、数詞述語 MOI(`mi re moi` / `mi ci mei`)、
    項のみのフラグメント(`mi` 単独)、単独感情標識(`.ui`)
- **出力**: 整形ツリー / S 式 / JSON(`{"version":1,"rule","text","children"}` 形式。ルートに版数付き)
- **lujvo 生成・分解**(CLL 4.11/4.12 準拠): rafsi 列からハイフン規則(r/n/y)・
  tosmabru 検査・語中クラスタ規則(CLL 3.6)を適用して新語を合成し、
  公式スコアを算出(`--build-lujvo` / `lojban::lujvo::build()`)。
  逆方向の分解も対応(`--split-lujvo` / `lojban::lujvo::decompose()`)。
  build → decompose の roundtrip をテストで保証

## 使い方

```console
# 整形ツリーで表示
$ lojban "mi tavla do"

# S 式で表示
$ lojban "le mlatu cu cadzu" --sexpr

# JSON で表示
$ lojban "le mlatu cu cadzu" --json

# stdin から入力
$ echo "coi la alis." | lojban

# lujvo(新語)を生成(CLL 4.11 のハイフン規則 + tosmabru 検査 + 4.12 スコア)
$ lojban --build-lujvo "zba sai"
zbasai (score 5847)

# lujvo を rafsi 列に分解
$ lojban --split-lujvo "sairzbata'u"
sai (Cvv)
-r- [hyphen]
zba (Ccv)
ta'u (CvvApo)
```

### ライブラリ API

```rust
use lojban::{parse, tree};

let pairs = parse("mi viska le gerku")?;
println!("{}", tree::to_sexpr(pairs));
```

## 開発

```console
$ cargo test      # 全テスト(150件=単体146+doc 4、実文283文を含む)
$ cargo clippy --all-targets
$ cargo run -- "mi klama"
$ cargo bench    # 性能ベンチマーク(criterion)
```

## アーキテクチャ

```
src/
├── lib.rs            # 公開 API(parse / tree)
├── main.rs           # CLI(clap)
├── tree.rs           # 解析木 → S式 / 整形ツリー変換
└── grammar/
    ├── mod.rs        # LojbanParser(pest_derive)
    └── lojban.pest   # ロジバン文法(zantufa 移植)
```

文法ファイル内の構成:

| セクション | 内容 |
|---|---|
| 文字・音韻 | 母音/子音規則(連結合法性を内蔵)、音節、ストレス先読み |
| 形態論 | cmavo_form、cmevla(jbocme/zifcme)、brivla(rafsi 一式) |
| cmavo クラス | 各 selmaho の語彙リスト(`*_core`)+ 語境界ラッパー(`*_clause`) |
| 統語 | text → content → sentence → terms/sumti/selbri/tanru → free |

## 性能

同一文章での簡易ベンチマークでは、リファレンス実装 camxes.js(JS)より
高速(v0.37 再計測で約 3.6〜4.5 倍。v0.9 時点は 5〜8 倍だったが、
機能拡張に伴い解析コストは増加)。詳細・注意書きは
[docs/comparison.md](docs/comparison.md)、絶対値は `cargo bench`。
再現: `cargo run --release --example speed_check`

## 参考資料

- **主移植元**: [guskant/gerna_cipra](https://github.com/guskant/gerna_cipra) `zantufa-1.9999.peg`
- 差分確認: [lojban/ilmentufa](https://github.com/lojban/ilmentufa) `camxes.peg`(現行標準文法)
- 技法参考: [lojban/lensisku](https://github.com/lojban/lensisku) `src/grammar/*.peg`

## テスト

| ファイル | 内容 |
|---|---|
| `tests/morphology.rs` | 語形認識(gismu/lujvo/cmevla/ストレス等) |
| `tests/syntax.rs` | 統語構造の検証 |
| `tests/fuzz.rs` | 簡易ファジング(ランダム・変異・深さ掃引)。重量版は `cargo test -- --ignored` |
| `tests/corpus.rs` | **実文283文**(Tatoeba 実文220文、CC BY 2.0 FR + CLL 風厳選例文63文) |

実文コーパスは [Tatoeba](https://tatoeba.org) のロジバン文を使用しています。

## 既知の制限・ロードマップ

- 接続詞は基本形対応済み(`bo` グルーピング・BIhI 間隔接続・述語先接続 GUhA、
  先接続 `ganai … ginai`(分離形 `ga nai … gi nai` 含む)、MAhO 演算子)。
  未対応は FUhE/FUhO 先置論理と項set(NUhI)
- 数理表現(mex)は LI…LOhO の項と描述内数量詞(`le re su'i ci gerku`)に対応
  (`vei … ve'o` 括弧、`ki'o` `ma'u` `ni'u`)。演算子は左結合の単純連鎖で、
  SE 変換(`se pi'i`)・NAhU 派生演算子(`na'u zmadu`)・MAhO(`ma'o ny`)・
  BIhI 間隔、被演算子の mo'e+sumti(`mo'e ti`)、先置形式
  (`peho su'i re ci [kuhe]`)に対応
- lerfu は BY 語形(`by` `xy` `abu` 等)と BU 文字化(任意の語 + `bu`)に対応
- 消去(SI/SU)対応: 解析前に意味論を適用(`si`=直前語消去、`su`=文頭まで遡って消去。
  引用内と `zo` 直後は保護)。解析木は消去後の文に基づく
- 引用は `lu … li'u`(入れ子可)/ `zo` / `lo'u … le'u` / `zoi DELIM 本文 DELIM` に対応。
  ZOI は区切り語対応を解析前スキャンで検証し、本文は解析木上 `zo'e` に正規化される
  (pest に後方参照がないための設計。未閉鎖・不一致はエラー)
- タグは FA と BAI(bau mu'i 等。項と文頭に接続。`ri'a nai` のような NAI 否定可)
  に加え、SE 変換(分離形 `se ki'u …` と結合形 `sepi'o` `seva'u` `semu'i` 等)、
  FIhO モダルタグ(`fi'o dunda [fe'u] do`)、時制マーク連鎖+sumti
  (`pu le cabdei ku` / `vi ne'i le zdani`)に対応。selbri の前後どちらの項位置でも可。
  述語マークは否定 `na` に対して肯定 `ja'a`(`ja'a go'i` 等)に対応
- 述語の先接続(GUhA `gu'e … gi`、NAhE 併用可)と間隔端点の GAhO(`ga'o bi'o ke'i`)に対応
- cmavo 語彙は標準 CLL 系の主要語に絞り込み(実験的 cmavo は未収録。
  `lojban.pest` の各 `*_core` に選択肢を追加するだけで拡張可能)
- zantufa が許す「無ポーズ隣接単語」(例: `mibroda` = mi+broda)は不受理。
  本パーサーでは常にポーズ(空白・`.` `,` `!` `?`)区切りを要求する
- 母音のみの語(例: `iii`)は zantufa 原典準拠で fu'ivla として受理される
- 引用(lu / lo'u)と数式括弧(vei)の入れ子は深さ 8 まで(それ超過は高速拒否。
  PEG のバックトラックが指数時間になるためのリソース保護)
- 非語トークン(タイポ等)の拒否には rafsi 分解のバックトラックで
  100ms 級かかる場合がある

## ライセンス

本プロジェクトは [MIT OR Apache-2.0](LICENSE-MIT) デュアルライセンスで提供します。

参考文法 zantufa / camxes は各リポジトリのライセンスに従います。
