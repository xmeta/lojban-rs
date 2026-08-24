# 実装済み cmavo クラス一覧

`lojban.pest` から抽出した語彙クラスと統語接続の状況(文法定義順)。
「統語接続」= 対応する `*_clause` が文法規則から参照されている(項・文として届く)。
同期は `tests/coverage_doc.rs` が検証する。

| selma'o | 語彙 | 統語接続 |
|---|---|---|
| A |  | ✅ |
| BAI | `bau` `bai` `cau` `cu'u` `mu'i` `mu'u` `ni'i` `ri'a` `ta'i` `ki'u` `kai` `pa'u` `se'o` `si'u` `va'o` `zu'e` `ja'e` `ra'i` `ri'i` `de'i` `du'o` `fau` `ka'a` `gau` `cihu` `ci'u` `pu'a` `jihe` `ji'e` `jihu` `ji'u` `jiho` `ji'o` `raha` `ra'a` | ✅ |
| BE | `be` | ✅ |
| BEI | `bei` | ✅ |
| BEhO | `be'o` `beho` | ✅ |
| BO | `bo` | ✅ |
| BOI | `boi` | ✅ |
| CO | `co` | ✅ |
| COI | `coi` `co'o` `je'e` `ju'i` `re'i` `nu'e` `fi'i` `fe'o` `mu'o` `mi'e` `ta'a` `pe'u` `ki'e` `viho` `vi'o` `keho` `ke'o` `doi` | ✅ |
| CU | `cu` | ✅ |
| DOhU | `do'u` `dohu` | ✅ |
| FA | `fi'a` `fa` `fe` `fi` `fo` `fu` | ✅ |
| FAhA | `ca'u` `cahu` `ti'a` `tiha` `zu'a` `zuha` `ga'u` `gahu` `ni'a` `niha` `ru'u` `ruhu` `ne'i` `pa'o` `paho` `te'e` `ne'a` `re'o` | ✅ |
| MOhI | `mo'i` `mohi` | ✅ |
| FAhO | `fa'o` `faho` | ✅ |
| FEhU | `fe'u` `fehu` | ✅ |
| FIhO | `fi'o` `fiho` | ✅ |
| GA | `ga` `ge` `go` `gu` | ✅ |
| GAhO | `ke'i` `kehi` `ga'o` `gaho` | ✅ |
| GEhU | `ge'u` `gehu` | ✅ |
| GI | `gi` | ✅ |
| GIhA | `gi'a` `gi'e` `gi'i` `gi'o` `gi'u` | ✅ |
| GIhI | `gihi` | — |
| GOI | `po'e` `po'u` `no'u` `goi` `pe` `po` `ne` | ✅ |
| GOhA | `go'i` `go'o` `nei` `ra'o` `du` `mo` | ✅ |
| I | `i` | ✅ |
| BIhI | `bi'o` `bi'i` `mi'i` | ✅ |
| VUhU | `su'i` `vu'u` `pi'i` `fe'i` `gei` `de'o` `te'o` `re'a` `va'a` `pa'i` `si'i` `fu'u` | ✅ |
| NAhU | `na'u` `nahu` | ✅ |
| PEhO | `peho` `pe'o` | ✅ |
| MAhO | `maho` `ma'o` | ✅ |
| KUhE | `kuhe` `ku'e` | ✅ |
| TEhU | `tehu` `te'u` | ✅ |
| MOhE | `mo'e` `mohe` | ✅ |
| VEI | `vei` | ✅ |
| VEhO | `ve'o` `veho` | ✅ |
| LOhO | `lo'o` `loho` | ✅ |
| BY | `by` `cy` `dy` `fy` `gy` `jy` `ky` `ly` `my` `ny` `py` `ry` `sy` `ty` `vy` `xy` `zy` `abu` `ebu` `ibu` `obu` `ubu` `ybu` | ✅ |
| BU | `bu` | ✅ |
| GUhA | `gu'a` `gu'e` `gu'o` `gu'u` | ✅ |
| SI | `si` | ✅ |
| SU | `su` | ✅ |
| JA | `ja` `je` `jo` `ju` | ✅ |
| JOI | `jo'e` `joi` `fa'u` `ku'a` `johu` `jo'u` | ✅ |
| KE | `ke` | ✅ |
| KEhE | `ke'e` `kehe` | ✅ |
| KEI | `kei` | ✅ |
| KOhA | `zo'e` `zu'i` `ke'a` `mi'o` `ko'a` `ko'e` `ko'i` `ko'o` `ko'u` `fo'a` `fo'e` `fo'i` `fo'o` `fo'u` `vo'a` `vo'e` `vo'i` `vo'o` `vo'u` `mi'ai` `miahi` `mi'a` `ma'a` `do'o` `tu'a` `dei` `di'u` `dihu` `de'u` `dehu` `da'u` `dahu` `mi` `do` `ti` `ta` `tu` `ri` `ra` `ru` `ko` `ma` `da` `de` `di` | ✅ |
| KU | `ku` | ✅ |
| KUhO | `ku'o` `kuho` | ✅ |
| LAhE | `la'e` `lu'e` | ✅ |
| LE | `le'i` `lo'i` `la'i` `lei` `loi` `lai` `lo'e` `lehe` `le'e` `le` `lo` `la` | ✅ |
| LEhU | `le'u` `lehu` | ✅ |
| LI | `li` | ✅ |
| LIhU | `li'u` `lihu` | — |
| LOhU | `lo'u` `lohu` | ✅ |
| LU | `lu` | ✅ |
| LUhU | `li'u` `lihu` | ✅ |
| NA | `na` | ✅ |
| JAhA | `ja'a` `jaha` | ✅ |
| NAhE | `na'e` `to'e` `no'e` `je'a` | ✅ |
| NIhO | `ni'o` `niho` | ✅ |
| NOI | `poi` `noi` `voi` | ✅ |
| NU | `je'i` `mu'e` `pu'u` `zu'o` `li'i` `su'u` `du'u` `nu` `ka` `ni` | ✅ |
| PA |  | ✅ |
| ROI | `roi` | ✅ |
| SE | `se` `te` `ve` `xe` | ✅ |
| SEI | `sei` | ✅ |
| SEhU | `se'u` `sehu` | ✅ |
| TO | `to` | ✅ |
| TOI | `toi` | ✅ |
| UI | `u'i` `u'u` `ru'e` `ju'o` `pe'i` `sa'e` `ta'o` `e'o` `e'e` `ehu` `e'u` `ohu` `o'u` `a'e` `i'a` `buho` `bu'o` `kuhi` `ku'i` `ja'o` `po'o` `dahi` `da'i` `jehu` `je'u` `laha` `la'a` `zaha` `za'a` `gahi` `ga'i` `uho` `u'o` `ihi` `i'i` `oha` `o'a` `ehi` `e'i` `kahu` `ka'u` `ruha` `ru'a` `jiha` `ji'a` `zuhu` `zu'u` `baha` `ba'a` `ai` `au` `kiaha` `ki'a` `zo'o` `a'u` `o'o` `u'a` `u'e` `cai` `ui` `oi` `ie` `ii` `uu` `ua` `ue` `uo` `ia` `iu` `ei` `xu` `i'e` `be'e` `be'u` `di'ai` `fau'u` `ge'e` `li'a` `ni'au` `pei` `o'i` `su'a` | ✅ |
| VAU | `vau` | ✅ |
| VUhO | `vu'o` `vuho` | ✅ |
| Y |  | ✅ |
| NAI | `nai` | ✅ |
| MAI | `pamai` `remai` `cimai` `vomai` `mumai` `xamai` `zemai` `bimai` `somai` `nomai` | ✅ |
| CAI | `cai` `sai` `ru'e` `ruhe` `cu'i` `cuhi` | ✅ |
| PU | `pu` `ca` `ba` | ✅ |
| KI | `ki` | ✅ |
| CAhA | `ka'e` `ca'a` `nu'a` `ja'ai` | ✅ |
| CUhE | `cu'e` `cuhe` | ✅ |
| ZOhU | `zohu` `zo'u` | ✅ |
| ZAhO | `pu'o` `co'a` `za'o` `ca'o` `co'u` `mo'u` `ba'o` `de'a` `di'a` | ✅ |
| ZI | `ze'i` `zehi` `ze'a` `zeha` `ze'u` `zehu` `zi` `za` `zu` | ✅ |
| ZEhA | `zi'i` `zihi` `bi'o` `biho` `bi'i` `bihi` `mi'i` `mihi` | ✅ |
| VEhA | `ve'i` `vehi` `ve'a` `veha` `ve'e` `vehe` `ve'u` `vehu` | ✅ |
| VIhA | `vi'i` `vihi` `vi'a` `viha` `vi'u` `vihu` `vi'e` `vihe` | ✅ |
| VA | `vi` `va` `vu` | ✅ |
| TAhE | `ta'e` `di'i` `na'o` `ru'i` | ✅ |
| ME | `me` | ✅ |
| JAI | `jai` | ✅ |
| MOI | `mei` `moi` `sihe` `si'e` `cuho` `cu'o` `vahe` `va'e` | ✅ |
| MEhU | `mehu` `me'u` | ✅ |
| ZEI | `zei` | ✅ |
| SOI | `soi` | ✅ |
| XI | `xi` | ✅ |
| CEhE | `cehe` `ce'e` | ✅ |
| PEhE | `pehe` `pe'e` | ✅ |
| BAhE | `bahe` `ba'e` | ✅ |
| DAhO | `daho` `da'o` | ✅ |
| NUhI | `nuhi` `nu'i` | ✅ |
| NUhU | `nuhu` `nu'u` | ✅ |
| FUhE | `fuhe` `fu'e` | ✅ |
| FUhO | `fuho` `fu'o` | ✅ |
| ZO | `zo` | ✅ |
| ZOI | `zoi` | ✅ |
| CMAVO |  | ✅ |
| CMEVLA |  | ✅ |
| BRIVLA |  | ✅ |

