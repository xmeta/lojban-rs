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
    / 接続(`e` `a` `o` `u` + `nai`、`joi` 系)
  - selbri: tanru(名詞句修飾)/ `na` 否定 / `na'e to'e` スケール反転 / `se te ve xe` 変換
    / `co` 逆順 / tanru 接続(`melbi je cmalu`、`je bo`)/ 述語連鎖(`gi'e` `gi'a` …)
  - `be … bei … be'o` 項連結(linked sumti)
  - 抽象(`nu ka ni zu'o …`)/ `ke … ke'e` グルーピング
  - 引用(`lu … li'u` 文引用 / `zo` 単語引用 = sumti。`lo'u … le'u` 誤文引用 = 自由修飾語)
  - 関係節(`poi / noi`)/ 所有(`pe / po / goi`)
  - 自由修飾語(感情標識 `ui` 等、`xu` 疑問、`sei` 挿入、`to … toi` 注釈。
    連鎖(`mu'o ge'e coi`)や項・述語間の挿入(`xu do su'a djica`)も許容)
  - 呼格(`coi …`)/ 文連結(`.i` `ni'o`、`.ije` `.ijanai` 等の結合表記を含む)
  - 先接続詞(`ge … gi`: 項と文の接続)
  - **時制・相**(PU `pu ca ba` / CAhA `ka'e ca'a …` / ZAhO `co'a ca'o ba'o …`
    / ZI `zi za zu` / VA `vi va vu` / TAhE `ta'e di'i na'o ru'i`)
  - 数量詞+述語の項(`pa prenu cu klama`)、描述内数量詞(`le ci gerku`)
  - `me` 述語、項のみのフラグメント(`mi` 単独)、単独感情標識(`.ui`)
- **出力**: 整形ツリー / S 式 / JSON(`{"rule","text","children"}` 形式)

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
```

### ライブラリ API

```rust
use lojban::{parse, tree};

let pairs = parse("mi viska le gerku")?;
println!("{}", tree::to_sexpr(pairs, "mi viska le gerku"));
```

## 開発

```console
$ cargo test      # 全テスト(74件=単体70+doc 4、実文157文を含む)
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

同一文章での簡易ベンチマークでは、リファレンス実装 camxes.js(JS)の
約 5〜8 倍の速度で解析できる(詳細・注意書きは [docs/comparison.md](docs/comparison.md))。
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
| `tests/corpus.rs` | **Tatoeba 実文97文**(CC BY 2.0 FR)+ CLL 風厳選例文60文 |

実文コーパスは [Tatoeba](https://tatoeba.org) のロジバン文を使用しています。

## 既知の制限・ロードマップ

- 接続詞は基本形対応済み(`bo` グルーピング・BIhI 間隔接続 `bi'o bi'i mi'i` を含む)。GUhEK 等の詳細制御は未実装
- 数理表現(mex)は LI…LOhO の項と描述内数量詞(`le re su'i ci gerku`)に対応
  (`vei … ve'o` 括弧、`ki'o` `ma'u` `ni'u`)。演算子は左結合の単純連鎖
- lerfu(BY/BU)、消去(SI/SU)未実装
- 引用は `lu … li'u`(入れ子可)/ `zo` / `lo'u … le'u` / `zoi DELIM 本文 DELIM` に対応。
  ZOI は区切り語対応を解析前スキャンで検証し、本文は解析木上 `zo'e` に正規化される
  (pest に後方参照がないための設計。未閉鎖・不一致はエラー)
- タグは FA と BAI(bau mu'i 等。項と文頭に接続)に対応
- cmavo 語彙は標準 CLL 系の主要語に絞り込み(実験的 cmavo は未収録。
  `lojban.pest` の各 `*_core` に選択肢を追加するだけで拡張可能)
- zantufa が許す「無ポーズ隣接単語」(例: `mibroda` = mi+broda)は不受理。
  本パーサーでは常にポーズ(空白・`.` `,` `!` `?`)区切りを要求する
- 母音のみの語(例: `iii`)は zantufa 原典準拠で fu'ivla として受理される

## ライセンス

本プロジェクトは [MIT OR Apache-2.0](LICENSE-MIT) デュアルライセンスで提供します。

参考文法 zantufa / camxes は各リポジトリのライセンスに従います。
