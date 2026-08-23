# 開発ステータス

## 現在の状態: v0.18 完成(全テストグリーン)

- ライブラリ20 / 形態論11 / 統語62 / コーパス3 / doc 4 / fuzz 3(+ignore 2) = 計103テスト全パス

## v0.18 で追加
- 空間・移動時制: FAhA(ca'u ti'a zu'a ga'u ni'a ru'u ne'i pa'o te'e ne'a re'o、
  h 表記併用)と MOhI(mo'i ca'u 等)を時制マーク位置に接続
- LAhE 項修飾(la'e = 参照先 / lu'e = 参照元。ge'u による明示閉鎖可)
- naku(NA KU)の項位置否定。結合表記 naku 一語と時制マーク位置も受理。
  素の na は引き続き拒否される
- KOhA 補完: mi'a / ma'a / do'o、発話指示 di'u de'u da'u(h 表記併用)
- 教訓追加(移植時重要ポイント6): cmavo 選択肢でのアポストロフィ形の順序制約

## v0.17 で追加
- lujvo 分解(decompose()、CLI --split-lujvo): バックトラック型セグメンテーションで
  rafsi 列+ハイフンへ分解。gismu 単体は拒否。build → decompose の
  roundtrip テストで双方向の整合を保証

## v0.16 で追加
- lujvo 生成アルゴリズム(src/lujvo.rs、CLI --build-lujvo):
  CLL 4.11 のハイフン規則(r/n ハイフン、不許容語中クラスタへの y 挿入、
  tosmabru 検査)+ CLL 4.12 スコアリング。公式例4件の生成結果とスコア値が一致。
  生成語が自前の文法で brivla として受理される自己統合テスト付き

## v0.15 で追加
- 英語版 README(README.en.md)を新設。両版の先頭に言語切替リンク。
  日本語版の陳腐化箇所を修正(API 例の旧シグネチャ、接続詞の未実装表記)

## v0.14 で追加
- 掃除: tree API の未使用引数 _input を削除(破壊的変更)、LICENSE-APACHE の
  プレースホルダ解消、MSRV(rust-version = "1.74")指定
- GUhA 先接続述語(gu'e … gi、NAhE 併用可。camxes selbri_6 準拠)
- GAhO 間隔端点(ga'o bi'o ke'i 形。camxes joik 準拠)
- コーパス拡充: Tatoeba API から取得した実文のうち受理できる 63 文を追加。
  未受理分の多くは無ポーズ結合語(loka/lonu 等)で設計上の拒否対象

## v0.13 で追加
- 依存なし簡易ファザー(tests/fuzz.rs): xorshift 乱数入力・コーパス文変異・
  入れ子深さ掃引。常時実行はスモーク規模、重量版は --ignored で明示実行
- 入れ子深度上限 MAX_NEST=8 を導入(fuzzing で発見された指数時間問題への対処):
  lu/lo'u/vei の同時ネストが 8 を超えると解析前に高速拒否する
- ファジングで判明した特性: ジャンク語(非語トークン)の拒否は rafsi バックトラック
  のため 100ms 級かかる(実用上許容・既知制限として記録)

## v0.12 で追加
- BU 文字化: 任意の語 + bu を文字参照(bu_lerfu)として項に接続。
  PEG の選択子が最初の成功で確定する性質上、sumti_core の先頭に配置
  (bu 非続時は高速フォールバック)

## v0.11 で追加
- 消去(SI/SU)の意味論実装: parse() 前処理パイプライン(zoi 正規化 → 消去適用)。
  引用(lu/lohu)内・zo 直後の保護つき。文法側は引用内容語として SI/SU を受理。
  KNOWN_FAILURES から "su" を削除(実装完了の証)

## v0.10 で追加
- lerfu(BY): 文字語(by cy … zy / abu … ybu)を項(pro-sumti)と数式被演算子に接続。
  sumti_core では quant_selbri より先に配置(貪欲競合の回避)。
  quant_selbri への mex 埋め込みはこの競合のため不採用

## v0.9.1 で追加(brivla 解析の最速化)
- lujvo ルールの否定先読み(!gismu ~ !fuhivla)を削除。
  BRIVLA の選択順序と全サイレントルール構造により受理集合は不変
- brivla_core / any_extended_rafsi の選択順序を変更し、高価な fuhivla を最後に
- 効果(ベンチ比): 全体で 30〜45% 高速化。
  短文 278µs→153µs、lujvo 929µs→679µs、to_json 733µs→415µs
- 実験して却下①: BRIVLA の選択順序入替(gismu|lujvo|fuhivla → gismu|fuhivla|lujvo)は
  lujvo 入力で逆に数割遅くなる
- 実験して却下②: stress 先頭スキャンの軽量子音クラス化(連結合法性述語の省略)は
  lujvo 約4% の改善にとどまり、不正クラスタ語での意味論変化リスクに見合わないため取消。
  残るボトルネックは initial_rafsi* のバックトラック構造自体であり、
  文法レベルの最適化はここが実用限界。更なる高速化は
  カスタムパーサー/Packrat メモ化等のアーキテクチャ変更が必要

## v0.9 で追加
- JSON 出力: CLI `--json` とライブラリ関数 `tree::to_json`
- doc test 導入(parse / to_sexpr / to_tree_string / to_json の使用例がそのままテストに)
- ベンチマーク導入(criterion、`cargo bench`)。基準値:
  - v0.9.1 時点: parse 短文 約153µs / 描述+関係節 約405µs / 複合 約518µs
  - 形態論 gismu 約12µs / lujvo 約679µs(v0.9 から 27% 改善。なお gismu 比 57 倍で
    rafsi 分解のバックトラックが残るボトルネック)
  - 出力 to_sexpr 約469µs / to_json 約415µs

## v0.8 で追加
- ZOI 引用(zoi DELIM 本文 DELIM): parse() 前処理で区切り語の対応を検証し、
  本文を zo'e に正規化してから文法に渡すハイブリッド方式。
  入れ子 LU 引用は既存の再帰文法で対応済みであることを確認

## v0.7 で追加
- 描述内数量詞への mex 埋め込み(sumti_tail の数量詞位置を number → mex に拡張)

## v0.6 で追加(コミット 514d92b)
- mex 数理表現: VUhU 演算子(su'i vu'u pi'i fe'i gei de'o te'o re'a va'a pa'i si'i fu'u)、
  vei…ve'o 括弧、LI…LOhO による項化。PA に ki'o / ma'u / ni'u を追加

## v0.5 で追加(コミット 4f29290)
- BAI タグ接続: 項(`bau la lojban.`)と文頭・selbri 頭(`mu'i le nu ...`)
- NAhE スケール反転(`na'e to'e no'e je'a`)を selbri マークに接続
- 接続詞の `bo` グルーピング(`joi bo` / `je bo`)
- BIhI 間隔接続(`bi'o bi'i mi'i`)を項接続に追加

## v0.4 で追加(コミット a66ed7d)
- 接続詞一式:
  - 項接続: A(`e a o u` + nai)、JOI(`joi jo'e fa'u ku'a`)
  - tanru 接続: JA(`ja je jo ju` + nai)、述語連鎖: GIhA(`gi'e` 等)
  - 文接続: `.ije` `.ija` 等(I+JA)。結合表記(.ije/.ijanai)は IJ_joint ルールで対応
  - 先接続: GA…GI(項レベル gek_sumti / 文レベル gek_sentence)
- 技術メモ: 「ij」で始まる語(.ije 等)は fu'ivla と紛らわしいため
  tanru_unit の BRIVLA から排除(`!("ija"|"ije"|"ijo"|"iju")` ガード)
- テスト対象文: Tatoeba実文97文 + CLL風厳選例文51文 + 単体テスト用文
- cargo build / cargo fmt --check / cargo clippy --all-targets 警告ゼロ

## v0.3 で追加(改善ラウンド、コミット 72569bf..68bd29e)
- プロジェクト基盤: git 管理・GitHub Actions CI(fmt/clippy/test)・crates.io 用 metadata(v0.2.0)
- BE…BEI 項連結(linked sumti): `mi klama be le zdani bei le zarci`、be'o 明示閉鎖
- 引用統語: LU…li'u(文引用)/ zo(単語引用)= sumti、lo'u…le'u(誤文引用)= free
- 終端詞・ni'o の標準アポストロフィ表記対応(li'u, ku'o, ke'e, se'u, do'u, ge'u, ke'i 等。h 表記も併用可)
- 否定テスト(未閉鎖引用・項のない be 等)、CI 強化(--locked, concurrency)

## v0.2 で追加(コーパス駆動)
- 時制・相(PU/CAhA/ZAhO/ZI/VA/TAhE)と selbri・文頭への接続
- 項フラグメント(mi 単独)、単独感情標識(.ui)、Y(ためらい)
- free の連鎖(mu'o ge'e coi)、項と述語間の自由修飾語(xu do su'a djica)
- 数量詞+selbri の項(pa prenu)、描述内数量詞(le ci gerku)
- me 述語、du'u 抽象、se du'u 変換、UI 語彙10個追加

## 実装済み
- pest + clap ベース(Cargo.toml / src/lib.rs / src/main.rs / src/tree.rs)
- 形態論: zantufa 由来の語形認識(音節・ストレス・rafsi・cmevla)
- 統語コア: 文・項・sumti・selbri・tanru・関係節・抽象・自由修飾語・呼格・文連結
- CLI: 引数/stdin、--sexpr、整形ツリー出力
- README.md

## 移植時に判明した重要ポイント(将来の拡張時に注意)
1. 各 cmavo クラス規則には必ず語境界ガード(`&word_boundary`)が必要。
   ないと "tavla" を "ta"+"vla" に誤分割する
2. post_word→lojban_word→クラス規則→post_word の静的左再帰が起きるため、
   境界チェックは `!ASCII_ALPHANUMERIC ~ !"'"` の単純形を使用
3. pest の進行解析: 繰り返し `X*` の X が深い参照チェーンだと
   「非進行」と誤判定される。`(sp1 ~ X)*` 形で回避
4. ループ内の sp1 は失敗時に入力を巻き戻す。ループ直後に別規則を続ける場合は
   明示的に `sp1?` を挟む(selbri の s_marks→tanru 間など)
5. `X ~ sp ~ (終端詞)?` は禁止。`?` は現在位置での空マッチ成功のため sp の
   消費が取り消されず、外側の sp1 が壊れる(li_mex/vei_group で発生)。
   必ず `(sp ~ 終端詞)?` とグループごと optional にする
6. cmavo クラス等の選択肢では、アポストロフィ付き形を必ずその素の接頭辞より
   先に置く。素の形(^"mi")が先だと境界先読みで失敗した際に後続の長い形
   (^"mi'a")が試されない(pest 文字列選択のクセ。最小再現:
   `x = @{ (^"mi" | ^"mi'a") ~ &wb }` が "mi'a" を拒否することで確認済み)

## 次の拡張候補
- sumti を取る時制タグ(`pu le cabdei` / `vi ne'i le zdani`)、ZEhA 時制間隔、
  GUhEK 等の先接続詳細制御
