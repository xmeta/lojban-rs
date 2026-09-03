# 実装済み cmavo クラス一覧

`lojban.pest` から抽出した語彙クラスと統語接続の状況(文法定義順)。
「統語接続」= 対応する `*_clause` が文法規則から参照されている(項・文として届く)。
同期は `tests/coverage_doc.rs` が検証する。

| selma'o | 語彙 | 統語接続 |
|---|---|---|
| A |  | ✅ |
| BAI | `bau` `bai` `cau` `cu'u` `mu'i` `mu'u` `ni'i` `ri'a` `ta'i` `tai` `ki'u` `kai` `pa'u` `se'o` `si'u` `va'o` `zu'e` `ja'e` `ra'i` `ri'i` `de'i` `du'o` `fau` `ka'a` `gau` `piho` `pi'o` `cihu` `ci'u` `pu'a` `jihe` `ji'e` `jihu` `ji'u` `jiho` `ji'o` `raha` `ra'a` `ba'i` `ci'o` `rai` `bahi` `cuhu` `duho` `muhi` `nihi` `riha` `seho` `vaho` `rahi` `tahi` `ciho` `di'o` `du'i` `ga'a` `te'i` `muhu` `kihu` `sihu` `rihu` `dehi` `diho` `duhi` `gaha` `tehi` `ca'i` `cahi` `jahe` `pahu` `kahe` `puha` `zuhe` `va'u` `vahu` | ✅ |
| BE | `be` | ✅ |
| BEI | `bei` | ✅ |
| BEhO | `be'o` `beho` | ✅ |
| BO | `bo` | ✅ |
| BOI | `boi` | ✅ |
| CO | `co` | ✅ |
| COI | `coi` `co'o` `je'e` `ju'i` `re'i` `nu'e` `fi'i` `fe'o` `mu'o` `mi'e` `ta'a` `pe'u` `ki'e` `viho` `vi'o` `keho` `ke'o` `doi` | ✅ |
| CU | `cu` | ✅ |
| DOhU | `do'u` `dohu` | ✅ |
| FA | `fi'a` `fai` `fa` `fe` `fi` `fo` `fu` | ✅ |
| FAhA | `ca'u` `cahu` `ti'a` `tiha` `zu'a` `zuha` `ga'u` `gahu` `ni'a` `niha` `ru'u` `ruhu` `ne'i` `pa'o` `paho` `te'e` `ne'a` `re'o` `bu'u` `buhu` `du'a` `duha` `vu'a` `vuha` `ze'o` `zeho` `zo'i` `zohi` `fa'a` `faha` | ✅ |
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
| FIhU | `fihu` `fi'u` | ✅ |
| BIhE | `bihe` `bi'e` | ✅ |
| FEhE | `fehe` `fe'e` | ✅ |
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
| JOI | `jo'e` `joi` `fa'u` `ku'a` `johu` `jo'u` `ja` `je` `jo` `ju` `ji` | ✅ |
| KE | `ke` | ✅ |
| KEhE | `ke'e` `kehe` | ✅ |
| KEI | `kei` | ✅ |
| KOhA | `zo'e` `zu'i` `ke'a` `mi'o` `ko'a` `ko'e` `ko'i` `ko'o` `ko'u` `fo'a` `fo'e` `fo'i` `fo'o` `fo'u` `vo'a` `vo'e` `vo'i` `vo'o` `vo'u` `mi'ai` `miahi` `mi'a` `ma'a` `do'o` `tu'a` `dei` `di'u` `dihu` `de'u` `dehu` `da'u` `dahu` `mi` `do` `ti` `ta` `tu` `ri` `ra` `ru` `ko` `ma` `da` `de` `di` `ce'u` `cehu` `zi'o` `ziho` | ✅ |
| KU | `ku` | ✅ |
| KUhO | `ku'o` `kuho` | ✅ |
| LAhE | `la'e` `lu'e` | ✅ |
| LE | `le'i` `lo'i` `la'i` `lei` `loi` `lai` `lo'e` `lehe` `le'e` `le` `lo` `la` | ✅ |
| LEhU | `le'u` `lehu` | ✅ |
| LI | `li` | ✅ |
| LIhU | `li'u` `lihu` | ✅ |
| LOhU | `lo'u` `lohu` | ✅ |
| LU | `lu` | ✅ |
| LUhU | `lu'u` `luhu` | ✅ |
| NA | `na` | ✅ |
| JAhA | `ja'a` `jaha` | ✅ |
| NAhE | `na'e` `to'e` `no'e` `je'a` | ✅ |
| NIhO | `ni'o` `niho` | ✅ |
| NOI | `poi` `noi` `voi` | ✅ |
| NU | `je'i` `mu'e` `pu'u` `zu'o` `li'i` `su'u` `si'o` `du'u` `nu` `ka` `ni` | ✅ |
| PA |  | ✅ |
| ROI | `roi` `re'u` `rehu` | ✅ |
| SE | `se` `te` `ve` `xe` | ✅ |
| SEI | `sei` | ✅ |
| SEhU | `se'u` `sehu` | ✅ |
| TO | `to` | ✅ |
| TOI | `toi` | ✅ |
| UI | `u'i` `u'u` `ru'e` `ju'o` `pe'i` `sa'e` `ta'o` `e'o` `e'e` `ehu` `e'u` `ohu` `o'u` `a'e` `i'a` `buho` `bu'o` `kuhi` `ku'i` `ja'o` `po'o` `dahi` `da'i` `jehu` `je'u` `laha` `la'a` `zaha` `za'a` `gahi` `ga'i` `uho` `u'o` `ihi` `i'i` `oha` `o'a` `ehi` `e'i` `kahu` `ka'u` `kau` `ruha` `ru'a` `jiha` `ji'a` `zuhu` `zu'u` `baha` `ba'a` `ai` `au` `kiaha` `ki'a` `zo'o` `a'u` `o'o` `u'a` `u'e` `cai` `ui` `oi` `ie` `ii` `uu` `ua` `ue` `uo` `ia` `iu` `ei` `xu` `i'e` `be'e` `be'u` `di'ai` `fau'u` `ge'e` `li'a` `ni'au` `pei` `o'i` `su'a` `a'i` `ahi` `a'o` `aho` `ca'e` `cahe` `dai` `e'a` `eha` `io` `ju'a` `juha` `ke'u` `kehu` `le'o` `leho` `li'o` `liho` `o'e` `ohe` `pau` `pa'e` `pahe` `ra'u` `rahu` `ro'a` `roha` `ro'o` `roho` `se'a` `seha` `si'a` `siha` `ta'u` `tahu` `ti'e` `tihe` `to'u` `tohu` `va'i` `vahi` `vu'e` `vuhe` | ✅ |
| VAU | `vau` | ✅ |
| VUhO | `vu'o` `vuho` | ✅ |
| Y |  | ✅ |
| NAI | `nai` | ✅ |
| MAI | `pamai` `remai` `cimai` `vomai` `mumai` `xamai` `zemai` `bimai` `somai` `nomai` `mai` `mo'o` `moho` | ✅ |
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
| BAhE | `bahe` `ba'e` `za'e` | ✅ |
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

計 113 クラス定義 / 112 クラスが統語に接続。
未接続の GIhI は対応する統語(文連結 gihi)が未実装のための予備定義。
結合表記(joint)は次セクション。
注: UI の `ca'e` は CLL 分類では CAhE(独立 selma'o)だが、参照 3 種(zantufa
z0/z1・maftufa)が UI と同一挙動(自由修飾語位置・tanru 単位位置とも受理)で
あるため UI_core に収録している(v0.111)。
