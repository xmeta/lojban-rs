# 開発ステータス

## 現在の状態: v0.116 完成(NA 直後 CAI の UI 読み・全テストグリーン)

- ライブラリ20 / 形態論11 / 統語225 / battery 5 / cli 20 / coverage_doc 1 / コーパス3 / fuzz 9(+ignore 2) / gap_tracker 12 = 単体306 + doc 11 = 計317テスト + example 内 unit test 5 全パス
- tests/gap_tracker.rs は既知 GAP の追跡用 12 テスト。v0.108 でバッチ1(語彙+NAI 後置)の 4 件、
  v0.109 でバッチ2(接続詞・前置系)の 4 件、v0.110 でバッチ3(形態論・共有・前処理系)の 4 件を
  解消し全 12 テストが緑(下記「既知GAP」参照)
- コーパス 418 文(Tatoeba 実文受理率 94% を維持)
- Cargo.toml の版数を STATUS 版数に同期(0.116.0)

### 帳簿整理(v0.114 版数のまま。文法・受理挙動の変更なし)

- OVER 候補の未分類 31 行を既知差分クラスとして記録(下記
  「既知差分(OVER)」節。ZEhA 裸タグ 20 / soi 3 / CEhE 2 / kiaha 2 /
  miahi 3 / CAhA nu'a 1。tu'a 3 行は v0.114 で解消済み)
- zantufa UI 実験語の数を 93 語→111 語形に訂正(列挙 91 語からの
  20 語形の列挙漏れを peg 全数突合+文頭/文末両位置の再実測で補完)
- docs/parsing-guide.md に v0.100〜v0.110 追加分の規則
  (gek_tail/gek_head・tanru_post/co_post・gihek_joik と gihek 前置・
  項レベル ke・VUhO 関係節スロット・BY_prefix・tense_mark の NAI 後置と
  FA 裸タグ項・空項 cu 文・tanru_unit の NAhE/SE 前置)を追記
  (quant_sumti(v0.101)/prenex 拡張(v0.100)は既記載を確認)
- docs/architecture.md のテスト体制表を現状に同期(syntax 218・cli 20・
  corpus 418・doc 11・fuzz 9+2・battery/gap_tracker 行の追加)。
  docs/README.md の comparison.md 時点数を 3→5 時点に修正
- 「既知の性能特性」を architecture.md に記録(lu ネストは深度+1 で約 3 倍の
  指数成長・MAX_NEST=8 で最悪数秒に飽和・vei/gek/mex は平坦・
  語長上限 50 による rafsi 指数と stack overflow の封じ込み)。
  tests/fuzz.rs に lu ネスト最悪深度の境界テストを 1 本追加
  (fuzz の内訳 8→9。単体 298→299)

## v0.116 で追加(NA 直後 CAI の UI 読みと s_marks 直後 free スロット)

### 報告文の修正(gi'e 接続第2枝の na sai ka'e muvdu)

報告文「.i lo se kecti cu sligau lo rebla tai lo nu tolgei kei gi'e na sai
ka'e muvdu」が ours 拒否・z0 受理。切り分けると原因は `na sai`(NA+CAI)。
z0 の木の実構造は NA + post_clause の free(sai を UI 読み)+ tag(ka'e)+
selbri であり、NA と CAI の結合単位ではない(zantufa は CAI スケール語を
UI 語彙に包含し、NA の直後に post_clause の free を取る)。

### 実測(NA 6 種×CAI 5 種×CAhA 有無の全 60 形+裸 CAI 3 位置+融合形)

- `mi {na,na'e,to'e,no'e,je'a,ja'a} {sai,cai,ru'e,cu'i,sa'e} [ka'e] muvdu`
  の全 60 形を z0/z1/maf 実測: na/ja'a 系は参照 3 種全受理。
  na'e/to'e/no'e/je'a + CAI + ka'e のみ z1 が拒否の参照分裂
  (z0=1/z1=0/maf=1)。裸 CAI(`mi sai muvdu`/`sai mi klama`/
  `mi klama cu'i`等)・CAI+CAI(`mi sai cai muvdu`)・UI+NAI 空白形
  (`mi na sai nai muvdu`)・融合形(`sainai/cu'inai/cuhinai` 両位置)は
  参照 3 種全受理
- z0 の木: 「mi na sai ka'e muvdu」= NA(na)+free(UI sai)+tag(ka'e)+
  selbri。「mi na'e ba'e mutce」= NAhE(na'e)+tanru 前置 BAhE(ba'e)+
  selbri(free ではない)

### 実装(UI_core 3 形+UINAI_joint 3 形+s_marks 直後スロット)

