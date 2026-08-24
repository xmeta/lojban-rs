# 開発ステータス

## 現在の状態: v0.72 完成(全テストグリーン)

- ライブラリ20 / 形態論11 / 統語119 / coverage_doc 1 / コーパス3 / doc 6 / fuzz 3(+ignore 2) = 計163テスト全パス
- コーパス 333 文(Tatoeba 実文受理率 94% を維持)
- Cargo.toml の版数を STATUS 版数に同期(0.53.0)

## v0.72 で追加
- JSON 出力にバイト位置情報を追加(各ノードに start/end。
  エディタ統合・ハイライト用途)。版数ポリシー通り追加フィールドは
  非互換扱いにしない(json-schema.md 更新済み)

## v0.71 で追加(CLI エッジテストとライブラリ例)
- cli.rs にエッジケースを追加: 空 stdin(使い方表示・終了コード 2)、
  CRLF 改行での --lines 動作、引用混成文の --json(+3テスト)
- README(日英)のライブラリ API セクションに friendly_error と
  to_json の使用例を追加し、docs 2点への参照を設置

## v0.70 で追加(節目: 安定性・性能の総点検、コード変更なし)
- 重量ファジング(--ignored)を v0.67 BIhE(mex 構造変更)後として再実行:
  427 秒でパニックなし
- criterion 再計測(v0.54 スナップショット比): parse 短文 391→313µs(-20%)、
  描述+関係節 1144→927µs(-19%)、複合 1251→946µs(-24%)、
  lujvo 形態論 995→714µs(-28%)。全項目で改善を観測
  (計測誤差の範囲も含むが、追加機能がありながら全体で高速を維持)
- camxes.js 比較: 4.4/4.1/3.4 倍(comparison.md に追記)

## v0.69 で追加(main.rs リファクタ、挙動不変)
- CLI の各モードを関数へ抽出(run_classify / run_split_lujvo /
  run_build_lujvo / run_stats / run_lines / run_parse + resolve_input)。
  挙動不変は cli スイート13テストで保証
- 教訓: ヒアドキュメント経由の大規模コード生成はエスケープ崩れを
  起こすため、Write ツール等の直接書き込みを使う

## v0.74 で追加(Tatoeba 定期再検証、--lines のドッグフーディング)
- 未収録 719 文を lojban --lines -f で一括検証: 94%(677/719)を維持
  (v0.38 以降 8 回連続同率)。v0.52 のバッチモードが実務で機能
- 検証済み 15 文をコーパスに追加(計 388 → 403 文)

## v0.73 で追加
- tree::to_json_pretty と CLI --pretty を追加(インデント付き JSON。
  --lines では1行維持のため常にコンパクト)
- 出力形式フラグ(sexpr/json/dot/html)を ArgGroup で排他化。
  組み合わせ時は黙って一方を優先するのではなく明確にエラー

## v0.68 で追加(Tatoeba 定期再検証)
- 未収録 734 文で受理率 94%(692/734)を維持(v0.38 以降 7 回連続同率)。
  v0.67 の BIhE 追加による回帰なしを確認
- 検証済み 15 文をコーパスに追加(計 373 → 388 文)