計 110 クラス定義 / 108 クラスが統語に接続。
未接続の LIhU は LUhU と同語形のための予備定義(設計上未使用)。
結合表記(joint)は次セクション。

## 意図的に未収録の selma'o(camxes 対比)

camxes の語彙リストと照合し、本パーサーが意図的に収録していないクラス。
実験的・周辺的機能であり、必要になった時点で `*_core` 追加で対応できる。

| selma'o | 機能 | 非収録の理由 |
|---|---|---|
| BIhE | mex のべき乗接続 | 数理の詳細演算。利用例が稀 |
| CEI | 文字・数字の変換 | 周辺機能 |
| FEhE | 空間間隔の方向指定 | 空間時制の詳細 |
| FOI | 数字列の終端 | BOI で代替可能 |
| FUhA | mex の逆順マーク | 数理の詳細 |
| JOhI | ベクトル演算 | 実験的 |
| LAU / TEI / TUhU | 複合文字・テキスト範囲 | 周辺機能 |
| NUhA | mex 演算子への変換 | NAhU で大部分を代替 |
| RAhO | 関係節スコープ | 意味論が複雑 |
| SA | 遡及修正 | 直前一致 selma'o 探索が必要(README 参照) |
| ZIhE | なぜ疑問 | 周辺機能 |
