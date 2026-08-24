# 内部アーキテクチャ

本ドキュメントはパーサーの処理パイプラインと各モジュールの役割を解説する。
文法の語彙一覧は [coverage.md](coverage.md)、出力形式は
[json-schema.md](json-schema.md) と [parsing-guide.md](parsing-guide.md) を参照。

## 処理パイプライン

```
入力テキスト
  │  1. check_nesting   lu / lo'u / vei の同時ネスト数を数え、
  │                     上限(MAX_NEST=8)超過は高速拒否
  │  2. normalize_zoi   ZOI 引用(zoi DELIM 本文 DELIM)の区切り対応を検証し、
  │                     本文を zo'e に置換(pest に後方参照がないため)
  │  3. apply_erasure   SI/SU 消去の意味論を適用
  │                     (si=直前語消去 / su=文頭まで遡って消去。
  │                      引用内と zo 直後は保護)
  ▼
pest(LojbanParser, lojban.pest)
  │  PEG 解析。文法は 音韻 → 形態論 → cmavo クラス → 統語 の層構造
  ▼
Pairs<'_, Rule>(解析木)
  │  4. シリアライザ(tree.rs): 整形ツリー / S 式 / JSON(+pretty) /
  │                    DOT / HTML。EOI マーカーは除外
  ▼
出力
```

## 設計上の要点

### 前処理が存在する理由

1. **ZOI 正規化**: `zoi .ky. hello .ky.` のような区切り語対応は
   正規表現的な後方参照が必要で PEG 単体では表現できない。
   解析前に本文を `zo'e` へ置換することで文法を単純に保つ。
   解析木上は本文が `zo'e` に見える(仕様。json-schema.md 参照)
2. **SI/SU 消去**: 消去は「直前の語」という位置依存の意味論のため、
   文法で扱うより前処理で適用する方が正確かつ高速
3. **MAX_NEST**: 入れ子が深いと PEG のバックトラックが指数時間になる。
   ファジングで発見された問題へのリソース保護(v0.13)

### 文法ファイル(lojban.pest)の層構造

| 層 | 内容 |
|---|---|
| 音韻 | 母音・子音の定義に連結合法性が内蔵(zantufa 由来)。二重母音・音節・ストレス先読み |
| 形態論 | cmavo_form / cmevla(jbocme・zifcme) / brivla(gismu・lujvo・fu'ivla、rafsi 一式) |
| cmavo クラス | 各 selma'o の語彙(`*_core`)+ 語境界ラッパー(`*_clause`) |
| 統語 | text → content → item → sentence → terms/sumti/selbri/tanru → free |

### 実装上の重要な教訓(抜粋)

全リストは STATUS.md の「移植時に判明した重要ポイント」を参照。

1. より長い語形を接頭辞より先に置く(pest 文字列選択のクセ)
2. `(sp1 ~ X)?` は失敗時に sp1 の消費も取り消す。必須要素が続く場合は分岐で書く
3. 項リストの atom ルールを nullable にすると「常に成功する選択肢」となり
   pest がコンパイルエラーを出す
4. 結合 cmavo 語(sepi'o / ganai 等)は fu'ivla 形と紛らわしいため
   tanru_unit 冒頭のガードに登録する

## テスト体制

| スイート | 役割 |
|---|---|
| lib(20) | 公開 API・前処理(ZOI/SI/SU)の単体テスト |
| morphology(11) | 語形認識 |
| syntax(122) | 統語構造。バッテリー掃引で発見したギャップの回帰防止 |
| cli(16) | 実バイナリのエンドツーエンド(CARGO_BIN_EXE) |
| coverage_doc(1) | docs/coverage.md と文法の同期検証 |
| corpus(3) | Tatoeba 実文 403 文 + CLL 風例文(受理率 94% を実測担保) |
| fuzz(3+2ignored) | ランダム入力・コーパス変異・深さ掃引。重量版は定期手動実行 |
| doc(7) | README 的使用例がそのまま動くことの保証 |

## 性能

直近の測定値と camxes.js との比較は [comparison.md](comparison.md)、
criterion ベンチは `cargo bench` を参照。