## v0.67 で追加
- BIhE(bi'e 演算子強調)を実装: li re su'i ci bi'e pi'i vo = 2+(3×4)。
  camxes mex_1 準拠の右結合構造(mex_items + BIhE 右グループ)。
  「意図的未収録」リストから外れ、coverage は 112 クラス / 110 接続
- fuzz タイミング劣化なし

## v0.66 で追加(パッケージング・ポリッシュ)
- Cargo.toml に repository を追加(メタデータ完備。
  crates.io 公開方針変更時は即対応可能)
- docs/README.md 新設: ドキュメント4点セットのインデックス
- 確認: --build-lujvo の5文字形 gismu は末尾限定という CLL 挙動が
  明確なエラーメッセージ付きで正しく機能

## v0.65 で追加
- --stats に文数フィールドを追加(入力全体の解析成功時のみ sentences:N。
  失敗時はフィールド省略)
- --classify が空白・カンマ区切りの複数語に対応
  (JSON は配列、平文は1語1行。単語は先頭ポーズ文字を除去)

## v0.64 で追加(安定性ラウンド、コード変更なし)
- 重量ファジング(--ignored)を v0.61 mex 変更後として再実行: パニックなし
- TODO/FIXME 掃引: コードベースはクリーン
- README の全コンソール例(7件)が出力どおりであることを再確認

## v0.63 で追加
- CLI に --stats を追加: 語トークンを語種別に集計し JSON で出力
  ({tokens, gismu, lujvo, fu'ivla, cmevla, cmavo, unknown})。
  解析成否に依存しないためコーパスの粗視調査に使える。
  --classify と分類ヘルパー(classify_word)を共用

## v0.62 で追加(Tatoeba 定期再検証)
- 未収録 754 文で受理率 94%(712/754)を維持(v0.38 以降 6 回連続同率)。
  v0.61 の mex 変更による回帰なしを確認
- 検証済み 20 文をコーパスに追加(計 353 → 373 文)

## v0.61 で追加(バッテリー#8: mex 深組み合わせ8文を実測)
- FIhU 除算演算子(fi'u)を mex_operator に追加(pa fi'u re = 1/2)
- 前置単項 VUhU(va'a pa 等)を被演算子として許可。数詞が先に一致する
  選択順序のため既存解析は不変(fuzz タイミング劣化なし)
- coverage.md 再生成(111 クラス / 109 接続)
- 未対応のまま: 連結 vei 群(要演算子)、mex 内 xi 添字

## v0.60 で追加(節目: --classify 語種判定コマンド)
- lojban --classify <word>: gismu / lujvo / fu'ivla / cmevla / cmavo /
  unknown を判定。既定は平文、--json で {"word","class"}。
  既存の形態論規則をそのまま活用
- 学習者ツール・辞書系スクリプトからの利用を想定

## v0.59 で追加(tests/cli.rs 新設: CLI エンドツーエンドテスト)
- 実バイナリを CARGO_BIN_EXE で起動し、v0.44 以降に追加した全フラグを
  自動検証(整形ツリー/--json JSONL/-q 終了コード/--lines 行番号報告/
  -f 入力/lujvo サブコマンド JSON/--dot/--html)。計10テスト
- 手動テスト依存を解消し、CLI 変更の回帰を CI で捕捉可能に

## v0.58 で追加(docs/parsing-guide.md 新設)
- 出力に現れる主要規則(text〜free_unit)の意味を解説する
  解析木リファレンスを新設。JSON の rule 名の実質的な仕様書。
  これでドキュメントスイートが揃った
  (coverage.md=文法語彙 / json-schema.md=出力形式 /
  parsing-guide.md=木構造 / comparison.md=性能)

## v0.57 で追加
- --build-lujvo / --split-lujvo に --json 出力を追加
  (build: word/score/hyphens/forms、split: word/parts[kind/text/form])。
  両出力とも有効な JSON であることを実測確認
- これで全サブコマンド・全出力経路が JSON 対応となった

## v0.56 で追加(Tatoeba 定期再検証)
- 未収録 774 文で受理率 94%(732/774)を維持(v0.38 以降 5 回連続同率)。
  検証済み 20 文をコーパスに追加(計 333 → 353 文)
- 検証には v0.52 の --lines -q を活用できる状態になった
  (今回までは従来スクリプトを使用)

## v0.55 で追加(API とドキュメントの仕上げ)
- friendly_error を CLI からライブラリへ移行(lojban::friendly_error)。
  Rust 利用者も CLI 同等の日本語エラーサマリを取得可能(doc test 付き)
- docs/json-schema.md 新設: --json 出力の正式仕様(フィールド・不変条件・
  版数ポリシー。フィールド追加は非互換扱いにしない方針を明記)
- lib.rs のクレート文書に公開 API 一覧と関連ドキュメントへの参照を追記

## v0.54 で追加(性能スナップショット、コード変更なし)
- criterion 再計測: parse 短文 約391µs / 描述+関係節 約1.14ms /
  複合 約1.25ms / gismu 約17.7µs / lujvo 約995µs /
  to_sexpr 約1.32ms / to_json 約1.42ms。
  to_json は v0.31 比で 29% 改善、lujvo 形態論も改善(v0.31 比 -22%)。
  機能拡張を続けながら全体では v0.31 時点と同等以下を維持
- speed_check + camxes.js 再比較: 3.1〜4.2 倍(comparison.md に追記)
- 文法内の重複選択肢を機械掃引し、実重複なしを確認
  (v0.51 の sumti_core BY 重複以外は構造上正当な再掲のみ)

## v0.53 で追加
- --lines が --json/--sexpr を尊重(1行 = 1オブジェクトの JSONL 等)。
  lojban --lines --json -f corpus.txt でコーパス全体を JSONL 化できる
- docs/coverage.md に「意図的に未収録の selma'o」セクションを追加
  (camxes 語彙との照合で BIhE/CEI/FEhE/FOI/FUhA/JOhI/LAU/TEI/TUhU/
  NUhA/RAhO/SA/ZIhE の12クラスと理由を明記)

## v0.52 で追加
- CLI に --lines を追加: 入力を行単位で個別解析(1行 = 1文)。
  失敗行は行番号と日本語ヒント付きで stderr に報告し、-q 併用で無出力。
  lojban --lines -q -f corpus.txt でコーパス検証がワンパイプラインに

## v0.51 で追加
- CLI に -q/--quiet を追加(成功時は無出力で終了コードのみ。
  エラーは stderr に出るためバッチ検証スクリプトでそのまま使える)
- sumti_core 末尾の重複 BY_clause 選択肢を削除(zantufa 移植時からの残り物)
- SI/SU エッジ(si 文頭/su 文末/.i si/引用内 si)を実測し全通過を確認

## v0.50 で追加(節目: Tatoeba 再検証と README 監査)
- 定期再検証: 未収録 794 文で 94%(752/794)を維持(v0.38 以降安定)。
  検証済み 20 文をコーパスに追加(計 313 → 333 文)
- README(日英)に v0.40 以降の機能記載を補完
  (prenex/項set/PEhE/CEhE/MOI be/JAI/ba'e/xi/da'o/la'edi'u/roroi)
- 出力5形式(整形ツリー/S式/JSON/DOT/HTML)、入力3経路(引数/-f/stdin)

## v0.49 で追加(安定性検証ラウンド、コード変更なし)
- 重量ファジング(--ignored)を v0.42 terms refactor 後として再実行:
  421 秒でパニックなし。refactor と以降の全変更(joint 追加等)が
  ランダム変異下で安定していることを確認
- ストレス・バッテリー#7: 複数機能を混成した長文6文を実測。
  有効な5文はすべて通過(先接続+関係節+FIhO タグ+引用の混成、
  時制連鎖+VAU+.i 文分割+ba ku、prenex 抽象 等)。失敗2件は
  いずれも無効ロジバン(vau 後の .i 抜け、誤記)

## v0.48 で追加(バッテリー#6: 呼格・対話・cmevla・副詞的談話標識13文を実測)
- LAHEDI_joint(la'edi'u = la'e+di'u 直前発話の参照先)
- ROROI_joint(roroi = ro+roi 常に。量化 ROI の結合形)
- UINAI_joint を9語補完(ji'anai ru'anai e'inai i'inai o'anai ka'unai
  a'unai u'inai u'unai)
- BAhE(ba'e 強調マーカー)を自由修飾語として受理(camxes pre_clause 準拠。
  次語への意味論付与は非モデル)
- 無効テスト文の整理: 「xu la alis. klama」「coi la alis. la djan. cliva」は
  cmevla+brivla を説明詞 tanru として取り込む曖昧形で cu 明示が必要
  (参考実装と同じ挙動。xu la alis. cu klama は通過)

## v0.47 で追加
- CLI に -f/--file 入力を追加(優先順位: 位置引数 > -f > stdin)
- Tatoeba 定期再検証(v0.42 の terms refactor 後の実測):
  未収録 794 文で受理率 94%(752/794)を維持。
  refactor による受理集合の変化なしを確認

## v0.46 で追加
- --html をスタンドアロン文書化(DOCTYPE + 同梱 CSS)。
  内部ノードは details/summary 折りたたみ(深さ 0〜1 は初期展開)、
  葉ノードは原文を表示。ブラウザで開くだけで解析木を閲覧・操作できる
- Tatoeba 定期再検証(v0.39 以降の変更に対する回帰測定):
  未収録 794 文で受理率 94%(752/794)を維持。残存失敗は既知カテゴリ

## v0.44 で追加
- Graphviz DOT 出力(tree::to_dot / CLI --dot): ノードに規則名と原文を
  ラベル化し、dot -Tsvg 等で解析木を可視化できる
- 版数同期方針を決定: 従来のクレート 0.9.x と STATUS v0.x の不同期を解消

## v0.43 で追加(ドキュメント整備)
- docs/coverage.md に結合表記(joint)セクションを追加
  (IJ/IBO/NAKU/GANAI/GINAI/SEBAI/SEDUHU/UINAI の8規則と
  tanru_unit ガードとのセット管理の説明)。coverage_doc テストは
  クラス表のみ検証するため追記セクションは自由
- README(日英)に SA(sa)遡及修正構文の非対応方針を明記
  (SI/SU 消去の記述の隣。前向きの一致 selma'o 探索が必要なため)

## v0.42 で追加
- PEhE 項グループ接続(pe'e je / pe'e joi。camxes terms_1 準拠)。
  terms と tail_terms が共通の items 本体(silent 規則で木形状不変)を
  共有する構造に refactorし、CEhE/PEhE が selbri 前後の両位置で機能
- 教訓5裏面の変種を3連続で経験(項リスト refactor 中):
  1. atom ルールの * が空マッチ可能だと fragment が常に成功する選択肢に
     なり pest がコンパイルエラー(expression cannot fail)
  2. (sp1 ~ atom)+ と書くと最初の項にも空白が要求される
  3. (sp1 ~ free)* の free 失敗で sp1 が巻き戻り、直後の term が
     空白から始められない
  最終形: 先頭原子は sp1 なし、区切りはループ内 sp1、tail は
  (sp1 ~ items)? で空白ごと試行

## v0.41 で追加
- CEhE 項区切り(mi ce'e do tavla。camxes terms_2 準拠)を terms 連鎖に追加
- tests/coverage_doc.rs 新設: lojban.pest からクラス一覧を再生成し
  docs/coverage.md との同期を検証(108 クラス / 106 接続)。
  初版の抽出器には _core 接尾辞の扱いにバグがあり全クラスを
  wired=true と誤計算していたが、GIhI の不整合として自己検出・修正
  (ドキュメント検証テストが自分のバグを捕まえた例)

## v0.40 で追加(区切りの品質総点検)
- FAhO(fa'o テキスト明示終端)と VUhO(vu'o 項連結して関係節共有)を接続。
  これにより cmavo クラス 101 定義のうち 100 が統語に接続
  (LIhU は LUhU 同語形の予備定義で設計上未使用)
- docs/coverage.md 新設: 全クラスの語彙と統語接続状況の一覧
- 検証: 重量ファジング(--ignored)を完全実行し 314 秒でパニックなし。
  README の全コンソール例が出力どおりであることを確認

## v0.39 で追加(バッテリー#5: mex・数詞・lujvo 11文を実測、10文が既存で通過)
- MOI 述語に be 連結を許可(lo re moi be le ci gerku。moi は selbri のため項を取る)
- to_json のルートに "version":1 を埋め込み(doc test も更新)。
  出力形式を機械処理する場合のスキーマ判別用

## v0.38 で追加(Tatoeba 定期再検証)
- 全1000文を再取得し現行コーパス外 824 文で受理率を再測定: 94%(778/824)。
  v0.28 時点と同率を維持。残存失敗 46 文は実験的 cmavo・単独 CAI/時制
  フラグメントなど既知カテゴリ(v0.28 記録と一致)
- 重要な発見: e'u(提案)が v0.20 の語彙バッチ以来欠落していたことを
  実測が捕捉。o'u(平静)とともに追加(h 表記併用)
- コーパスに検証済み 30 文を追加(計 283 → 313 文)

## v0.37 で追加
- terms 間・項前後の自由修飾語混在(mi .ui do tavla)。v0.36 の
  tail_terms 対称。fuzz タイミング劣化なし
- docs/comparison.md を v0.37 再計測で更新: camxes.js 1262/3771/3611µs に対し
  lojban 350/1052/811µs = 約 3.6〜4.5 倍(v0.9.1 の 5.3〜8.1 倍からは低下。
  機能拡張の代償として STATUS ベンチ記録と整合)

## v0.36 で追加(バッテリー#4: 引用・入れ子9文を実測)
- 項set NUhI/NUhU(nu'i X Y [nu'u])を term に追加
- FUhE/FUhO 感情スコープ標識を自由修飾語に追加(camxes indicators 準拠)
- tail_terms が自由修飾語と項の混在を許容(mi cusku .ui do /
  cusku pe'i lo xamgu / 述語直後の sei 文)。free の各選択肢は
  先頭トークンが異なる literal のため失敗パスは安価(fuzz 劣化なし)
- 調査メモ: 「la alis. cusku …」は説明詞 greedy 読みが確定する曖昧形で
  cu による明示が必要(参考実装と同じ挙動)。バッテリー文のうち
  入れ子抽象2件は無効ロジバン(nu の内容は文であるべき)だった

## v0.35 で追加(バッテリー#3: 抽象・質問・タグ多用文14文を実測、12文が既存で通過)
- gek_sentence に frees_s? を先置(xu ganai … gi … / pe'i ganai … gi …)。
  内側を inner_sentence に統一し gek/prenex の入れ子も許容
- 残り2件の失敗は本修正で解消。バッテリー残存失敗なし

## v0.34 で追加
- XI 添字(lo gerku xi re / li xy xi pa)と DAhO を自由修飾語として追加
  (camxes xi_clause 準拠。XI + number/BY/vei)
- README 最終棚卸し: テスト数(88→150)・コーパス数(223→283)の更新、
  MAhO「未実装」記述の削除(v0.29 で実装済み)、実際の未対応
  (FUhE/FUhO、NUhI 項set)への差し替え、性能主張への計測時点注記、
  文法ファイル内の陳腐化コメント(ZOI 未対応表記等)の修正

## v0.33 で追加
- zei 複合語(zdani zei sinxa)。tanru_unit の先頭選択肢に置き
  BRIVLA 単独読みによる横取りを防止(camxes zei_tail 準拠だが
  語は完全な word に制限)
- SOI 入れ替え(soi vo'a vo'e [se'u])を自由修飾語に追加
- CLI エラーメッセージ改善: 日本語の位置サマリ + 頻出内部規則名の
  翻訳(rule_desc)を表示し、pest の詳細出力を従来どおり後段に表示
- 調査メモ: 文法19行目の SOI は pest の組み込み規則
  (Start Of Input)でありロジバン selma'o ではなかった(今回初めて判明)

## v0.32 で追加
- 数詞述語 MOI(mei moi si'e cu'o va'e、h 表記併用)を tanru_unit に追加
  (mi re moi / mi ci mei / le re moi prenu。camxes の語形リストを原典確認)
- me_form に省略可能な me'u と後続 MOI を追加(camxes: ME sumti MEhU? MOI?)
- 教訓5裏面の2例目を修正:
  1. 裸数詞項が直後 MOI を取らないよう否定先読みでガード
     (PEG 繰り返しは成功で確定し bridi_tail 側へ戻れないため)
  2. sumti_tail を (mex sp1)? selbri から明示分岐へ
     (mex 成功後に selbri 失敗すると「selbri 単独」に戻れない。
     le re moi prenu で発覚)

## v0.31 で追加
- JAI 変換を tanru_unit に追加(jai gau zdani / jai zdani。camxes tanru_unit_2 準拠)。
  BAI に gau(行為者)を追加
- PA から16進桁 jai を削除(quant_selbri が「jai + selbri」を数量詞として
  横取りし JAI 変換と衝突するため。dau fei rei vai は残置)
- 教訓5の裏面を確認: (sp1 ~ X)? は X 失敗時に sp1 の消費も取り消すため、
  直後に必須要素が続く場合は分岐で書く(JAI 枝で実測)
- ベンチマーク再計測(v0.9.1 比較、sample_size=20 の参考値):
  | 項目 | v0.9.1 | v0.31 |
  |---|---|---|
  | parse 短文 | 153µs | 約410µs |
  | parse 描述+関係節 | 405µs | 約1.03ms |
  | parse 複合 | 518µs | 約1.14ms |
  | morphology gismu | 12µs | 約16µs |
  | morphology lujvo | 679µs | 約1.27ms |
  | output to_sexpr | 469µs | 約1.40ms |
  | output to_json | 415µs | 約2.0ms |
  v0.9.1 以降に選択肢を大幅拡張した代償として 2〜4 倍程度の解析コスト増。
  機能カバレッジとのトレードオフとして受入れ、最適化は
  アーキテクチャ変更(Packrat 等)が必要な領域とする(v0.9.1 教訓の再確認)

## v0.30 で追加
- ZOhU 前置スコープ(su'o da zo'u … / ro da zo'u ganai … gi …)。
  item レベルと inner_sentence(抽象内)の両方で受理
- TEhU 演算子終端(li re na'u zmadu te'u ci)。NAhU/MAhO 枝に省略可で接続
- 方針決定: 実験的 cmavo(ki'ai dei'a xa'o 等)は引き続き非収録
  (CLL 標準語彙主義を維持。Tatoeba 残存未受理の主因と割り切る)
- パフォーマンス教訓: item 失敗パスで再試行される選択肢に完全な terms を
  置くと rafsi バックトラックが乗じて指数時間になる(fuzz 5s → 55s を実測)。
  prenex の項は量化詞+代名詞の単純形に制限して解決

## v0.29 で追加(空間間隔と先置数理)
- VEhA/VIhA 空間間隔を tense_mark に追加。語形は camxes の文法ソースを
  取得して原典確認(記憶ではなく検証: VEhA=ve'a ve'i ve'e ve'u /
  VIhA=vi'a vi'e vi'i vi'u)。FAhA(+nai) を後続可(ve'i ne'i le zdani)
- PEhO 先置数理(peho su'i re ci [kuhe])。教訓: number の空白連結が
  第2被演算子を取り込むため、先置用の被演算子は単一形に制限(fore_operand)
- MAhO(mex→演算子)を中置演算子として追加(li re ma'o ny ci)

## v0.28 で追加(Tatoeba 再取得による実測検証)
- Tatoeba API から jbo 文を再取得(全1000文、既存重複除外の 884 文を実測):
  受理率 92% → 微修正後に 94%(832/884)。受理できた複数語文から
  検証済み 60 文をコーパスに追加(計 223 → 283 文)
- 微修正: UI に ai(意図)/ au(欲望)/ ki'a(混乱疑問)、呼格の NAI(ju'i nai)、
  フラグメントの後続自由修飾語(mi'a uu)、KOhA に集合の mi'ai
- 教訓6を拡張: 接頭辞関係はアポストロフィ形同士でも発生
  (mi'a は mi'ai の接頭辞 → mi'ai を先に)
- 残存未受理 52 文の内訳: 実験的 cmavo(ki'ai dei'a xa'o 等)、単独 CAI/時制
  フラグメント(sai fanza / viska co'i)、設計上拒否が正しい非標準形

## v0.27 で追加(会話文バッテリー24文による実測ギャップ修正)
- selbri で時制の後の述語マークを許容(ti ba se citka / pu zi je'a citka)
- 埋め込み文(抽象/nu_form/sei/関係節)に先接続文を許容
  (lo ka ganai broda gi brode)。inner_sentence 規則を新設
- UI に zu'u(対比)/ ba'a(期待)を追加
- UINAI_joint: 談話標識+NAI の結合形(ta'onai ba'anai ku'inai 等13語)
- PA_seq: 無ポーズ連結数詞(li renono = 1200)。分離形の解析木形状は不変

## v0.26 で追加
- mex 演算子の拡張: NAhU+tanru(na'u zmadu)、SE 変換 VUhU(se pi'i)、
  BIhI 間隔演算子。被演算子に MOhE+sumti(mo'e ti)を追加
- 時制固定 ki(KI)を tense_mark に追加(解析レベルのマーカー。固定意味論は非モデル)
- 教訓: 二項 na'u 演算子には被演算子が2つ必要(li re na'u zmadu ci の形。
  li の入れ子は li_mex が mex を包含しないため不可 — CLL 同様 loho/boi での
  明示閉鎖が必要な領域)

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
- Tatoeba 再検証の定期実施、HTML 出力への DOT 拡張、
- crates.io 公開はユーザー判断で見送り中(方針変更時は版数同期済みのため即対応可)
