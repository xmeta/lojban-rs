# 解析木リファレンス

出力(整形ツリー / S 式 / JSON / DOT / HTML)に現れる主要な pest 規則の意味。
JSON の `rule` フィールドはここに挙げた名前そのもの。

## 階層の骨格

```
text        入力全体(ルート)
└─ content  実内容(先頭の .i 等のリード部を除く)
   └─ item  文・フラグメント・自由修飾語などの単位
      ├─ gek_sentence    先接続文(ganai … gi …)
      ├─ prenex_sentence 前置スコープ文(su'o da zo'u …)
      ├─ sentence        通常の文
      ├─ fragment        項のみの断片(mi 単独 等)
      └─ free            語・句の単独自由修飾(.ui 等)
```

## 文の内部

| 規則 | 意味 |
|---|---|
| `terms_full` | 主語等の項リスト + (`cu`) + 述語 |
| `terms` / `term` | 項とその並び。項間には自由修飾語・`ce'e` 区切り・`pe'e` グループ接続が現れ得る |
| `tagged` | タグ付き項(FA / BAI(+SE/NAI) / FIhO モダル / 時制マーク連鎖+sumti) |
| `na_ku` | 項位置の否定(`naku` / `na ku`) |
| `termset` | 項set(`nu'i X Y [nu'u]`) |
| `bridi_tail` | 述語とその項(tail)。`gihek`(gi'e 等)による連鎖を含む |
| `tail_terms` | 述語に続く項の列(自由修飾語混在可)+ `vau` |

## sumti(項)

`sumti` の中核は `sumti_core`(silent)で、次のいずれかが現れる:

| 規則 | 意味 |
|---|---|
| `KOhA_clause` | 代名詞(mi do ri ke'a di'u …) |
| `desc` | 冠詞句(le / lo / la / lo'e … + 述語) |
| `quant_desc` / `quant_selbri` | 数量詞+描述 / 数量詞+述語(pa prenu) |
| `bare_number` | 裸の数詞(直後が MOI の場合は項にならない) |
| `abstraction` | 抽象(nu / ka / du'u … + 文 + kei?)。`sedu'u` 結合形含む |
| `lahe_sumti` | LAhE 参照(la'e X / lu'e X、終端詞 lu'u で明示閉鎖可)と結合形 `la'edi'u` |
| `lu_quote` / `zo_quote` / `zoi_quote` / `lohu_quote` | 各種引用 |
| `li_mex` | 数理表現(li … loho) |
| `gek_sumti` | 先接続項(ge X gi Y) |
| `bu_lerfu` | 文字化(任意の語 + bu) |

`sumti` はさらに関係節(`relative_clauses`: poi/noi + `ke'a` + `ku'o`)、
項接続(`ek_joik`: e/joi/bi'o… + nai + bo)、`vu'o` 連結を後置できる。

## selbri(述語)

| 規則 | 意味 |
|---|---|
| `s_marks` | 述語マーク(na 否定 / ja'a 肯定 / se te ve xe 変換 / na'e to'e no'e je'a) |
| `tense_marks` | 時制・相・方位・モダルの連鎖(pu ca ba / ze'i ze'a ze'u / ROI / BAI / FAhA / MOhI / ZEhA / VEhA VIhA / cu'e / ki / naku / ZEhA・空間間隔 / 数詞+ROI/TAhE/ZAhO の複合タグ(so'u roi 等)) |
| `tanru` / `tanru_unit` | 複合述語。unit は brivla / GOhA / cmevla / nu_form / me_form(+me'u, MOI) / ke_group / JAI 変換 / 数詞+MOI / zei 複合 を取り得る(各 unit の前に BAhE(ba'e/za'e)強調を前置可) |
| `co_tail` | `co` による逆順 |
| `guhek_selbri` | 先接続述語(gu'e … gi) |

## 自由修飾語(free_unit)

感情標識(`ui` + CAI 強度 + nai、結合形 `ta'onai` 等)、談話標識
(`ku'i ja'o po'o …`)、`sei` 挿入(内部に文)、`to … toi` 注釈、
呼格(`coi … [do'u]`)、`soi` 入れ替え、発話序数(`pamai`)、添字(`xi`)、
`da'o` / `fa'o` / `fu'e` `fu'o` スコープ、BAhE 強調、SI/SU。

## 解析木が元テキストと異なるケース

- **ZOI 引用**: 本文は `zo'e` に正規化される(pest に後方参照がないため)
- **SI/SU 消去**: 解析前に適用され、木は消去後のテキストに基づく
- 上記以外は木の `text` を連結すると入力と一致する
