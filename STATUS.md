# 開発ステータス

## 現在の状態: v0.25 完成(全テストグリーン)

- ライブラリ20 / 形態論11 / 統語86 / コーパス3 / doc 4 / fuzz 3(+ignore 2) = 計127テスト全パス

## v0.25 で追加(語彙拡充バッチ)
- UI: ku'i ja'o po'o da'i je'u la'a za'a ga'i u'o i'i o'a e'i ka'u ru'a ji'a
  (h 表記併用)。ku'i(しかし)/ po'o(のみ)は実文最高頻度クラスの談話標識
- COI: vi'o ke'o / BAI: ci'u pu'a ji'e ji'u ji'o ra'a / JOI: jo'u /
  PA: su'e da'a
- tanru BRIVLA ガードに UI_clause を一括登録。UI_core は語境界付き完全一致
  なので正規の brivla に影響せず、今後の UI 語彙追加は自動保護される
  (個別列挙の時代終わり。joints/GANAI/SEBAI/SEDUHU/IBO/MAI は従来どおり個別)

## v0.24 で追加
- MAI 発話序数(pamai〜nomai の結合形): .i 直後と文末の自由修飾語として受理。
  fu'ivla 誤認防止のため tanru ガードに追加
- CAI 強度標識(cai sai ru'e cu'i、h 表記併用): ui_free で UI 語に後続
  (ui sai / ui cu'i)
- BOI 数終端詞を number 規則に接続(li re boi su'i ci …)。省略可

## v0.23 で追加
- SEDUHU_joint(結合形 sedu'u): 従来は fu'ivla として tanru に誤取り込みされる
  サイレント誤解析だったため、abstraction/nu_form の頭に joint を追加し、
  tanru BRIVLA ガードにも登録。項位置(sedu'u broda cu jitfa 等)では抽象として解析。
  なお selbri 直後(cusku sedu'u broda)は PEG の選択順序で nu_form 側が
  先着する(受理集合は正しく、グルーピングのみ selbri 側になる既知挙動)
- 文接続の BO: 分離形 .i bo と結合形 .ibo / .ijebo(IBO_joint)。
  母音始まり fu'ivla 形態が ibo を奪うため tanru ガードに追加
- 時制疑問 cu'e(CUhE)を tense_mark に追加
- 述語連鎖の bo グルーピング(gi'e bo)。tanru_link/ek_joik と対称化

## v0.22 で追加
- ZEhA 時制間隔(zi'i bi'o bi'i mi'i、h 表記併用): PU/ZI オフセットを両端に
  置く time_interval 規則を tense_mark に追加(pu bi'o ba / ca bi'i ba 等)
- BAI タグの NAI 否定: 項位置(ri'a nai le nu …)と文頭タグ位置
  (mu'i nai le nu …)の両方で受理

## v0.21 で追加
- 先接続詞の NAI 完成: GA+NAI の結合形(ganai genai gonai gunai)と分離形
  (ga nai)、GI 側も ginai / gi nai を受理(guhek_selbri の gi も同様)。
  ganai … gi … は if-then の標準形
- SE+BAI の結合タグ(SEBAI_joint): sepi'o seva'u semu'i secau seja'e
  secu'u seba'i seki'u seri'a seka'a(+h 表記)。項位置(文頭タグ含む)で受理
- 教訓: 結合 cmavo 語は fu'ivla 形と紛らわしいため tanru_unit の BRIVLA に
  ガードが必要(.ije の先例を GANAI/GINAI/SEBAI に拡張)。ガードがないと
  gek 内側の文が ginai を tanru として取り込み、PEG シーケンスは
  成功済み要素を短く再試行しないため全体が失敗する

## v0.20 で追加
- JAhA(肯定 ja'a)を s_mark に追加(na の対。ja'a go'i 等の応答表現)
- KOhA に tu'a(抽象化持ち上げ)/ dei(この発話)を追加
- LE に lo'e / le'e(典型例・完全集合の記述詞)を追加
- UI に e'o e'e a'e i'a bu'o を追加
- tagged を拡張: SE+BAI 変換タグ(se ki'u …)と FIhO モダルタグ
  (fi'o selbri [fe'u] sumti。fe'u は省略可 — selbri は tail_terms を
  含まないため直後 sumti との境界が自明)

## v0.19 で追加
- sumti を取る時制タグ(CLL 9): tagged に「時制マーク連鎖 + sumti(+ku 閉鎖)」を
  追加。selbri 前の項位置(mi ca le cabdei cu klama)と後の項位置
  (mi klama ca le cabdei)の両方で受理。時制マーク語彙と sumti 開始語彙は
  クラス分離しているため最長一致の曖昧性なし
- ZI クラスに期間形 ze'i ze'a ze'u(h 表記併用)を追加(ze'a lo cacra 等)

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
- NAhU/MOhI 由来の詳細時制、VEhA/VIhA 空間間隔、mex の演算子強化(MAhO/NAhU)