- UI_core に `sai` `cu'i` `cuhi` を収録(cai/ru'e/sa'e/ruhe は収録済みの
  ため残り。cai/ru'e と同様の二重収録)。ui_free の UI+CAI/NAI 後置と
  frees_mid 経由の CAI+CAI 連続も自動で有効化
- UINAI_joint に `sainai` `cu'inai` `cuhinai` を収録(参照 3 種全受理実測)
- selbri に s_marks 直後の `(sp1 ~ !BAhE_clause ~ frees_mid)?` スロットを
  追加(従来 sp1? が空白を先食いして届かなかった)。BAhE は z0 が後続
  tanru_unit の前置に束ねるため除外。frees_mid は silent のため既存入力の
  木は不変(ただし na'e+ba'e 形は free 奪取にならないことを回帰ピンで固定)

### 掃引結果(v0.116。プローブ 2,228 行=2,201 行+新規 27 行)

- ours ok 2,123 / z0 ok 1,986 / z1 ok 1,986 / maftufa ok 1,955。
  GAP 候補 32 件、OVER 候補 145 件(ともに不変)
- ok→err ゼロ、既存行の err→ok ゼロ、参照列の変化ゼロ。新規 27 行は
  24 行が全緑+3 行が z1 分裂の正確な記録(ours=z0=maf ok・z1 err)
- tests/syntax.rs に 1 テスト追加(報告文ピン+NA×CAI マトリクス+裸 CAI
  3 位置+融合形+BAhE 除外ピン)。cargo test 317 全パス

## v0.115 で追加(UI+NAI 無空白結合形と旧 UI 語 h 変体の補完)

### UI+NAI 無空白結合形の収録(UINAI_joint の拡張・文頭 GAP 100 行の緑化)

UI_core の全語形 × nai の無空白融合形を z0/z1/maf で全数実測し、受理される
125 形を UINAI_joint に追加(既存 26 形の選言順序は最優先に維持。語形間に
前置衝突がないこと(どの語形も他の語形の接頭辞でない)を全数確認)。
融合形は個別 @ トークンとして定義する必要がある(UI_core ~ NAI_core の
連結は語境界ガードで失敗するため)。不収録は kiahanai のみ
(参照 3 種とも拒否の整合。ki'a の結合形は参照の UI 語彙に無い)。
CAI 後置との組合せ・nai 二重は z0 実測の上で整合を確認。
副次効果: 文末形(「mi klama dainai」)は従来 fuhivla 緩さの tanru 誤読による
偶然 ok だったが、UINAI_joint の正規 UI 読みに統一(木ピンで固定)。

### 旧 UI 語 h 変体 26 形の収録('↔h 併記規約の完成)

sahe/ohi/taho/pehi/juho/uhi/uhu/ruhe/eho/ehe/ahe/iha/zoho/ahu/oho/uha/
uhe/ihe/behe/behu/dihai/fauhu/gehe/liha/nihau/suha を
UI_core に追加(全形を z0/z1/maf の自由修飾語 2 位置+空白形で全数再実測の
上で収録)。ruhe は CAI_core と共通語形のため二重収録
(cai/ru'e/cu'i と同様。UI+CAI スロットの既存木は不変)。

### 描述直後と selbri 時制後の自由修飾語スロットの追加

z0 は全語の直後に post_clause の free を取るため「lo .ui broda ku」
「mi pu .ui klama」型が参照 3 種すべて受理される(pre-existing GAP)。
desc と selbri に (sp1 ~ frees_mid)? スロットを追加(z0 同型の読み)。
既存入力はスロットが空で失敗した後 sp1 を付け直すだけのため木・受理集合
ともに不変(frees_mid は silent)。UI+NAI 融合形を tanru_unit の BRIVLA
ガードで排除した結果「lo dainai broda ku」「mi pu dainai klama」型は
このスロットで救済される。

### 掃引結果(v0.115。プローブ 2,201 行=2,097 行+新規 104 行)

- ours ok 2,096 / z0 ok 1,959 / z1 ok 1,962 / maftufa ok 1,928。
  GAP 候補 32 件、OVER 候補 145 件(不変)
- ok→err ゼロ、err→ok 100 行(文頭融合 GAP の緑化)、参照列の変化ゼロ、
  新規 104 行は全行 ours=参照 3 種 ok(融合文頭 26 / h 変体素形 26 /
  h 変体融合文頭 26 / 描述・selbri free スロット他)
- tests/syntax.rs に 6 テスト追加(融合文頭全数・文末正規 UI 読み・
  CAI 後置と空白形の不変・h 変体・free スロット)。cargo test 316 全パス

## v0.114 で追加(tu'a の KOhA → LAhE 移設と不閉鎖 to の実測記録)

### tu'a の誤束ねの修正(LAhE_core への移設・実文の意味論が変わる修正)

`tu'a` が KOhA_core に含まれていたため、「tai tu'a lo since」が
tagged(tai) + KOhA(tu'a) + desc(lo since) の**2つの別項**と読まれていた
(タグが tu'a だけに付き lo since が別項)。z0(zantufa-0.9999.js)は
LAhE(tu'a) + sumti(lo since) の**1項**で束ね、タグが tu'a 項全体にかかる
——実文の意味論が変わる問題。また裸 tu'a(後続 sumti なし)が KOhA 項として
過剰受理だった(「tu'a klama」「mi tu'a klama」「mi viska tu'a」の OVER 3 行)。

### z0 の文法実測(LAhE 形態論と sumti_5)

- zantufa-0.9999.js の LAhE 形態論は `tuha`(tu'a の h 表記)/`lahe`/`luhe`/
  `luho`(lu'o)/`luhi`(lu'i)/`luha`(lu'a)等を含み **tu'a は LAhE 語**。
  sumti_5 の LAhE 枝は `LAhE relative_clauses? sumti LUhU_elidible` の 1 形で
  **後続 sumti が必須**(裸 LAhE 語の項は存在しない)
- z0 の木(実測): 「tai tu'a lo since」= tag_term(tag(BAI tai), sumti_5(
  LAhE_clause(tu'a), sumti(lo since), LUhU elided))。「lu'e tu'a lo si'o
  broda」= LAhE(lu'e) → LAhE(tu'a) → sumti(lo si'o broda) の入れ子。
  「mi djica tu'a do klama」は z0 も拒否(LAhE 項の後の selbri が届かない)
- 注記(LAhE 直後 relative の構造差): z0 の LAhE 枝は
  `LAhE relative_clauses? sumti` で LAhE 語の直後に relative_clauses の
  スロットを持つが、本実装の lahe_sumti にはこのスロットがない
  (tu'a 移設前から la'e/lu'e と共通の既知仕様)。ただし実測では
  「tu'a noi broda ku do」「la'e noi broda ku do」型は z0/z1/maf とも
  **拒否**されるため挙動上の差分は発生していない(ours も拒否で整合)。
  内項の sumti 側に相対節を取る形「tu'a lo broda noi mi viska ke'a」は
  既存の sumti 経路で z0/z1/maf・ours とも受理
- h 変体 `tuha` は z0/z1/maf 実測で「tuha do」型が受理・「tuha klama」型は
  拒否。v0.113 まで tuha は本実装で未収録(「tuha do」が err の既存 GAP)
  だったため、'↔h 併記規約に従い LAhE_core に併記(GAP 解消)

### 実装(lahe_sumti 経路)

- KOhA_core から `tu'a` を削除し、LAhE_core に `tu'a`/`tuha` を追加。
  既存の lahe_sumti(LAHEDI_joint | LAhE_clause ~ sp1 ~ sumti ~ (sp1 ~ LUhU)?)
  がそのまま経路となり、規則の変更はなし(la'e/lu'e と同型の木)
- 木形状は z0 の sumti_5(LAhE + sumti + LUhU elided)と同型:
  「mi djica tu'a do」= tail term の sumti(lahe_sumti(LAhE_clause(tu'a),
  sumti(do)))。「tai tu'a lo since」= tagged(BAI tai, lahe_sumti(tu'a, lo since))
  の 1 項(タグが束ねる)。「tu'a lo broda cu brode」= 項(LAhE lo broda)
  + cu 文。明示閉鎖 lu'u も受理(la'e/lu'e と同一スロット)
- 受理集合の変化は KOhA 読みの裸 tu'a と、z0 が拒否する周辺形のみ(下記掃引)

### スイープ結果(v0.114 実施。プローブ 2,062 → 2,097 行)

- **既存行の受理変化は ok→err 3 行のみ**(いずれも KOhA 読みだった頃の
  OVER 行が z0 整合の拒否に変わる意図的縮小): `tu'a klama` / `mi tu'a klama` /
  `mi viska tu'a`(参照 3 種とも拒否)。**err→ok はゼロ**、参照列の変化もゼロ
- 新規 35 行: LAhE 語彙プローブ 4 行(tu'a/tuha × `mi viska {W} di'u` /
  `mi viska {W} lo broda ku`。ours ok 4 行・参照 3 種との不一致ゼロ)/
  裸 tu'a 拒否ピン 5 行(mi djica tu'a / tai tu'a / tu'a zo'u broda /
  mi cu tu'a lo since / tuha klama。参照 3 種とも拒否の整合)/
  tu'a 束ね受理ピン 12 行(tai tu'a lo since / mi djica tu'a do lu'u /
  lu'e tu'a lo si'o broda / mi klama tu'a lo zdani broda / tu'a do zo'u broda /
  tuha do 等。全行 ours ok = 参照 3 種 ok)/ to 引用ピン 9 行(下記)/
  既存 GAP 候補の記録 5 行(下記)
- 集計: total=2,097 / ours_ok=1,892 / z0_ok=1,855 / z1_ok=1,858 / maf_ok=1,824。
  GAP 候補 127 → 132 件(+5 は pre-existing の記録行。下記)/
  OVER 候補 148 → 145 件(−3 は裸 tu'a の OVER 解消)
- z0 が受理する既存の tu'a 形はすべて受理し続けることを確認:
  描述の tanru が伸びる形(mi klama tu'a lo zdani broda。z0 は内項 lo zdani
  broda の読み)、prenex 項(tu'a do zo'u broda)、lu'u 閉鎖、lu'e 入れ子、
  zo'e/ri 内項(mi djica tu'a zo'e / mi djica tu'a ri)は既存経路で受理

### 不閉鎖 to の実測(z0 と整合のため修正なし)

「mi to klama .i do broda」型の不閉鎖 to が黙って後続(次文まで)を呑む問題
について、z0 の文法と挙動を実測:

- z0 の free 第5枝は `TO_clause text TOI_elidible`。text は入力の残り全体を
  貪欲に吸収するため、**z0 も不閉鎖 to で後続の .i と次文まで呑む**
  (z0 の木: free → TO_clause → text 内に .i do broda が内包される実測)
- TOI は常に省略可で、**toi も text の word として先に消費されるため
  閉鎖として機能しない**(「mi to klama toi do broda」の toi は引用内)
- 本実装の to_quote(TO_clause ~ word* ~ TOI?)と**受理集合・挙動とも完全一致**
  (to 系 12 形を z0/z1/maf 交叉実測し全形一致。うち 9 行を構造プローブと
  して固定: gap_sweep_results.csv の line_no 2084〜2092。未採用の 3 形は
  文頭引用+後続文(to mi klama toi mi broda)/ selbri 直後引用
  (mi klama to broda toi brode)/ 参照 3 種とも拒否の抽象内形
  (mi cusku lo se du'u to broda toi kei)で同型のため省略)。
  修正は行わず、意味論上の注意のみ記録: 閉じられない to は発話の残り
  全体を引用する(z0 と同じ挙動)

### 既知差分の追加記録(スイープの GAP 候補に 5 行追加・pre-existing)

tu'a/LAhE 関連で z0 が受理し本実装が拒否する既存差分(本版の変更とは無関係。
v0.110 の残差記録と同様に構造プローブとして比較表に固定):

- 呼格+LAhE sumti 引数(3 行): z0 の free 第3枝は vocative sumti?
  DOhU_elidible で sumti に LAhE 形も取れる。「ju'i tu'a do」
  「farlu ju'i tu'a do cnita」「mi klama gi'e ju'i tu'a do cadzu」。
  本実装の vocative_arg/vocative_koha は CMEVLA/desc/KOhA のみで LAhE 形が
  届かない(la'e/lu'e でも同様の既存 GAP)
- mex operand の LAhE(2 行): z0 の operand は
  (LAhE_clause / NAhE BO_clause) mex LUhU_elidible を取るため
  「li la'e pa su'i re lohO」「li tu'a pa su'i re lohO」型が受理される。
  本実装の mex_operand には LAhE 枝がない(la'e/lu'e でも同様の既存 GAP)

### テスト・ドキュメント

- 統語 216 → 218: `tu'a_は_lahe_経路で後続_sumti_を束ねる`(LAhE 経路の
  exact-tree ピン・裸 tu'a 8 形の拒否ピン・tuha/prenex/lu'u/入れ子)+
  `to_引用は_不閉鎖でも_参照実装と同じ挙動`(to 9 形の受理ピン+木形状)。
  既存ピン更新: koha_tua_dei → koha_dei(tu'a の KOhA ピンを削除)/
  fai_と_faha_fa_a_補完_対象文と受理 の tai tu'a ピンを lahe_sumti 読みに更新
- docs/coverage.md: KOhA 行から `tu'a` を削除、LAhE 行に `tu'a` `tuha` を追加
  (coverage_doc テストが語彙検証で強制)。README/README.en の LAhE 記述も同期
- プローブ生成器: 構造プローブに v0.114 の tu'a/to ピンと GAP 候補記録 5 行を
  追加(2,062 → 2,097 行)

## v0.113 で追加(長大トークンのリソース保護: 語長上限 MAX_TOKEN_CHARS=50)

長い lujvo 風トークン(kaprenmit 型 CVC 連鎖)で 2 つの致命的挙動が
確認された問題の堅牢化。pest 文法は変更せず、lib.rs の前処理で対処した
(既存受理集合への影響はコーパス・参照実装テスト全体でゼロ)。

### 実測に基づく問題の特定

- **指数時間**: pest は packrat 型でない(zantufa-0.9999.js(z0)の
  PEG.js 生成コードは位置ごとにメモ化)ため、rafsi 分割のバックトラックが
  時間の大半を占める。規則別の直接解析で `Rule::lujvo`(brivla の
  `initial_rafsi* ~ brivla_core` の分割列挙)に集中することを確認
  (54文字で 2.0 秒。fuhivla 単独 0.05 秒・cmevla 0.003 秒に対し 40〜600 倍)。
  悪化率は 9文字あたり約 2.1 倍(18字=0.05s / 54字=2.1s / 72字=9.2s)
- **スタックオーバーフロー**: 規則の相互再帰が語長に比例して深くなり、
  10万字トークンで `--classify`(classify_word)・`--split-lujvo`
  (lujvo::decompose の seg 再帰)が SIGABRT(exit 134)。
  panic ですらない異常終了だった
- **z0 も破綻**: z0 に長トークンを食わせた実測では、CVC 連鎖は
  約9300文字まで受理(境界は 0.26 秒)するが、それ超は JS スタック上限の
  `RangeError: Maximum call stack size exceeded` で例外終了。
  つまり z0 の受理上限は文法上の保証ではなく環境依存のスタック残量であり、
  「z0 が受理する長さすべてを受理し続ける」ことは原理的に不可能。
  堅牢性優先の方針を採用した

### 実装(pest 変更なし・前処理でのガード)

- `MAX_TOKEN_CHARS = 50`: 1語(空白・ポーズ区切り)の最大文字数。
  上限ちょうどの最悪トークンでも約1.5秒で解析完了(実測カーブから設定)。
  正当な語の実用最長は本リポジトリ全テスト・コーパス・gerna_cipra
  テストデータ全体で 29 文字(`oicairo'aro'ero'iro'oro'ure'e`)に対し
  十分なマージン
- `parse()`: ZOI 正規化・SI/SU 消去の**後に**語長検査を適用。
  zoi 本文内の長い語(URL 等)は zo'e に置換済み、「長い語 si」は
  消去後に語が残らないため、いずれも従来どおり受理
- `classify_word()`: 上限超過は形態論規則を試行せず `unknown` を返す
  (--classify・--stats の保護)
- `lujvo::decompose()`: 上限超過は高速に `DecomposeError`(seg 再帰の保護)
- エラーは既存の CustomError 経由で「語が長すぎます(上限 50 文字):
  「(先頭12文字)…」」形式のクリーンエラー(トークン全体はダンプしない)

### テスト(tests/fuzz.rs に追加・5件)

- 上限ちょうど(50文字)の最悪トークンが時間内に完了
- 上限+1(51文字)がクリーンエラー・10万字も即座にエラー
- classify_word / decompose の長大入力でパニックしない(旧 SIGABRT 回帰)
- ガードが前処理後に効くこと(zoi 本文 300 文字・消去済み長語の受理)
- 上限以内の長い cmavo 連鎖(UI 29 文字・y 連鎖 48 文字)の受理維持

## v0.112 で追加(PA_word の欠落語補完と fi'u の数詞読み統一)

ユーザー報告「.i .ei mi xruti gi'e jitro so'o nuncatra noi mi pu minde」が
拒否される問題の修正。二分法で失敗点は「so'o nuncatra」(quant_selbri)、
**PA_word に so'o が欠落**(so'a/so'e/so'i/so'u は収録済み)と特定。
全数調査の結果、CLL PA の欠落語を一括補完した。

### 語彙追加(PA_word。z0 実測: 全語が quant_selbri・描述内 mex・
li 内 mex・PA_seq 連結形で受理)

- 量化系: `so'o` `rau`(enough)`du'e`(too many)`mo'a`(too few)
- 数学定数系: `te'o`(指数 e)`ka'o`(虚数単位)`pai`(円周率)`ci'i`(無限大)
  `tu'o`(空被演算子)`ce'i`(パーセント)`fi'u`(分数スラッシュ)
- h 変体(z0 の '↔h 同値規約。個別に quant_selbri プローブで受理を実測してから収録):
  `soho` `duhe` `moha` `teho` `kaho` `cehi` `fihu` `suho` `jihi`
- 接頭辞衝突への注意: pai は pa の、ci'i は ci の、so'o/soho は so の
  **前に**配置(word_boundary 失敗後に選択肢は再試行されないため。教訓6と同型)

### ni は PA に収録しない(競合実測の結論)

`ni` は NU(抽象詞)と同形。実測の結果、**PA への収録は行わない**判断:

- z0/z1 の PA 形態論(生成パーサーからの語形抽出)に ni は存在しない(NU のみ)
- z0 は「ni prenu cu klama」を**拒否**(ours は NU 抽象の項 + cu 文として
  既に受理。PA 収録はこの OVER を拡大するだけ)
- z0 の「ni broda」「lo ni broda」の受理は NU 抽象を tanru_unit に取る読みで、
  本実装の nu_form / abstraction 経路と**同型の木**が既に成立している。
  PA 収録すると sumti_core の選言順(quant_selbri が abstraction より先)と
  sumti_tail の mex 枝が先取りして既存の抽象の木が壊れる
- PA_seq に nisu'o 等の z0 非受理連結形が入り込む(z0 は「nisu'o prenu cu
  klama」を NU ni + 無ポーズ su'o の別読みでのみ受理。本実装は無ポーズ
  cmavo 連結未対応のため既知 GAP のまま)
- 既存抽象の木の不変は tests/syntax.rs「ni_は数詞に収録せず抽象の木を維持」が
  exact-tree 相当のピンで保証(lo ni broda = desc + nu_form、ni broda =
  nu_form 単独文、ni prenu cu klama / ni broda cu brode = abstraction 項 + cu)

### fi'u の mex 読み(z0 同期の木変化・1件のみ)

fi'u を PA に収録した結果、空白区切りの「li re fi'u ci」は z0 と同様に
PA 連続の**数詞**(number = PA_clause+)として読まれるようになった
(z0 の実木では演算子分割は存在しない。fi'u は z0 でも PA)。これにより
mex 内の FIhU 演算子枝は空白区切り形では到達しなくなるが、camxes 互換の
予備として維持。既存テストで FIhU_core の出現をピンしていた
「mex_fihu_と_前置単項」のみ PA_core \"fi'u\" ピンに更新(受容の意図は不変)。

### PA_seq の実測(無ポーズ連結)

PA_seq は PA_word+ のため新語は自動的に連結形へ反映されるが、**収録語は
z0 の PA 形態論に含まれる語のみ**のため連結形も z0 整合(so'opa / paso'o /
so'ore / te'opa / ka'opa / paipai / tu'ono / ce'ire / refi'u / sohore /
suhopa / jihipa / fihure 等を z0 実測 ok)。z0 が拒否する連結形
(nisu'o。ni 非収録により自然に排除)は PA_seq に入らないため
**語形ガード(数字系/量化系の分離)は不要**と確認。

### スイープ生成器の修正(PA 語彙プローブの復活)

generate_gap_probes.py の委譲取得が `len(refs) == 1` 条件のため、
PA_core = @{ PA_word ~ &word_boundary } の 2 参照で委譲が落ちており、
**PA クラスの語彙プローブが 1 件も生成されていなかった**。委譲先を
^"..." リテラルを持つ規則から選ぶ方式に修正し、PA 51 語 × 項3位置 =
153 行のプローブを復活(総数 1,909 → 2,062 行)。

### スイープ結果(v0.112 実施。プローブ 2,062 行)

- **既存 1,909 行の ok→err はゼロ、err→ok もゼロ**(ni 競合・PA_seq 変更の
  主要ゲート合格。既存入力の受理集合は完全不変)
- 新規 153 行: ours ok 153 行。内訳: 参照 3 種全部 ok 88 行 /
  z0・maf ok・z1 のみ拒否 14 行(za'u/ma'u/ni'u/ce'i/cehi/fi'u/fihu の
  7 語 × 2 位置 =「{W} klama」「mi {W} klama」。za'u/ma'u/ni'u は
  CLL 標準 PA のため z1 の PA 未収録ではなく z1 固有の制約) /
  OVER(ours ok・参照全 err)51 行 = 裸数詞項「mi viska {PA}」の既存 OVER
  クラス(mi klama re と同型。v0.112 の新語に限らず PA 51 語すべてに
  一様に発生する既存受容で、生成器復活により初めて可視化されたもの)
- GAP 候補 127 件(変化なし)/ OVER 候補 97 → 148 件(+51 は上記の
  裸数詞項クラス)

## 既知GAP(全 12 件解消済み。tests/gap_tracker.rs は全テスト緑)

zantufa 系参照パーサー(z0 = zantufa-0.9999.js / z1 = zantufa-1.9999.js /
maftufa = maftufa-1.9999.js)が受理するのに本パーサーが拒否する形(GAP)を、
語彙×統語位置のプローブ行列で体系的掃引し、tests/gap_tracker.rs に
テストとして固定した。バッチ1(v0.108)・バッチ2(v0.109)・バッチ3(v0.110)
で全 12 件を解消。

### スイープ方法論

- プローブ生成: tests/data/generate_gap_probes.py が lojban.pest から全
  `*_core` 語彙リストを機械抽出し(coverage_doc.rs と同手法)、クラスごとの
  代表統語位置(タグ4位置 / 項3位置 / 自由修飾語2位置 / クラス固有テンプレート)
  に流し込む。加えて既知 GAP 候補と OVER 記録用の構造プローブ。
  生成物は tests/data/gap_probes.txt(1,909 行。v0.111 の語彙追加で +113、
  UI+NAI 無空白結合形テンプレートの追加でさらに +252 = UI 語形 126 ×
  文頭/文末の 2 型)
- 一括比較: tests/data/run_gap_sweep.sh が本パーサー(CLI `--lines` バッチ)と
  参照パーサー 3 種(tests/data/refparse.js。camxes_preproc.js を parse 前に
  適用)を走らせ、比較表 tests/data/gap_sweep_results.csv
  (1,909 行 × {ours, z0, z1, maftufa})を出力
- 掃引結果(v0.111 実施。プローブ 1,909 行=v0.110 の 1,544 行+
  v0.111 新語彙 113 行(ce'u/cehu/zi'o/ziho の項3位置 12 行、UI 新語
  43 語形の自由修飾語2位置 86 行、構造プローブ 15 行)+ UI+NAI 結合形
  252 行。既存 1,657 行の受理集合は変化なし):
  ours ok 1,717 / z0 ok 1,723 / z1 ok 1,740 / maftufa ok 1,692。
  GAP 候補(参照 ok / ours err)127 件、OVER 候補(ours ok / 参照全 err)97 件。
  v0.111 語彙追加直後(1,657 行)からの変化は UI+NAI 結合形テンプレートの
  GAP 追加 100 行(文頭形。STATUS 次バッチ課題参照)**のみで ok→err はゼロ**、
  OVER 増減もゼロ。新規 113 行は ours ok 113 行で参照 3 種との不一致ゼロ
- 採用基準: 参照パーサーが受理する正当なロジバン形のみ。次の 4 クラスは
  意図的差分として GAP に入れていない(比較表には残る):
  - z0/z1 のみの緩受理: 裸 mo'i・fe'e のタグ位置(maftufa は拒否)
  - 無ポーズ隣接単語: caku mi klama 等(README 記載の意図的非対応)
  - v0.97 で見送りを明記した尾部形 quantifier+sumti(lo pa le gerku ku。
    拒否ピンあり)
  - 差分なしの確認項目: 抽象詞 li'i/su'u/ni と逆参照 ri/ra/ru は本パーサーも
    受理、je'i は z0/z1 が未収録(ours ok の逆差分)、na'e bo broda は
    既存経路で受理、「数式+mai」の mex 全体形は参照 3 種も拒否のため
    GAP は不在

### GAP 一覧(12 件。tests/gap_tracker.rs の各テストが 1:1 対応。✅=解消済み)

| 入力 | z0 | z1 | maftufa | 原因推定 |
|---|---|---|---|---|
| lo byklesi ku / mi byklesi / lo cyklesi ku | ok | ok | ok | ✅ v0.110 解消。tanru_unit にレタル接頭融合語枝(BY_prefix+BRIVLA)を追加 |
| fa nai mi klama / mi klama fa nai / fai nai mi klama | ok | ok | err | ✅ v0.108 解消。tagged の FA 枝に NAI 後置+sumti 省略(裸タグ項)を追加 |
| mi ji do klama / do ji mi broda | ok | ok | ok | ✅ v0.108 解消。ji を JOI_core に収録 |
| va'u mi klama / se va'u mi klama / mi klama se va'u lo nu broda | ok | ok | ok | ✅ v0.108 解消。va'u/vahu を BAI_core に収録 |
| farlu ju'i co cnita(.oi ta ca'o… も同形) | ok | ok | ok | ✅ v0.109 解消。tanru_post に co_post 枝を追加 |
| mi klama bo cadzu | ok | ok | ok | ✅ v0.109 解消。tanru_link に裸 BO 枝を追加 |
| mi broda joi brode(jo'e/fa'u/ku'a/jo'u/johu も同形) | ok | ok | ok | ✅ v0.109 解消。gihek_link に gihek_joik(JOI/BIhI)を追加 |
| mi viska lo broda vu'o noi mi klama | ok | ok | ok | ✅ v0.110 解消。sumti に (VUhO relative_clauses)? スロットを追加 |
| mi pu nai klama / mi ba nai klama / mi ca nai klama | ok | ok | ok | ✅ v0.108 解消。tense_mark に NAI 後置を追加 |
| mi cusku zoi gy. broda .gy | ok | ok | ok | ✅ v0.110 解消。normalize_zoi(lib.rs)の区切り語一致をポーズ記号除去形に変更 |
| mi jai se gau broda / mi jai se gau klama lo zdani | ok | ok | ok | ✅ v0.109 解消。JAI 枝に (SE \| NAhE)+ 変換タグ経路を追加 |
| mi klama ke lo zdani broda ke'e | ok | ok | ok | ✅ v0.110 解消。term に項レベル ke グループ(KE term+ KEhE?)を追加 |

### gap_tracker の扱い

- tests/gap_tracker.rs は GAP ごとに 1 テスト(計 12)。全 12 件とも
  受理をアサートするテストとして緑化済み(v0.108/v0.109/v0.110 で順次解消)。
  既存テストの拒否ピンは解消時に受理ピンへ更新済み
  (mi klama bo cadzu は v0.109、lo byklesi ku は v0.110 で更新)
- 再実行手順: `bash tests/data/run_gap_sweep.sh`
  (gerna_cipra の clone 先を GERNA_CIPRA_JS で指定可。既定
  /tmp/opencode/gerna_cipra/js)

## 既知差分(OVER: 本実装が受理し zantufa 参照 3 種が拒否する既存クラス)

スイープ(tests/data/gap_sweep_results.csv、2,097 行)の OVER 候補
(ours ok・z0/z1/maftufa すべて err。v0.114 時点で 145 行)のうち、
これまで版ごとの記録に分類されていなかった 31 行を既知差分クラスとして
整理した(調査で判明した 34 行のうち tu'a 関連 3 行は v0.114 で解消済み)。
いずれも「zantufa は拒否 / ours は受理」の差分で、CLL 規範・歴史的経緯
(移植元実装・camxes の語彙/構造の踏襲)に基づく**維持判断**
(受理集合を変更しない)であり、修正対象とはしない:

- **ZEhA 区間語の裸タグ受理**(20 行): `mi bi'o klama` / `mi klama mi'i
  lo zdani` 等。zi'i/zihi/bi'o/biho/bi'i/bihi/mi'i/mihi(ZEhA_core。
  BIhI 区間語 bi'o/bi'i/mi'i を含む)を selbri 前タグ・尾項タグ位置の
  間隔タグとして受理する。z0 の time_interval は `pu bi'o ba` 型の
  両端オフセット付きのみ受理するため裸形は拒否。語彙は v0.22 導入済みで
  裸タグ位置の過剰受理が未記録だった。位置別の内訳: selbri 前タグ
  (mi {W} klama)・尾項タグ+sumti(mi klama {W} lo zdani)の 2 位置は
  8 語形すべて OVER、selbri 前タグ+sumti(mi {W} lo zdani klama)・
  文頭タグ+sumti({W} lo zdani klama)は zi'i/zihi のみ OVER
  (bi'o/bi'i/mi'i 系は `mi bi'o lo zdani klama` 型で参照 3 種すべて受理。
  文頭形は maftufa も受理するため OVER 外)。
  CLL でも間隔タグは時制・空間タグの正当な語彙のため受理を維持
- **soi 自由修飾語**(3 行): `mi klama soi vo'a vo'e [se'u]` 型。
  ours の soi_free(SOI + sumti(+sumti)(+SEhU 省略可))は camxes/CLL の
  free の soi 形相当で、CLL 的には ours の読みが正しい。zantufa の
  free は soi を含まないため拒否。camxes/CLL 準拠の維持判断
- **CEhE 項区切り**(2 行): `mi klama ce'e do`(+h 変形 cehe)。
  v0.41 で camxes terms_2(term (CEhE term)*)準拠で導入。
  zantufa は CEhE を語彙に持たないため拒否。
  歴史的経緯(camxes 準拠)に基づく維持判断
- **kiaha**(2 行): `kiaha mi klama` / `mi klama kiaha`。旧 UI 語 ki'a
  (混乱)の h 変形で ours の UI_core が併録するが、z0 の UI 語彙は
  ki'a(kiha)のみで kiaha を持たない(kiahanai も参照 3 種とも拒否。
  v0.111 掃引で記録済み)。h↔' 規約の適用を優先した維持判断
- **miahi**(3 行): `miahi klama` / `mi miahi klama` / `mi viska miahi`。
  KOhA_core の実験語 mi'ai の h 変形で、z0/z1/maf は mi'ai を受理するが
  miahi は語彙に持たない(h↔' 規約の適用範囲の揺れ)。ours は規約どおり
  併録する維持判断
- **CAhA nu'a**(1 行): `mi nu'a klama`。nu'a は CLL 標準語彙(NUhA。
  selbri を数理演算子として前置する旗)で、ours は移植時から
  CAhA_core に収録。z0 は NUhA を廃して nu'a を ME 系の語彙に移して
  いるためタグ位置では拒否。CLL 規範に基づき ours が正の維持判断

OVER 候補の残りは既存の記録済みクラス(li … du 型の mex 受容(v0.101)、
裸数詞項 `mi viska {PA}` クラス(v0.112)、レタル接頭融合語の無ポーズ隣接
(v0.110)、`lo kuku ku` 型 CVCV 短形残部(v0.110)、lo aburobu ku
(v0.106)、mi pu bo ge broda gi broda 型(v0.107))と je'i(z0/z1 が
未収録の逆差分。掃引方法論の節参照)で、いずれも記録済み。

## v0.111 で追加(KOhA の ce'u/zi'o と CLL 標準 UI の欠落語彙)

ユーザー報告「.i sy mintu lo purdykurji lo ka ma kau tarmi ce'u .i clani
kurfa gi'e plita gi'e se kojna lo xance jo'u lo jamfu」が拒否される問題の修正。
二分法で失敗点は「lo ka ma kau tarmi ce'u」、さらに「mi klama ce'u」も拒否
(z0/z1/maf 実測: lo ka tarmi ce'u / mi klama ce'u / mi klama zi'o /
ma klama ce'u は全て参照 ok)から、**KOhA_core に ce'u(ラムダ変数)と
zi'o(消去項)が欠落**していると特定。

### スイープの盲点の教訓

語彙抽出式スイープ(generate_gap_probes.py)は文法に存在しない語を
プローブできない。ce'u/zi'o は「文法に無い語」ゆえに従来の掃引では
検出不能だった(教訓: 参照パーサー側の語彙リストとの差分突合を
追加の検証経路とする)。v0.111 では zantufa-0.9999.js.peg の
KOhA/UI 語彙リストを h→' 変換の上で本実装と突合し、欠落候補を
全数プローブして仕分けした。

### 語彙追加

- KOhA_core: `ce'u`/`cehu`(ラムダ変数)と `zi'o`/`ziho`(消去項)。
  z0/z1/maf とも項位置で受理の実測。h 変体は規約どおり併記
- UI_core(CLL 標準 UI の欠落 23 語+h 変体 20 語形の計 43 語形):
  `a'i`(努力) `a'o`(希望) `ca'e`(CAhE 前置肯定) `dai`(共感)
  `e'a`(容易) `io`(敬意) `ju'a`(断定) `ke'u`(反復) `le'o`(不寛容)
  `li'o`(省略) `o'e`(親密) `pau`(問い) `pa'e`(議論好み) `ra'u`(特に)
  `ro'a`(社会的反応) `ro'o`(精神的反応) `se'a`(自己充足) `si'a`(類似)
  `ta'u`(明示) `ti'e`(伝聞) `to'u`(省略表現) `va'i`(言い換え)
  `vu'e`(徳)。アポストロフィ語は h 変体(ahi/aho/cahe/eha/juha/kehu/
  leho/liho/ohe/pahe/rahu/roha/roho/seha/siha/tahu/tihe/tohu/vahi/vuhe)
  も併記(z0 実測受理形)
- 追加条件は従来どおり「z0/z1/maf で受理かつ ours が拒否」。ki'e は
  COI(呼格)、se'o は BAI、cai/sai/ru'e/cu'i は CAI として既存収録済み
  のため重複収録しない

### プローブで確認して不採用とした語

- `cei`(CLL の代入。z0/z1/maf とも「ko'a cei broda」は拒否。ただし
  zantufa は selbri_4 末尾の「mi broda cei brode」形を独自受理するため
  次バッチ課題として記録)
- `sa'i`(BAI)/`pe'o`(PEhO)/裸 `sei`(SEI は inner_sentence 必須の現行契約):
  参照 3 種とも拒否のため不採用(ours も拒否で整合)
- `bu'u` 裸タグの尾項位置(「mi klama bu'u」): 参照 3 種とも受理するが
  本実装は FAhA タグに裸尾項経路がない構造差分。次バッチ課題

### 次バッチ課題(zantufa 実験語彙・参照語彙差分の残り)

- zantufa KOhA 実験語(2015-08-20 追加分。全て z0/z1/maf の項位置受理・
  ours 拒否を確認済み): bo'a bo'e bo'i bo'o bo'u ca'au da'ai da'au da'e
  de'e dei'e dei'ei dei'o dei'u di'au di'e di'ei di'oi do'ei do'i
  kau'a kau'e kau'i lau'e lau'u mai'i mi'oi nau'u nei'o ri'au tu'oi xai
  zai'o zi'oi zu'ai zu'i'a
- zantufa UI 実験語(111 語形。全て自由修飾語位置(文頭・文末の両形)で
  z0/z1/maf 受理・ours 拒否を確認済み。初期列挙は 91 語で、
  zantufa-0.9999.js.peg の UI 語形リスト(h→' 変換)との全数突合により
  20 語形の列挙漏れを修正(漏れ分も文頭/文末両位置で再実測済み):
  a'a / au'u / ba'u / ci'au'u'au'i / cu'ei の母音スケール形 8 形
  (cu'ei'a cu'ei'ai cu'ei'e cu'ei'ei cu'ei'i cu'ei'o cu'ei'oi cu'ei'u) /
  fu'ei の母音スケール形 5 形(fu'ei'a fu'ei'e fu'ei'i fu'ei'o fu'ei'u。
  基本形 fu'ei は maftufa のみ拒否のため含めない) /
  ki'a'au'u'au'i / sai / cu'i。sai/cu'i は標準 CAI 語形だが zantufa は
  CAI スケールを UI 語彙に包含するため裸形の自由修飾語として受理され、
  ours は sai/cu'i を CAI_core(UI+強度)としてのみ収録するため拒否
  (なお cai/ru'e は ours の UI_core にも収録済みのため本差分には出ない)):
  a'a au'u ba'u bi'a bi'u bo'oi bu'a'a cau'i ci'ai ci'au'u'au'i cu'ei
  cu'ei'a cu'ei'ai cu'ei'e cu'ei'ei cu'ei'i cu'ei'o cu'ei'oi cu'ei'u cu'i
  dai'i dai'o dau'a dau'i de'ai de'au de'oi do'a do'ai doi'a fai'a fu'au
  fu'ei'a fu'ei'e fu'ei'i fu'ei'o fu'ei'u fu'i ge'ei i'o i'u ia'u ie'i
  je'au jei'u ji'au ji'ei jo'a ju'oi kai'a kai'e ke'e'u ki'a'au'u'au'i
  ko'oi koi'e lai'i li'oi mau'i mau'u me'ai mi'u moi'i mu'a na'i
  na'oi ne'au ne'e oi'a oi'o oi'u pe'a pe'ai pei'a pei'e pei'o ra'i'au
  re'e ri'e ro'e ro'i ro'u sa sa'a sai sa'u se'i sei'i si'au ta'ei ta'oi
  te'i'o toi'e toi'o u'ai uai uau ue'i uei'e vei'i xa'a xa'a'a xa'i
  xai'a xau'e'o xau'o'o xo'o xu'u'i xy'y zai'a zi'a zi'ai
- 裸 `nai` の自由修飾語(「nai mi klama」。z0/z1/maf 受理・ours 拒否)、
  `ra'o` の自由修飾語(「mi klama ra'o」。z0/z1 受理・maftufa 拒否の
  参照分裂)
- UI+NAI の無空白結合形(文頭形「dainai mi klama」。z0/z1/maf 全 ok・ours 拒否)。**v0.115 で解消**(UINAI_joint に 125 形収録・文頭 GAP 100 行緑化。下記 v0.115 節)。
  v0.111 掃引で UI 語形 126 × 文頭/文末の全組合せを実測: 文頭形 126 行のうち
  GAP(参照 3 種全 ok・ours 拒否)100 行、ours ok 25 行(UINAI_joint 既存語形
  ta'onai/da'inai/ja'onai/ku'inai/po'onai/je'unai/la'anai/za'anai/ga'inai/
  zu'unai/ba'anai/ju'onai/cu'inai/ji'anai/ru'anai/ehinai/e'inai/ihinai/i'inai/
  ohanai/o'anai/kahunai/ka'unai/a'unai/u'inai/u'unai の 26 語形のうち UI_core
  語形と対応する 25 行)、参照 3 種とも拒否の整合 1 行(kiahanai。ki'a の
  結合形は参照の UI 語彙にも無い)。文末形(「mi klama dainai」)は 125 行が
  ref 全 ok・ours ok だが、ours は fuhivla 緩さの tanru 誤読による偶然 ok
  (v0.110 OVER 既知クラスと同族)で kiahanai のみ整合拒否。既存 UINAI_joint
  には uinai/einai が未収録の欠落クラス。プローブテンプレートに両位置を
  追加済み(掃引の GAP は 27 → 127 行)
- 旧 UI 語の h 変体完成(26 形。全形 z0/z1/maf ok・ours 拒否を確認済み。**v0.115 で解消**。
  v0.111 の新 UI 語は ' / h 併記済みだが既存語は ' 形のみのものが残存):
  sahe(←sa'e) ohi(←o'i) taho(←ta'o) pehi(←pe'i) juho(←ju'o) uhi(←u'i)
  uhu(←u'u) ruhe(←ru'e。CAI_core 既存だが UI 単独自由修飾語位置では無効)
  eho(←e'o) ehe(←e'e) ahe(←a'e) iha(←i'a) zoho(←zo'o) ahu(←a'u)
  oho(←o'o) uha(←u'a) uhe(←u'e) ihe(←i'e) behe(←be'e) behu(←be'u)
  dihai(←di'ai) fauhu(←fau'u) gehe(←ge'e) liha(←li'a) nihau(←ni'au)
  suha(←su'a)
- zantufa の selbri 末尾 `cei` 前置(「mi broda cei brode」。z0/z1/maf
  受理・ours 拒否。CLL 形「ko'a cei broda」は参照 3 種とも拒否)

## v0.110 で追加(バッチ3 GAP 解消: 形態論・共有・前処理系)

語彙追加はなし(既存語彙の受理範囲拡張と lib.rs の前処理修正のみ。
coverage.md に変更なし。BY_prefix は既存 BY_core の語境界なし前置形で
新語彙ではないため coverage_doc の語彙抽出対象外)。

- 文法①(レタル接頭融合語): tanru_unit に BY_prefix 枝
  (BY_prefix ~ !(NAKU_joint) ~ BRIVLA_clause)を追加。
  GAP_レタル接頭lujvo_byklesi 解消。「lo byklesi ku」「mi byklesi」
  「lo dyjamynai ku」等。z0 実測(zantufa-0.9999.js):
  「by」は brivla ではなく、by+brivla の無ポーズ隣接(cmavo の
  post_word 経路)により sumti_tail 内で lerfu_string(by) + BOI(省略)
  + selbri(klesi) として解析される(形態論的な単一 brivla ではない)。
  本実装は無ポーズ隣接を一般にサポートしないため融合語トークンを
  tanru_unit として受理(木形状差異 = 既知クラス)。
  受理スコープ(z0/z1/maf 交叉・実測): 子音レタル17語+ 直後に無ポーズで
  続く有効な brivla。母音レタルは前置しない(「lo abu ku」参照 3 種とも拒否)。
  形態論ストレステスト(レタル×各種後続 80 行を z0/z1/maf 交叉実測):
  - 過剰受理チェック(レタル接頭×残部語形マトリクス 52 行+拡張 15 行を
    z0/z1/maf 交叉実測): 「lo abu ku」「lo ebu ku」「lo cybu'u ku」
    「lo byku ku」は拒否維持(z0 一致)。CVCV 4字/CVCVV 5字の stress なし
    短形残部(bynaku / bykuku / bynunu / byzozo / bypapa / bydada /
    cykuku / zykuku / bykukai 型)は BRIVLA_core が fuhivla/lujvo に誤
    マッチする実装上の性質のため新規 OVER になったが、
    !(cvcv_short_tail) ガードで参照 3 種整合の拒否に修正(v0.106 の
    lo abu ku 処置と同型。parent 81a54c2 で全形 err の確認済み=
    退行なしの縮小。NAKU_joint の点排除は CVCV 短形の一般排除に統合)
  - ガードの範囲(過剰縮小の回避。実測ピン): word_boundary 付き短形の
    み排除するため 5 字以上の有効 brivla 残部は受理維持
    (「lo bykukla ku」CVCCV gismu 形 / 「lo bynunkapi ku」正規 lujvo /
    「lo bynaselci ku」/「lo byduduki ku」/「lo bykukybu'e ku」
    CVCy lujvo 等)。6 字以上の無記名 CVCVCV 連鎖
    (「lo bykukula ku」型)の一括排除は z0 が受理する 6 字形
    (byduduki 型)を壊すため行わず残存(z0 err の残存 OVER。
    形態論レベルの stress 判定が次バッチ課題。接頭なし
    「lo kuku ku」型は pre-existing OVER)
  - 受理一致で読みが異なる形: 「mi suta byklesi」は z0 では su(消去)+
    ta(KOhA 項)への形態論分割読み(su_clause が intro に現れる)、
    本実装は tanru(suta, by+klesi)。受理一致の既知読み差異クラス
  - 受容優先の既知クラス(参照 3 種とも拒否・本実装は受理): brivla+レタル接頭
    融合語の無ポーズ隣接(「mi klama byklesi」「mi klesi byklesi」
    「mi klama gi'e byklesi」。tanru 2単位目・gihek 後・selbri 直後。
    非正規字形で実害なし。v0.95 タグ+BO / v0.107 mi pu bo ge と同型)
  - 残差(参照 3 種 ok / 本実装 err。次バッチ候補として記録): 多レタル接頭
    (lo byfyklesi ku。z0 は lerfu_string 連鎖)、関係節が続く形
    (lo byklesi noi broda ku。z0 は埋め込み lerfu 項+selbri+relative)、
    レタル+タグ cmavo 隣接(mi byta'e klama / mi byca klama。z0 は
    by 項+BAI/PU タグ+selbri)、項分離読みの各形(mi bynaku klama。
    z0 は by 項+naku 項+klama。lo bydudu ku / mi bydudu /
    lo bydudu klama。z0 は dudu を GOhA cmavo の無ポーズ隣接の
    tanru 読み。lo bysukai ku。z0 は lo/by を su で消去+残部 kai タグ項の
    フラグメント読み)は項分離読みの実装が要るため未対応
- 文法②(VUhO 後の関係節共有): sumti に
  (sp1 ~ VUhO_clause ~ sp1 ~ relative_clauses)? の第2スロットを追加。
  GAP_VUhO後の関係節共有 解消。z0 実測: sumti = sumti_1
  (VUhO relative_clauses)? の形で vu'o の直後は relative_clauses のみ。
  「mi viska lo broda vu'o noi mi klama」型(noi/poi/voi/GOI)。第2枝を
  連結ループの前に置き「vu'o noi …」を連結ループに食わせない設計。
  なお z0 は「vu'o + sumti」の項連結を持たない(「vu'o mi」は z0 err)が、
  既存の VUhO 項連結枝は pre-existing 受理のため維持(既知 OVER 差分)。
  連結後の二重関係節共有は z0 も拒否のため単一スロット
- 文法③(zoi 区切り語のピリオド正規化): lib.rs の normalize_zoi を修正
  (統語でなく前処理の問題)。GAP_zoi区切り語のピリオド正規化 解消。
  区切り語トークンの前後のポーズ記号(. , ! ?)は語の一部ではないため、
  比較前に両端から除去する(zantufa 形態論では区切り語は lojban_word で
  前後のポーズは spaces 側が消費。「gy.」/「.gy」/「.gy.」はいずれも
  「gy」に対応)。開き/閉じの片方だけポーズ付き・本文空・引用後継続も
  z0/z1/maf 実測 ok で受理。ポーズ記号のみの区切り語(「zoi . abc .」)は
  語形不正としてエラー。lib.rs 内の zoi 引用ユニットテストを拡充
- 文法④(項レベル ke グループ): term に KE_clause ~ sp1 ~ term ~
  (sp1 ~ term)* ~ (sp1 ~ KEhE_clause)? 枝を追加。
  GAP_ke_group内の項 解消。z0 実測: term_2 = KE term+ KEhE_elidible の
  項グループで「mi klama ke lo zdani broda ke'e」は tail_terms 内の
  term(KE + 描述 + ke'e)として受理される。複数項・KOhA・明示 ku・
  KEhE 省略も z0 受理実測で同時緑化。グループ内が sumti で始まらない形
  (「ke mi broda ke'e」「ke lo zdani broda ke'e brode」)は z0 も拒否。
  既存の tanru_unit 側 ke_group(KE selbri KEhE?)は selbri グループを
  先に処理するため木は不変
- 文法⑤(gihek の (NA? SE?) 前置): gihek_link の GIhA 枝と gihek_joik の
  両枝に (NA_clause ~ sp1)? ~ (SE_clause ~ sp1)? 前置スロットを追加。
  スイープ新記録 GAP 候補 6 行解消。zantufa の gihek は NA? SE? GIhA、
  joik は GAhO? NA? SE? JOI GAhO? で前置スロットを持つ。
  「mi broda na gi'e brode」「mi broda se gi'e brode」「mi broda na joi
  brode」「mi broda na se joi brode」「mi broda na gi'a brode」
  「mi broda se gi'a brode」等が z0/z1/maf 実測 ok。GIhA 形の z0 の木は
  「na」を前の bridi_tail の裸 NA 項(na ku の KU 省略形)として取るが、
  本実装は gihek_link の前置スロットで受ける(木形状差異 = 既知クラス)。
  含めないもの(z0 交叉・実測): NAI 前置(nai joi / nai gi'a)は z0/z1 のみ
  受理・maftufa は拒否の緩受理のため実装しない(既知クラス維持)。
  「se na gi'e」(SE と NA の逆順)は z0 も拒否。「na nai gi'e」は z0 が
  na 項+nai(UI free)+gihek の別読みで受理するが本実装は未対応のまま
  (残差記録)
- 文法⑥(呼格+KOhA 引数の継続): vocative_arg に KOhA_clause を追加し、
  gihek_free_unit に vocative_koha(COI + KOhA 引数必須 + DOhU 省略可)
  を追加。スイープ新記録 GAP 候補 2 行解消。z0 実測: free の第3枝は
  vocative sumti? DOhU_elidible で sumti は代名詞も取れる
  (「mi klama gi'e ju'i do cadzu」「farlu ju'i do cnita」)。
  裸 vocative(引数なし)は連結部で引き続き拒否(「gi'e ju'i cadzu」は
  z0/z1/maf とも拒否)。cmevla 引数形(「gi'e ju'i la alen. cadzu」)は
  z0/z1/maf とも拒否のため連結部では KOhA 引数のみ(既存の拒否と整合)。
  tanru 継続位置(「farlu ju'i la alen. cnita」)は既存 vocative_arg 経路
  (CMEVLA/desc)のまま受理
- スイープ再実行: err→ok 16 行(緑化対象のみ)・**ok→err ゼロ**・
  OVER 増減ゼロ。z0/z1 交叉(バッチ3 全修正プローブ 83 行)で
  受理系は完全一致、拒否系の差分は上記の記録済み残差のみ
- テスト: 統語 198→203(レタル接頭融合語/VUhO 関係節共有/項レベル ke/
  gihek 前置/呼格+sumti の回帰+既存不変ピン)。gap_tracker は残り 4 件
  緑化(全 12 件緑)
- レビュー対応(naku 誤読防止の汎化): 形態論ストレステストで検出した
  CVCV/CVCVV 短形残部の新規 OVER(bynaku / bykuku / bynunu / byzozo /
  bypapa / bydada / cykuku / zykuku / bykukai 型。parent 81a54c2 では
  全形拒否=本変更で新規導入)を !(cvcv_short_tail) 汎化ガードで
  参照 3 種整合の拒否に修正(NAKU_joint の点排除を一般排除に統合。
  詳細は上記「過剰受理チェック」「ガードの範囲」参照)。
  ストレステストの実測プローブ 52 行は
  tests/data/generate_gap_probes.py の構造プローブに取り込み
  (プローブ 1,492→1,544 行)で再現性を確保
- ベンチ: --quick で方向性劣化なし。WASM ビルド ok

## v0.109 で追加(バッチ2 GAP 解消: 接続詞・前置系の再配線)

語彙追加はなし(既存語彙の統語接続の拡張のみ。coverage.md に変更なし)。

- 文法①(裸 tanru BO 接続): tanru_link に BO 単独の選択肢を追加。
  z0 実測: 「mi klama bo cadzu」は selbri_6 = tanru_unit (BO tanru_unit)*
  の tanru 接続(gihek ではない)。tanru 繰り返しの平準形は既知クラスの
  木形状差異(v0.103 と同型)で、受理・読みは同一。
  連鎖「mi klama bo cadzu bo bajra」、描述内「lo broda bo brode ku」、
  「gi'e brode bo brodi」「co cadzu bo bajra」も z0 受理実測で同時緑化。
  「mi na'e bo broda」は z0 も拒否(既存の拒否ピンを維持)
- 文法②(gihek の JOI/BIhI 拡張): gihek_link に gihek_joik を追加
  (GIhA 枝は第1枝に維持し「mi broda gi'e brode」の既存木は不変)。
  z0 実測: gihek は NA? SE? GIhA のみだが、zantufa の joik は総称接続詞
  (JOI+JA系+BIhI)を selbri_4/selbri_5 の selbri 接続で取るため
  「mi broda joi brode」が受理される。本実装は camxes 系 gihek 拡張に倣い
  bridi_tail 連結部で受ける(木形状差異 = 既知クラス: z0 は selbri_4 の
  joik で 2 つの selbri を結ぶが、本実装は gihek_link で 2 つの bridi_tail
  を結ぶ。受理・読みは同一)。
  含めるもの(z0/z1/maf 実測): joi/jo'e/fa'u/ku'a/johu/jo'u/ji、
  NAI 後置(joinai)、BO 後置(joi bo)、SE 変換(se joi / se bi'i)、
  BIhI+GAhO 両端(ga'o bi'i ke'i)+NAI 後置。
  含めないもの(z0 交叉で確認):
  - A 系(a/e/o/u)は z0/z1/maf とも拒否のため含めない(ek_joik との差分)
  - JA 単独の新設枝は置かない。JOI_core の総称構成員として ja/je/jo/ju も
    この位置に到達し得るが、裸形は tanru_link の JA 経路が selbri 解析中に
    先に消費するため gihek には到達しない(「mi broda je brode」の tanru 木
    は不変。je+free 形は z0 も拒否のため gihek_free 側で除外)
  - JOI 枝の GAhO 前置(ga'o joi ke'i)は z0/z1 のみ受理で
    CLL 規範上 GAhO は BIhI の境界指定のため含めない(意図的差分)
  - 前置の否定(nai / na)は含めない。実測差分は前置語で分裂する:
    nai 前置(nai joi / nai gi'a)は z0/z1 のみ受理・maftufa は拒否
    (zantufa の joik 内 NA? スロットと nai を UI free として読む緩さの
    帰結)、na 前置(na joi / na gi'a 等)は z0/z1/maf の 3 種とも受理。
    CLL 規範上 NAI は接続詞の後置のため前置は実装せず、nai 前置は
    z0/z1 のみの緩受理・na 前置は GAP 候補として記録(下記残差節)
  副次効果: 「mi broda ji brode」(v0.108 掃引で検出された残存 GAP 行)も緑化
- 文法③(gihek 直後の vocative 制限): bridi_tail 連結部の free ループを
  gihek_free(vocative 制限版)に変更。「mi klama gi'e ju'i cadzu」は
  z0/z1/maf とも拒否(v0.92 からの過剰受容)に対し、感情標識等
  (.ui / xu / pe'i / ba'e)は z0/z1/maf 受理の実測。vocative は
  DOhU 明示閉鎖形のみ許容(「gi'e ju'i dohu cadzu」は z0/z1/maf 受理)。
  v0.109 で追加する JOI/BIhI 枝にも同じ制限が要るため連結部で一括定義
  (「mi broda joi ju'i brode」「mi broda je ju'i brode」は z0/z1/maf とも拒否)。
  木形状差異(既知クラス): 旧実装の free*(非 silent)を silent の
  gihek_free に置換したため、「mi klama gi'e .ui cadzu」等の既存受容入力の
  解析木から free ラッパが消失する(非 silent の free_unit は
  gihek_free_unit の参照経路で残り、bridi_tail の直接の子に平準化される。
  tail free 側は tail_terms > free > free_unit と free ラッパが残る)。
  受理集合の変化は vocative の制限のみ(実測ピンは
  tests/syntax.rs gihek拡張_既存不変ピン_v0_109)。
  なお gihek_free_unit は free_unit と語彙を単一ソース化
  (`!COI_clause ~ free_unit | vocative_closed` の二択。COI で始まる
  free_unit は vocative のみのためガード分離は受理不変)
- 文法④(JAI+SE/NAhE 変換タグ): tanru_unit の JAI 枝に第3選択肢
  (SE | NAhE)+ ~ tense_mark ~ tanru_unit を追加。
  z0 実測: 「mi jai se gau broda」は tanru_unit_1 = JAI(tag) tanru_unit_1 の
  tag が SE+BAI を取る形(JAI ノード内に [jai][se][gau] と平準化)。
  本実装は SE/NAhE を JAI の兄弟ノードに平準化(既知クラスの木形状差異)。
  変換タグの前置は (SE | NAhE)+ のみ: z0 実測で se gau / na'e gau /
  se na'e gau / na'e se gau / se pu / se ta'i は受理、na gau / ja'a gau /
  se ja'a gau は拒否(z1/maf は na/ja'a も受理する参照分裂だが z0 整合優先)。
  採用側にも参照分裂あり: se pu(「mi jai se pu broda」)と
  se na'e gau(「mi jai se na'e gau broda」)は z0/z1 受理・maftufa は拒否
  (z0 整合優先で受理)。
  第3枝は既存2枝の後に配置: 「mi jai frili」「mi jai gau broda」
  「mi jai se broda」(tanru_unit の SE 前置 brivla)は既存木のまま(z0 も
  SE を tanru_unit_1 側で取る同形)。「jai se gau」(タグだけ)は z0 も拒否
  のため拒否維持
- 文法⑤(free 後の co 転換 selbri 継続): tanru_post に co_post 枝を追加。
  z0 実測: 「farlu ju'i co cnita」は post_clause(free ju'i,
  selbri co cnita, DOhU elided)。tanru 繰り返しの平準形は既知クラスの
  木形状差異。受理スコープ(z0 交叉実測): co 後は tanru と SE 前置
  (co se cnita)まで — s_marks 全体ではない(co ja'a / co na は z0 も拒否)、
  NAhE は tanru_unit の前置で受理(co na'e cnita)。連鎖 co
  (co cnita co brodi / co cnita co se brodi)と連鎖途中の co+SE も受理。
  DOhU 後置不可(farlu ju'i co cnita dohu は z0 も拒否)、ku 後置不可、
  co 後の gek/gi 不可(co broda gi broda は z0 も拒否)。
  free のみで後続が無い入力は従来どおり tail free にフォールバック
  (klama .ui / klama ju'i .ui の既存木は不変)。
  PEG 実装上の注意: 連鎖の2個目以降は区切りの sp1 から始まるため
  繰り返し本体の先頭に sp1 を置く(CO_clause はアトムで空白を消費しないため、
  本体に sp1 を含めないと2個目の co が必ず失敗する。実測で発見)
- 残差の記録(非 GAP・意図的差分): 描述内 selbri の JOI 接続
  「mi viska lo broda joi brode ku」は z0/z1/maf とも受理するが、z0 の読みは
  selbri_4 の joik(selbri 内接続)で、本実装の gihek 経路は描述の sumti_tail
  内には現れないため未対応。構造プローブとして比較表に記録
  (バッチ2 のスコープ外。selbri レベルの接続導入が要る)
- 残差の記録(GAP 候補。いずれも z0/z1/maf ok / ours err。構造プローブで記録:
  【v0.110 で両方とも解消。下記は v0.109 時点の記録】:
  - 呼格+sumti 引数の DOhU 省略形: 「mi klama gi'e ju'i do cadzu」
    「farlu ju'i do cnita」は z0 では vocative が sumti を引数に取れるため
    受理される。本実装の vocative_arg は CMEVLA/desc のみで KOhA を取れず、
    gihek_free 側の vocative_closed は DOhU 必須のため届かない。
    なお cmevla 引数形「mi klama gi'e ju'i la alen. cadzu」は z0/z1/maf とも
    拒否(参照一致)で、残差は KOhA 引数の DOhU 省略形に限られる
    (gap_tracker への RED 化は次バッチ候補として記録のみ)
  - gihek の (NA? SE?) 前置: zantufa の gihek は NA? SE? GIhA、joik は
    GAhO? NA? SE? JOI GAhO? で前置スロットを持つため「mi broda na joi brode」
    「mi broda na se joi brode」「mi broda na gi'e brode」「mi broda
    se gi'e brode」「mi broda na gi'a brode」「mi broda se gi'a brode」の
    6 形が z0/z1/maf とも受理。本実装は gihek_link に前置スロットがなく、
    v0.109 で gihek_joik の JOI/BIhI 枝に SE 前置を実装済みのため
    「se joi」は受理だが GIhA 枝の「se gi'e」は拒否の非対称。
    (NA? SE?) 前置の実装は次バッチ課題(gap_tracker への RED 化は記録のみ)。
    注: 単独形「na joi」「na se joi」は z0 が拒否(z1/maf ok)のため
    プローブは連結部のスロットを直接測る文脈付き形で記録

## v0.108 で追加(バッチ1 GAP 解消: 語彙 ji/va'u と NAI 後置)
- 文法①: JOI_core に ji(接続詞疑問)を追加。zantufa の JOI 終端は接続詞の
  総称で、v0.101 の ja/je/jo/ju と同クラスの語彙追加。
  z0 実測: 「mi ji do klama」は joik_ek(JOI_clause ji)の項接続。
  3 形とも z0/z1/maftufa 受理。mex 演算子位置(li pa ji re 等)も z0 受理
- 文法②: BAI_core に va'u(〜のおかげで)と h 変体 vahu を追加
  (v0.102 BAI 完全化時の漏れだった既知 GAP)。結合形 seva'u/sevahu
  (SEBAI_joint)と単独 2 語形 se va'u の揺れを解消。
  結合形経路は不変(長い語形の SEBAI_joint が優先)、2 語形は
  tagged の SE+BAI 枝で受理
- 文法③: tagged の FA 枝に (sp1 ~ NAI_clause)? を追加(fa nai / fai nai 等。
  BAI 枝の既存形式と同型)。maftufa は FA+NAI を拒否する参照分裂だが
  z0 整合優先で受理。併せて FA 枝の sumti を省略可能に:
  z0 の tag_term は tag + (sumti | KU_elidible) で裸タグ項を受理するため
  (「mi klama fa」/「mi klama fa nai」/裸 fa フラグメントが z0/z1 受理の実測)。
  オプション化のため sumti 付き既存入力の木は不変
- 文法⑤(レビュー対応): タグ契約の KU 半分を実装。FA/BAI 枝の sumti と
  明示 KU を (sumti | KU_clause) の排他選択にし、「fa ku」「fa nai ku」
  「fa fi'a ku」「fa ku do」「se va'u ku」を受理(z0/z1/maf 実測 ok)。
  sumti+ku の二重閉鎖「fa mi ku」「va'u mi ku」は z0/z1/maf とも拒否の
  実測のため排他(拒否ピン)。BAI 枝は sumti 省略化(裸 BAI 項「mi klama
  va'u」。z0/z1/maf 実測 ok)で従来 tense_tags+KU 枝が受けていた
  「va'u ku」「va'u nai ku」を本枝に移動(silent 規則のため木は同形。
  exact-tree ピン)。
  裸 BAI のみ、直後に BO か selbri が続く場合は確定しない否定先読みガード
  を併設(z0 の tag_term ガード !(!tag selbri …) 相当)。term 位置で裸確定
  すると「ni'i bo klama」等の selbri 前置タグが項に食われて文が閉塞する
  PEG 部分成功確定の回避(v0.93 と同型の教訓)
- 文法④: tense_mark(全タグ共通のサイレント規則)に (sp1 ~ NAI_clause)? を後置
  (mi pu nai klama = 過去ではない)。BAI 枝の既存 in-branch 後置は維持
  (bai nai nai は z0 受理実測。PU 単独の二重 nai pu nai nai は z0 ok/
  本実装 err の残差。非掃引対象)。オプション後置のため nai を含まない
  既存入力の木は不変
- 文法⑥(レビュー対応): tense_mark の NAI/BO 後置を
  (sp1 ~ (NAI_clause | BO_clause)) の排他選択に変更。組合せの
  「pu nai bo klama」は z0/z1/maftufa とも拒否する実測のため
  拒否に変更(v0.108 初版は受理していた z0 非整合の新規 OVER を解消。
  「pu nai」自体が err だった HEAD 比較では退行なしの未記録分)。
  逆順「pu bo nai」も z0 一致の拒否。単独「pu bo」は z0 は拒否するが
  意図的拡張(受容優先)として BO 枝を維持
- 読みの実測差異(z0 交叉): zantufa は「nai」をタグ直後の
  post_clause(free(UI_clause nai))として読む(UI 拡張)。
  本実装は NAI_clause のタグ否定(CLL 読み)で受理集合を整合させる。
  木形状は既知の読み差異クラス(tests/syntax.rs に実測コメント記載)
- 意図的残差: 二重 nai は z0/z1 ok・本実装 err(単一後置の限界)。
  PU 直後の「mi ca nai nai klama」に加え、FA/BAI 尾項でも
  「mi klama fa nai nai」「mi klama va'u nai nai」が同様
  (z0/z1 ok / maftufa err の参照割れ。GIhA/UI 系の結合形
  UINAI_joint とは別経路)。未掃引のため GAP に入れていない
- docs/coverage.md は BAI 行(va'u/vahu)・JOI 行(ji)を同期
  (coverage_doc テストが全語彙検証で強制)
- 既存ピンの更新: fai nai の拒否ピン(tests/syntax.rs
  fai_と_faha_fa_a_補完_既存不変と_z0差分ピン)を受理ピンに変更
- テスト: 統語 180→193(ji/va'u/se va'u/fa nai/裸タグ項/時制 nai の
  回帰+既存不変ピン 9 件、レビュー対応で排他化・タグ契約 KU・裸タグ
  追ピン+木不変 exact-tree ピン 4 件)。gap_tracker は 4 件緑化
  (ヘッダコメントに解消済みマーク。assertion は不変)・
  8 件 RED のまま。全体は 267(単体256+doc11、example 内 5 は別途)+gap 12
- スイープ再実行: err→ok 12 行(緑化対象のみ)・**ok→err ゼロ**(前節参照)。
  クラス別 err→ok: FA+NAI 5 行 / ji 3 行 / va'u 3 行 / 時制 nai 1 行。
  レビュー対応(排他化+タグ契約 KU)後の再実行でも既存行の受理集合は不変
  (ok→err ゼロ・新規 OVER ゼロ)
- ベンチ: --quick で方向性劣化なし(環境ノイズ大)。WASM ビルド ok

## v0.107 で追加(gek の項スロット)
- 文法: 接続 gek(ga…gi…)に**項スロット**を追加。①terms_full の連続部選択に
  `gek_tail`(terms 共用。通常文は bridi_tail 枝が先に成功し gek_tail は
  試行されない=ゼロコスト。裸タグ pu/cu を吸う tense_marks? スロット付き)
  ②文レベル第3代替 `gek_head`(項なし形専用)
- 性能上の重要判断: 当初計画の文レベル gek_after_terms(terms? 付き)は、
  入れ子失敗経路(引用 lu…li'u 等)で各階層の terms を二重解析し
  **指数時間化**(fuzz lu ネスト d=7 で 11.8s→112s 実測)。
  terms を1回だけ解析する構造(gek_tail+gek_head)に再設計し
  lu d=8 が 40.9s(v0.106 37.4s)に復帰。
  v0.30/v0.100 の教訓(terms 二重解析禁止)の反映
- 動機: Alice 翻訳の実文 `.i ca lo nu .abu cu za'u re'u mipri catlu kei
  ge la finpe selfu ba'o cliva gi lo drata cu zutse lo loldi ne'a lo vorme
  gi'e bebna catlu fa'a lo tsani` がエラー。z0 は受理
  (camxes `sentence-50 <- terms? gek sentence gik sentence` 相当)
- 効果: 対象文受理。ハーネス(521行)で退行ゼロ・木変化ゼロ、
  受理拡張13行(`ca項+gek`/`mi ga`/`mi cu ge`/`da ge`/`mi pu ge`/
  `mi gu broda gi broda` 等)。z0 交叉 24/25
- 意図的拡張: `mi pu bo ge broda gi broda`(v107 ok/z0 err、
  既存 `mi pu bo klama` と同クラス)
- 木形状差異: gek_tail はサイレントで GA/GI/内側 sentence が
  terms_full 直下に平準化(z0 は gek ノード)。
  ca を tense_marks が先取りする v0.93/94 既知クラス
- テスト: 統語 176→180。全体は 254(単体243+doc11、example 内 5 は別途)
- ベンチ: 交互 A/B 実測で比率 0.94〜1.01(方向性劣化なし)、
  全指標 p>0.05(環境ノイズ大)。WASM 8/8 PASS

## v0.106 で追加(レタル語の tanru 誤吸収修正)
- 文法: tanru_unit の BRIVLA 枝の否定先読みガード群に BY_core を追加。
  レタル語(abu/ebu/ibu/obu/ubu 等)を tanru 単位の brivla としない
  (形態論規範。zantufa-0.9999.js 実測: lo abu ku err)。
  word_boundary 付き BY_core により byklesi 型レタル接頭 lujvo には
  一致しない(非破壊性実測済み)
- 動機: Alice 翻訳の実文「ni'o ze'a lo mentu .abu sanli gi'e catlu lo zdani
  gi'e pensi lo du'u .ei ba zi zukte ma kau .i」がエラー。
  BRIVLA_core が「abu」を brivla 誤マッチし、描述の tanru が
  「mentu abu sanli」を吸収→sentence の bridi_tail が空になり失敗
  (PEG 部分成功確定)
- 効果: 対象文受理。**意図的な受理縮小5件**(ハーネス退行ゼロ規約に対する
  例外的な z0 整合修正): lo abu/ebu/ibu/obu/ubu ku が誤受理から
  z0 整合の拒否に変化。受理拡張2件(対象文+
  ze'a lo mentu .abu sanli gi'e catlu)。木変化3件(lo mentu .abu sanli→
  desc+項+selbri、ze'a lo mentu .abu catlu lo zdani→sentence 読み、
  .abu .ebu .ibu→fragment(BY 項列))、pa abu broda→quant_sumti 読み
- 既知差分: lo byklesi ku(z0 ok/ours err——レタル接頭 lujvo 未対応、
  既存 GAP)/ lo aburobu ku(ours ok/z0 err——母音始まり fu'ivla の
  既存 OVER、本変更と無関係)
- テスト: 統語 175→176。全体は 250(単体239+doc11、example 内 5 は別途)
- ベンチ: 全指標 p>0.05(環境ノイズ大)。WASM ok。z0/z1 交叉一致
  (maftufa 参照は tester が z0/z1 で再検証済み・結果同一)

## v0.105 で追加(fai と fa'a の語彙補完)
- 文法: FA_core に fai(公式 FA 不定格)を追加。fa は fai の接頭辞のため
  fai を先に配置(教訓6 前例)。FAhA_core に fa'a/faha を追加
  (v0.98 FAhA 補完の取りこぼし)。va'a は VUhU の単項演算子(加法逆元)で
  FAhA ではないため未収録(z0 実測で va'a/vaha は拒否)
- 動機: Alice 翻訳の実文「.i .abu mutce gleki lo nu facki lo du'u lo cnebo cu
  jai frili fai lo nu krobi'o fa'a ro da tai tu'a lo since」がエラー。
  z0 は受理(JAI 転換 selbri・BAI+LAhE タグ項は既存対応、fai/fa'a の語彙欠落が原因)
- 効果: 対象文受理。ハーネス(421+449行)で退行ゼロ・受理変動ゼロ・
  木変化ゼロ(861 both-ok 行 sexpr 一致)。z0 交叉 29/29 一致
- 既知差分: FA 枝は NAI を取らないため「fa nai」「fai nai」は
  z0 受理/本実装拒否の既知 GAP(実測・コメント記録済み)
- テスト: 統語 173→175。全体は 249(単体238+doc11、example 内 5 は別途)
- ベンチ: 方向性劣化なし(環境ノイズ大)。WASM ok

## v0.104 で追加(tanru 単位の se/na'e 前置と空項 cu 文)
- 文法①: tanru_unit の BRIVLA 枝に `(NAhE_clause ~ sp1)? ~ (SE_clause ~ sp1)?` の
  オプション前置を追加。tanru の2単位目以降に se/te/ve/xe 変換および
  na'e/to'e/no'e/je'a スケール反転を取れるように。SE/NAhE のみで na/ja'a は含めない
  (z0 実測スコープ: tagji se klama ok / tagji na'e klama ok / tagji na klama err /
  tagji ja'a klama err / tagji se ja'a klama err / tagji se na'e klama err=順序固定)。
  副次効果: tagji na'e se klama と klama se broda(tanru 継続)も受理(z0 ok 実測)
- 文法②: terms_full に第2枝 `| CU_clause ~ sp1 ~ frees_mid? ~ bridi_tail` を追加。
  項が空の cu 文(cu klama / cu pu klama / cu tai klama / cu tagji se danre)を受理
  (camxes terms-80 の空許容相当)。第1枝優先で既存木不変、cu 単独や cu mi は
  拒否(bridi_tail 必須)
- 動機: Alice 翻訳の実文「.i lo .abu xejni'a cu tai tagji se danre lo jamfu ja'e
  lo nu carmi nandu fa lo nu kargau lo moklu」がエラー。z0 の木:
  terms(lo [a bu] xejni'a) + CU + selbri(BAI tai + tanru(tagji, se danre)) +
  tail terms。selbri への BAI 前置は既存対応だったが、tanru 2単位目の se が
  未対応だった
- 効果: 対象文受理。ハーネス(421+449行)で退行ゼロ・受理変動ゼロ・木変化ゼロ。
  z0 交叉 25文 mismatch 0
- テスト: 統語 169→173。全体は 247(単体236+doc11、example 内 5 は別途)
- ベンチ: 方向性劣化なし(環境ノイズ大)。WASM ok

## v0.103 で追加(tanru 単位間の自由修飾語)
- 文法: tanru の繰り返しに第3枝 tanru_post(サイレント規則)を追加。
  tanru 単位間に free 修飾語(vocative/UI 等)を挟めるように。
  後続に単位/link/DOhU が無い free は本枝が失敗して tail free に
  フォールバック(既存の「selbri+末尾 free」の木不変)
- 動機: Alice 翻訳の実文「.i .oi ta ca'o farlu ju'i cnita」がエラー。
  z0 の木は selbri = ca'o + tanru(farlu, [free ju'i], cnita)+DOhU elided
  ——tanru 単位間の free で、呼格引数ではない
- 効果: 対象文受理。ハーネス(421+449行)で退行ゼロ・受理変動ゼロ・
  木変化ゼロ(861 both-ok 行の sexpr 比較)
- z0 整合の拒否を維持: 裸 DOhU(klama dohu)/link 直後の free
  (farlu je ju'i cnita)は z0 も拒否
- 既知差分: 過剰受容2件(farlu .ui dohu / farlu .ui cnita dohu——z0 は err。
  DOhU 枝が free 語種を表明しない設計上の限界、受容側に倒す)+既知 GAP 1件
  (farlu ju'i co cnita——z0 は co 転換 selbri 継続を受理するが本実装未対応。
  将来候補)
- z0 との木形状差異: z0 は後続 selbri を post_clause 内に右再帰ネスト、
  本実装は tanru 繰り返しの平準形(受理・読みは同一)
- テスト: 統語 167→169。全体は 243(単体232+doc11、example 内 5 は別途)
- ベンチ: 方向性劣化なし(環境ノイズ大)。WASM ok

## v0.102 で追加(BAI の語彙完全化)
- 文法: BAI_core を 72語に完全化(base 40語=公式 BAI 36語+実験 ji'e/ji'o/ji'u+ci'u、
  h 変体32語=apostrophe を含む全 base 32語をカバー。うち cihu は legacy)
- 動機: Alice 翻訳の実文「lebna lo tajgai se rai lo ka junri simlu」が
  エラーになった。rai(traji 由来「superlatively」)は公式 BAI だが未収録だった
- 追加語: 公式8語(ba'i/ci'o/rai/di'o/du'i/ga'a/te'i/ca'i)+h 変体26語
  (' ↔ h 規約。ri'i→rihu/ka'a→kahe は z0 実測形を採用)
- z0 差分: タグ付き項位置で se'o/seho の2語のみ z0 が拒否
  (selbri 前タグ位置では z0 も受理=z0 内部の BAI 二系統と整合)。
  seho は規約整合での意図的収録、se'o は pre-existing
- z0 が非 BAI cmavo(caku 等)をタグ位置で緩く受理する点は CLL 規範優先で追わない
- SEBAI_joint に注記追加(結合形リストは pre-existing の部分集合。
  seva'u があるのに va'u が BAI_core 未収録の揺れは意図的に維持)
- テスト: 統語 165→167。全体は 241(単体230+doc11、example 内 5 は別途)
- docs/coverage.md は BAI 行を同期済み。coverage_doc テストが全語彙検証で強制
- ハーネス(421+449行): 退行ゼロ・木差分ゼロ。全72語の z0 タグ位置プローブ実測済み。
  WASM ok。criterion ベンチ方向性劣化なし(環境ノイズ大)

## v0.101 で追加(mex 接続詞演算子と量化 sumti)
- 文法: ①新規サイレント規則 `mex_conn`(A/JOI+NAI | GAhO? BIhI GAhO? NAI。
  BO は演算子に付かない——z0 が「.i se ju bo no da klama」を拒否するのを
  実測で確認)②`mex_operator` に `(SE)? ~ mex_conn` を追加(SE 変換接続詞を
  mex 演算子として受理)③`mex_operand` に前置形を追加 ④`sumti_core` に
  `quant_sumti = { mex ~ sp1 ~ sumti_core }` を追加(camxes の
  quantifier+sumti_5 相当)⑤`JOI_core` に ja/je/jo/ju を追加
- JOI_core 語彙追加の性質: zantufa の JOI 終端は接続詞の総称で JA 系を包含
  (z0 実測: 対象文の ju は JOI_clause、mi ja do klama も joik_ek(JOI ja))。
  CLL の規範では項接続に JA 系は使えないため zantufa 準拠の意図的拡張。
  ji は z0 が受理するが未収録(既知差分 GAP)
- 動機: Alice 翻訳の実文「.i se ju no da mi tolprali lo nu troci」が
  エラーになった。z0 の解析では「se ju」は文接続詞ではなく mex の演算子
  (SE 変換 joik)+被演算子 no(0)で、「se ju no da」は量化 sumti
- 効果: 差分ハーネス(421+449行)で退行ゼロ、受理拡張14行(全て z0 受理を
  実測: se ju/se ja/se bi'i/se ju nai 量化系、li se ju no du re ci、
  li pa se ju re、mi ja/ju/je do klama、lo pa joi re gerku cu barda)
- 木変化クラス: ①数詞+KOhA の2項→quant_sumti 1項(no da klama /
  pa da klama 等13行、z0 整合)②li mex 中置接続の読み変わり
  (li pa joi re / li pa a re が旧2項接続→単一 mex。z0 も単一 mex を
  実測一致)③lo pa joi re gerku cu barda が既知差異の拒否から受容に
  (z0 も受理)
- 既知差分: li ... du ... 形は z0 が全て拒否する既存の受容済み差分クラス
- テスト: 統語 163→165。全体は 239(単体228+doc11、example 内 5 は別途)
- docs/coverage.md は JOI 行を同期。coverage_doc テストを全語彙検証に強化
  (語彙ドリフトを検出できる)
- criterion ベンチ: 方向性劣化なし(環境ノイズ大)。WASM ビルド+Node
  ハーネス 6/6 PASS

## v0.100 で追加(prenex の拡張)
- 文法: ①`prenex_term` の選言末尾に `sumti` を追加(完全な sumti=描述等を
  prenex 項に取れる。PA_seq/PA_clause/KOhA_clause が先に一致するため
  既存木形不変)②`prenex_sentence` の `inner_sentence` を任意化
  (裸 prenex=zo'u で閉じるトピック風の受理)③`item`/`inner_sentence` で
  `sentence` を `prenex_sentence` より先に試行する順序交換(非 prenex 入力の
  二重解析を解消する性能修正。ZOhU を直接消費する規則は prenex_sentence のみ
  のため受理・木形不変。例外: zei_compound/bu_lerfu が word 経由で zo'u を
  吸収し得るが退行なし)
- 動機: Alice 翻訳の実文「ni'o lo di'u preti zo'u」がエラーになった。
  zantufa z0/z1 は受理
  (木: `(NIhO:ni'o [{LE:lo <KOhA:di'u G:preti> KU} ZOhU:zo'u])`)
- 効果: プローブ10行(描述を項に取る prenex、裸 prenex)が z0/z1 と一致。
  868行コーパスで受理変動ゼロ・sexpr 完全一致(順序交換の不変性実証)
- 既知差分: 入れ子の裸 prenex(`ganai mi zo'u gi broda` /
  `mi nu lo preti zo'u kei klama`)は本パーサーが受理するが z0/z1 は拒否
  (意図的拡張として記録)
- 回帰テストを tests/syntax.rs に追加(統語 161→163)。全体は 237
  (単体226+doc11、example 内 5 は別途)
- docs/coverage.md は語彙クラス追加なしのため差分なし
- criterion ベンチ: 順序交換により非 prenex 入力の二重解析を解消。
  8項目すべて p>0.05(環境ノイズ大)
- WASM: ビルド+Node ハーネス 16/16 PASS

## v0.99 で追加(quant_selbri への間隔プロパティガード)
- 文法: quant_selbri に `!(sp1 ~ (ROI|TAhE|ZAhO)_clause)` ガードを追加
  (v0.93 の bare_number ガードと同形式)。数詞の直後が間隔プロパティ語なら
  quantifier+selbri の項としない。zantufa の `!tag sumti` ガード相当
- 効果: 抽象内で「数詞+ROI 複合タグ+selbri」(za'u re'u sudga 等)が
  quant_selbri に貪欲消費されて文が閉塞する問題(v0.93 の
  「ba zi ku le gerku klama」と同型の PEG 部分成功確定)を解消。
  タグ読みは z0 と同構造
- 動機: Alice 翻訳の実文「ni'o lo pa moi preti cu li'a du'u ta'i ma kau
  za'u re'u sudga」がエラーになった
- 木形状変化: number+ROI 系の入力(mi za'u re'u klama 等)で
  fragment(quant_selbri)→sentence(タグ読み)に変化(z0 整合)
- vs-z0 既知差分: number+TAhE/ZAhO の組合せで z0 は ta'e を BAI 扱いする
  非標準分類のため差が発生(各2件)。本パーサーは CLL 準拠の
  interval_property 読みを優先
- 回帰テストを tests/syntax.rs に追加(統語 160→161)。全体は 235
  (単体224+doc11、example 内 5 は別途)
- docs/coverage.md はクラス接続不変のため差分なし
- criterion ベンチは p>0.05 で変化なし
- 差分ハーネス(420行/448行コーパス)で GAP 増減ゼロ・真の新規 OVER ゼロ

## v0.98 で追加(FAhA 残り5語の補完)
- 文法: FAhA_core に標準(CLL 10.12)の残り5語 bu'u / du'a / vu'a / ze'o / zo'i と
  h 表記(buhu/duha/vuha/zeho/zohi)を追加し、FAhA セルマォ16語を完全対応。
  「bu'u lo lalxu(湖と一致する位置に)」等の空間タグ付き項が解析可能に
- 動機: Alice 翻訳の実文「ni'o ca ku .abu tirna lo nu da va jaurjanli
  bu'u lo lalxu」がエラーになった
- h 表記は ze'o→zeho(zeoho ではない。アポストロフィ→h 置換規約。
  zeoho は zantufa も拒否することを実測済み)
- 回帰テストを tests/syntax.rs に追加(統語 159→160)。全体は 234
  (単体223+doc11、example 内 5 は別途)
- docs/coverage.md の FAhA 行を同期
- criterion ベンチは劣化なし
- 差分ハーネス(z0/z1 × 448入力)で GAP 16→0・リグレッション 0・
  真の新規 OVER ゼロ

## v0.97 で追加(描述内の埋め込み sumti)
- 文法: sumti_tail に `!mex ~ sumti ~ sp1 ~ selbri` 枝を追加し、描述内で
  selbri の前に埋め込み sumti を許容。所有形「lo mi gerku(私の犬)」・
  参照形「lo di'u valsi(前述の語)」等が解析可能に。zantufa の
  `sumti_tail <- relative_clauses? (!quantifier sumti)? sumti_tail_1` の
  埋め込みスロット相当
- 数詞描述(lo pa mlatu 等)は既存の mex 経路を維持(mex 枝を先に配置し
  解析木は不変)
- 既知差異(GAP)の記録: 尾部形 quantifier+sumti(lo pa mi gerku /
  lo pa le gerku ku)は zantufa が受理するが本実装は拒否。尾部形の完全対応は
  sumti_tail の2層構造への再編を要するため見送り(テスト・文法コメントで明記)。
  なお lo pa joi re gerku(数詞+JOI 項接続)は v0.101 から mex_conn 経由で
  受容に変わった(「v0.101 で追加」参照)
- 動機: Alice 翻訳の実文「ni'o ca lo nu .abu cusku lo di'u valsi kei
  lo jamfu be .abu cu sakli .i」がエラーになった
- 回帰テストを tests/syntax.rs に追加(統語 157→159)。全体は 233
  (単体222+doc11、example 内 5 は別途)
- docs/coverage.md はクラス接続不変のため差分なし
- criterion ベンチはフルサンプルで改善方向(劣化なし)
- 差分ハーネス(420行コーパス)で GAP 0・真の新規 OVER ゼロを維持

## v0.96 で追加(発話序数 mai の分離形受容)
- 文法: MAI_core に mai/mo'o/moho を追加。従来は融合形(pamai〜nomai)のみで
  selma'o MAI の本体語 mai が欠落していた(mo'o は段落序数)
- mai_free を MAI_clause | number ~ sp1 ~ MAI_clause に拡張し、分離形
  「数詞+mai」(pa mai = 第一に)を自由修飾語として受理。zantufa の
  free <- mex_2 MAI_clause 相当(number までのサブセット)
- 動機: Alice 翻訳の実文「pa mai .abu troci lo nu catlu lo cnita gi'e facki
  lo du'u .abu ma kau klama」がエラーになった
- 意図的緩和の記録(v0.95 タグ+BO の前例準拠): 裸の mai/mo'o 単独も free として
  受理される(zantufa は mex 前置を要求。例: mi klama mai)。テストでピン済み
- 既知差分: 融合形の mo'o 複合(pamo'o 等)は未収録(分離形で受理可能)。
  「paremai」は lujvo(brivla)としての既存受容
- 回帰テストを tests/syntax.rs に追加(統語 156→157)。全体は 231
  (単体220+doc11、example 内 5 は別途)
- docs/coverage.md は MAI 行に mai/mo'o/moho 追加済み
- criterion ベンチはノイズ範囲で変化なし
- 差分ハーネス(420行コーパス)で受容差分ゼロ、GAP=0/OVER 増減ゼロ

## v0.95 で追加(タグ+BO 短スコープ結合の受容)
- 文法: tense_mark に (sp1 ~ BO_clause)? を後置し、タグ+BO の短スコープ結合
  (ni'i bo / ki'u bo / pu bo / mu'i bo 等)を一様に受理。zantufa の
  statement_2 / bridi_tail_2 / sumti_2 / selbri_5 等の `tag? BO_clause` 相当
- 動機: Alice 翻訳の実文「.i ni'i bo lo nunfarlu temci cu mutce banzu …」が
  エラーになった。ki'u bo(〜のため)は実文で高頻度
- 参照版差: 文頭タグ+BO は zantufa-0.9999 が受理・1.9999 は拒否(退行)。
  本パーサーは z0 方針に整合
- 意図的拡張(過剰受容として記録。v0.90 BAhE スコープの記録前例準拠):
  「mi pu bo klama」(selbri 前タグ+BO)は z0/z1 とも拒否だが受容優先で受理。
  宙吊り「pu bo」フラグメント等も副次的に受理(テストでピン済み)
- 既知 GAP の維持: 「mi klama bo cadzu」(裸 tanru BO 接続)は未対応のまま
  (テストでピン)
- 回帰テストを tests/syntax.rs に追加(統語 153→156)。全体は 230
  (単体219+doc11、example 内 5 は別途)
- docs/coverage.md はクラス接続不変のため差分なし
- criterion ベンチは p>0.05 で変化なし

## v0.94 で追加(裸時制連鎖フラグメントの受容)
- 文法: tense_item = { tense_mark } → { tense_marks }。裸時制の発話フラグメントで
  複数タグの連鎖(ku なし)を1項として受理可能に(「mo'i ni'a mo'i ni'a mo'i ni'a」等)。
  zantufa は連鎖を1項として解析するための整合修正
- レビュー対応の追加修正: tense_item に VAU 後置 (sp1 ~ VAU_clause)? を追加
  (zantufa の fragment terms ~ VAU_elidible 相当。「mo'i ni'a vau」等が受理可能に)。
  fragment 末尾に否定先読みガード !(sp1 ~ tense_mark) を追加(「naku pu」「na ku ba」が
  fragment の na_ku 部分確定で tense_item 全体一致に届かない PEG 確定問題の解消)
- いずれも zantufa 照合済みの受容系追加(mo'i ni'a vau / naku pu / na ku ba)。
  差分ハーネス(744文)突合で退行ゼロ・新規過剰受容ゼロを確認
- 動機: Alice 翻訳の実文「ni'o mo'i ni'a mo'i ni'a mo'i ni'a .i xu lo nu farlu cu
  no roi mulno .i」がエラーになった
- 回帰テストを tests/syntax.rs に追加(統語 151→153)。全体は 227
  (単体216+doc11、example 内 5 は別途)
- docs/coverage.md はクラス接続不変のため差分なし
- criterion ベンチは p>0.05 で変化なし
- 差分ハーネス(744文)再検証: v0.94 による新規受理は4文ですべて zantufa も受理、
  リグレッション 0。GAP は既知の意図的非対応(「mi je do klama」系)のみ

## v0.93 で追加(zantufa 差分調査による kau/re'u 追加と時制 ku 項の受容)
- zantufa リファレンスとの差分ハーネス(743文比較)で特定された GAP の修正:
  - 文法語彙: UI_core に kau(間接疑問マーカー。h 表記 kahu は既存で基本形が
    欠落していた drift バグ)/ ROI_core に re'u/rehu(selma'o ROI は
    roi+re'u の2語だったが roi のみだった)
  - 構造: tagged に「タグだけ項」枝(tense_tags ~ sp1 ~ KU_clause)と
    タグ〜sumti 間の自由修飾語許容。「ba zi ku le gerku klama」等の
    [時制+明示ku]+描述+cu無し selbri 形を zantufa 互換の fragment 構造で受理
  - 数詞絡み: bare_number に ROI/TAhE/ZAhO 直前ガード、interval_property を
    分岐形式に書き直し(空白区切り数詞連鎖「za'u ro re'u」等を許容)
- 動機: Alice 翻訳の実文「ni'o ba zi ku la .alis. mo'i ne'i jersi ry gi'e no roi
  pensi lo du'u ta'i ba'e ma kau .abu ba za'u re'u bartu」がエラーになった件。
  差分調査の過程で gerna_cipra の zantufa-1.9999 ビルド自体に
  「za'u + C'V 形 cmavo」で [yY] を要求する退行があることも特定
  (本プロジェクトは camxes/z0 と同じく受理するのが正。参考情報)
- 差分ハーネス(743文)による体系的検証: GAP 17件→1件。残る「mi je do klama」は
  camxes 準拠の意図的非対応。真の新規過剰受容ゼロ
- docs/coverage.md は UI 行に kau、ROI 行に re'u/rehu 同期済み
- 回帰テストを tests/syntax.rs に追加(統語 145→151)。
  criterion ベンチ A/B で性能差なし

## v0.92 で追加(gihek 直後の自由修飾語)
- bridi_tail の連結部で gihek_link(gi'e 等)の直後に自由修飾語(UI 感情標識等)を
  挿入可能に。zantufa の GIhA:gi'e UI:u'a 相当。「mi klama gi'e .u'a cadzu」等が
  解析可能に。`.u'a nai`・`u'a sai`(強度)も既存経路で受理
- 動機: Alice 翻訳の実文「.i .abu bai lo nu kucli cu bajra pagre lo foldi
  gi'e jersi ry gi'e .u'a viska …」(終端スペースなし)が zantufa では通るのに
  本パーサーではエラーだった。切り分けの結果、KE グループ・MOhI+FAhA・
  lujvo spabi'u・noi 等の他要素はすべて既存対応済みで、欠落は gihek 後の
  自由修飾語のみだった
- docs/coverage.md はクラス接続状況不変のため再生成でも差分ゼロ
- 回帰テストを tests/syntax.rs に追加(統語 143→145)。
  criterion ベンチ A/B で性能差なし(関係節ベンチに一時的な変動があったが
  再測定で復帰=ノイズ)
- 解析木: UI なし入力は完全不変。唯一 `gi'e ba'e <selbri>` 形で ba'e の帰属ノードが
  tanru_unit 前置から独立 FREE ノードに(受容可否不変、意味論整合)

## v0.91 で追加(NU に概念抽象 si'o)
- NU_core に概念抽象 si'o(selma'o NU)を追加。標準 NU セルマォ11語の
  最後の欠落分(CLL 準拠補完)。「lo si'o ri viska」等の描述内 si'o 抽象が
  解析可能に
- 動機: Alice 翻訳の長実文「.i ku'i la .alis. ca lo nu … fa lo si'o ri pu no roi viska …」
  (終端スペースあり)が zantufa では通るのに本パーサーではエラーだった。
  切り分けの結果、be 連結項・字詞項・lujvo・pu no roi・poi+.a 接続等の
  文の他要素はすべて既存対応済みで、欠落は si'o のみだった
- docs/coverage.md の NU 行を再生成で同期(si'o 追加。クラス数不変のため
  計数行は変更なし)
- 回帰テストを tests/syntax.rs に追加(統語 141→143)。
  criterion ベンチ A/B で性能差なし。対象文は release で 0.023 秒で解析

## v0.90 で追加(BAhE 前置と LAhE 終端の修正)
- BAhE(ba'e 強調)を tanru 各単位に前置可能に(「na'e ba'e mutce」等)。
  zantufa の [BAhE G:x] 相当。camxes は語単位だが本パーサーは受容優先で
  tanru 単位ごとに受理
- 同 selma'o の za'e(前借り語)も語彙追加
- LAhE(la'e/lu'e)の終端詞を GEhU→LUhU に修正(CLL 6.7/zantufa 準拠)。
  これに伴い:
  - 「la'e di'u lu'u …」のような明示閉鎖が可能に(新規受容)
  - **破壊的変更: 「la'e … ge'u」は拒否されるように**(旧実装の誤り。
    corpus/battery 既存文への影響がないことは確認済み)
- LUhU/LIhU を分離: LUhU_core は本来の lu'u/luhu に、引用終端 li'u/lihu は
  LIhU_clause へ。解析木ノード名が変わるのは lu 引用の閉鎖ノードのみ
  (LUhU_clause→LIhU_clause)
- 動機: Alice 翻訳の実文「ni'o la'e di'u na'e ba'e mutce lo ka cizra」が
  zantufa では通るのに本パーサーではエラーだった
- docs/coverage.md を再生成で同期(BAhE 行に za'e、LIhU 接続化、LUhU 行の
  語形修正。接続クラス数 111→112)
- 回帰テストを tests/syntax.rs に追加(統語 135→141)。
  criterion ベンチ A/B で性能差なし

## v0.89 で追加(BAI に様態タグ tai)
- BAI_core に様態タグ tai(selma'o TAI)を追加(zantufa の BAI:tai 相当)。
  「lo tai senva tcima」等の描述内タグ前置や「mi tai sutra cadzu」等が
  解析可能に
- 動機: Alice 翻訳の実文「.i .oi lo kusru ci mei ni'a lo tai senva tcima cu
  ruble pikci fi lo lisri .i 」(終端スペースあり)が zantufa では通るのに
  本パーサーではエラーだった。切り分けの結果、.i .oi・kusru ci mei(数詞+MOI)・
  ni'a(FAhA タグ項)・末尾 .i は既存対応済みで、欠落は tai 語彙のみだった
- docs/coverage.md の BAI 行を再生成で同期(tai 追加。過去取りこぼしの
  piho pi'o も解消)
- 回帰テストを tests/syntax.rs に追加(統語 129→135)。
  criterion ベンチ A/B で性能差なし

## v0.88 で追加(PA+ROI 複合タグと末尾区切りの受容)
- tense_mark に interval_property(数詞+ROI/TAhE/ZAhO の複合タグ)を配線。
  「so'u roi(まれに)」「re roi(2度)」「pu re roi」等が selbri 前タグとして
  解析可能に(zantufa 準拠)
- 文区切りを sep_conn(接続詞付き .ije/.ibo/.ijanai 等・後続文必須)と
  sep_bare(.i / ni'o 単独・後続任意)に分離。実文で頻出する末尾の単独 .i を
  zantufa どおり受理。宙吊り接続詞(.i je だけ等)は引き続き拒否
- 動機: Alice 翻訳の実文(so'u roi 複合タグと末尾 .i を含む)が zantufa では
  通るのに本パーサーではエラーだった
- 回帰テストを tests/syntax.rs に追加(統語 122→129)。
  criterion ベンチ A/B で性能差なし

## v0.87 で追加(Web Playground)
- examples/web_playground.rs: 追加依存ゼロのローカル HTTP サーバー(std::net のみ)。
  cargo run --example web_playground で http://127.0.0.1:8787 を起動
- 機能: 解析木ビュー・単語分類・JSON・S式タブ、ノード Inspector(規則の日本語説明)、
  エラー位置と期待規則の可視化、入力履歴(localStorage)、共有URL、コピー/ダウンロード、
  Tree 全展開/折りたたみ、キーボード操作(Ctrl/⌘+Enter 解析、Alt+1〜4 タブ切替)、
  パーサー処理時間表示
- Regression Lab: 1行1ケース・最大200件の一括検証(成功率・失敗診断・ケース別処理時間)
- 同梱形態: example 本体に加え site/wasm/(wasm-bindgen 製 WASM クレート lojban-web)。
  site/build-pages.sh が GitHub Pages 用 dist を生成し、.github/workflows/pages.yml が
  自動デプロイ。runtime.js により server/WASM 両モードで同一 UI が動作
- 品質: 全テスト(196+example 5)グリーン。XSS なし(textContent ベースの DOM 構築)、
  読み取りタイムアウト付きの堅牢なローカルサーバー

## v0.86 で追加(クリーンビルド検証と v1.0 ロードマップ)
- cargo clean からの完全再構築で全ゲート(196テスト/clippy/fmt)が緑。
  ビルド成果物に依存しない再現性を確認
- 「v1.0 に向けて」節を新設: 到達度の整理と判定基準(API 凍結、
  定期検証の継続、未収録 selma'o の最終承認、他プラットフォーム確認)

## v0.85 で追加(Tatoeba 定期再検証)
- 未収録 689 文で受理率 94%(647/689)を維持(v0.38 以降 11 回連続同率)。
  v0.84 の pi'o 追加は今回のサンプルに該当文がなく影響なし
  (Tatoeba の単純文では pi'o 使用が稀)。失敗 42 文は既知カテゴリのみ

## v0.84 で追加
- tree::leaf_spans を追加: 葉ノード(規則・原文・バイト位置)の列挙。
  空幅ノード(tail_terms 全省略等)は除外
- BAI に pi'o(using)を追加。空間分離 SE+BAI 枝が BAI 語彙に依存するため、
  pi'o 欠落により se pi'o … が解析できない状態だった
- battery ベンチを criterion に追加(混成実文5文のワークロード計測)。
  ベンチ用連結文の ve'o 不釣り合いは作成ミスとして修正

## v0.83 で追加(Tatoeba 定期再検証)
- 未収録 689 文を --lines パイプラインで検証: 94%(647/689)を維持
  (v0.38 以降 10 回連続同率)。文法変更がないため想定どおり。
  失敗 42 文は既知カテゴリのみ(実験的 cmavo 4件、単独 CAI/時制フラグメント)

## v0.82 で追加(README 数値の再監査)
- テスト数(150→195)とコーパス数(283→418)の陳腐化を修正(日英)。
  版数更新が頻繁なため、数値系はマイルストーン時に再監査する運用を明記

## v0.81 で追加(tests/battery.rs 新設)
- バッテリー掃引(#1〜#8)で検証した実文 59 文を恒久回帰スイートに統合。
  対話・抽象/先接続・引用/入れ子・数理・呼格/談話標識・混成長文の
  7グループで構成し、文法変更時の実用性リグレッションを継続捕捉する

## v0.80 で追加(節目: 安定性・性能の総点検、コード変更なし)
- 重量ファジング(--ignored)を再実行: 289 秒でパニックなし
  (v0.67 BIhE / v0.77 FEhE の文法変更後として確認)
- criterion 再計測: parse 短文 約230µs / 描述+関係節 約688µs /
  複合 約780µs / lujvo 形態論 約598µs / to_json 約748µs。
  ラン間分散があるため断続的な比較観測用だが、いずれも過去最好水準
- camxes.js 比較(v0.80 再計測): 3.6/3.8/2.8 倍。
  過去 4 回の計測で比率 2.8〜4.5 倍の範囲で安定

## v0.79 で追加(語種判定・統計のライブラリ化)
- lojban::classify_word(単語の語種判定)と lojban::word_stats
  (WordStats 構造体による語種別集計)を公開 API 化(doc test 付き)。
  main.rs の --classify/--stats は委譲に切り替わり、
  CLI が薄いラッパーのみの構成になった

## v0.78 で追加(Tatoeba 定期再検証)
- 未収録 704 文で受理率 94%(662/704)を維持(v0.38 以降 9 回連続同率)。
  v0.77 の FEhE 追加による回帰なしを確認
- 検証済み 15 文をコーパスに追加(計 388 → 418 文)

## v0.77 で追加
- FEhE(fe'e 空間間隔プロパティ)を実装: VEhA/VIhA(+FAhA)に続き
  roi/ta'e/zaho の空間用法を連鎖(ve'i fe'e roi …)、単独形も受理
  (camxes space_int_props 準拠)。coverage は 113 クラス / 111 接続。
  「意図的未収録」リストから除外
- 教訓5裏面(先頭 sp1 問題)が空間間隔の単独形でも再発し修正。
  cargo doc の冗長リンク警告2件も解消


## v0.76 で追加(docs/architecture.md 新設)
- 処理パイプライン(前処理3段階 → pest → シリアライザ)、文法の層構造、
  設計上の要点(ZOI 正規化の理由/MAX_NEST の経緯等)、テスト体制を
  解説するコントリビューター向けドキュメントを新設。
  docs/README.md のインデックスにも追加


## v0.75 で追加
- HTML 出力の各ノードに data-start / data-end 属性を追加
  (JSON の位置情報との一貫性。ブラウザ側から原文マッピングが可能)
- comparison.md にバッチモードのスループット参考値を追記
  (412 行 / 約13.5 秒。病理ケース込みの値)


## v0.74 で追加(Tatoeba 定期再検証、--lines のドッグフーディング)
- 未収録 719 文を lojban --lines -f で一括検証: 94%(677/719)を維持
  (v0.38 以降 8 回連続同率)。v0.52 のバッチモードが実務で機能
- 検証済み 15 文をコーパスに追加(計 388 → 403 文)


## v0.73 で追加
- tree::to_json_pretty と CLI --pretty を追加(インデント付き JSON。
  --lines では1行維持のため常にコンパクト)
- 出力形式フラグ(sexpr/json/dot/html)を ArgGroup で排他化。
  組み合わせ時は黙って一方を優先するのではなく明確にエラー


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

## v1.0 に向けて

v0.86 時点での到達度と、v1.0 判定の基準を整理する。

### 到達度
- CLL コア構文は網羅(意図的未収録 11 selma'o を除き、coverage.md 参照)
- Tatoeba 実文受理率 94%×11 回の実測安定性
- テスト 196 件(9 スイート)+ 重量ファジング合格×5 回
- ライブラリ API(parse/friendly_error/classify_word/word_stats/tree×6/lujvo×3)
- CLI(解析5形式・lujvo 3コマンド・classify/stats・バッチモード)
- ドキュメント(README 日英 + docs 6点)

### v1.0 の判定基準と進捗
1. 公開 API の凍結宣言(セマンティックバージョニングへの完全移行) — **未着手**
2. 上記到達度の維持を 3 回以上の定期検証で連続確認 — **11 回連続 94% で達成済み**
3. 意図的未収録 selma'o の最終承認 — **方針文書済み(coverage.md)、承認待ち**
4. クリーンビルド・クロスプラットフォーム確認 — **部分達成(v0.87)**:
   - クリーンビルド: v0.86 で確認済み
   - クロスコンパイルチェック: wasm32-unknown-unknown /
     x86_64-pc-windows-gnu / aarch64-apple-darwin の3ターゲットで
     cargo check 合格(wasm でのブラウザ/Node 利用も視野)
   - MSRV 検証(v0.88): 1.74 ツールチェーンで実測した結果、
     依存 pest 2.9 の実要件は rustc 1.83+ であり、宣言値 1.74 が
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            