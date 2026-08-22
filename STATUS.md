# 開発ステータス

## 現在の状態: v0.4 完成(全テストグリーン)

- ライブラリ5 / 形態論11 / 統語37 / コーパス3 = 計56テスト全パス

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

## 次の拡張候補
- 接続詞の bo グルーピング・BIhI 間隔接続、mex 数理、BAI タグ接続、ZOI・入れ子引用
