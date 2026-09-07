# 実装済み cmavo クラス一覧

`lojban.pest` から抽出した語彙クラスと統語接続の状況(文法定義順)。
「統語接続」= 対応する `*_clause` が文法規則から参照されている(項・文として届く)。
同期は `tests/coverage_doc.rs` が検証する。

| selma'o | 語彙 | 統語接続 |
|---|---|---|
| A |  | ✅ |
| BAI | `bau` `bai` `cau` `cu'u` `mu'i` `mu'u` `ni'i` `ri'a` `ta'i` `tai` `ki'u` `kai` `pa'u` `se'o` `si'u` `va'o` `zu'e` `ja'e` `ra'i` `ri'i` `de'i` `du'o` `fau` `ka'a` `gau` `piho` `pi'o` `cihu` `ci'u` `pu'a` `jihe` `ji'e` `jihu` `ji'u` `jiho` `ji'o` `raha` `ra'a` `ba'i` `ci'o` `rai` `bahi` `cuhu` `duho` `muhi` `nihi` `riha` `seho` `vaho` `rahi` `tahi` `ciho` `di'o` `du'i` `ga'a` `te'i` `muhu` `kihu` `sihu` `rihu` `dehi` `diho` `duhi` `gaha` `tehi` `ca'i` `cahi` `jahe` `pahu` `kahe` `puha` `zuhe` `va'u` `vahu` `pu'i` `xa'o` `dei'a` `co'i` `co'u` | ✅ |
| BE | `be` | ✅ |
| BEI | `bei` | ✅ |
| BEhO | `be'o` `beho` | ✅ |
| BO | `bo` | ✅ |
| BOI | `boi` | ✅ |
| CO | `co` | ✅ |
| COI | `coi` `co'o` `je'e` `ju'i` `re'i` `nu'e` `fi'i` `fe'o` `mu'o` `mi'e` `ta'a` `pe'u` `ki'e` `viho` `vi'o` `keho` `ke'o` `doi` `ki'ai` `di'ai` | ✅ |
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
| GOhA | `go'i` `go'o` `nei` `ra'o` `du` `mo` `bu'a` `bu'e` `bu'i` `cei'i` `co'e` `gai'o` `go'a` `go'e` `go'u` `no'a` `xe'u` | ✅ |
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
| KOhA | `zo'e` `zu'i'a` `zu'i` `ke'a` `mi'oi` `mi'o` `ko'a` `ko'e` `ko'i` `ko'o` `ko'u` `fo'a` `fo'e` `fo'i` `fo'o` `fo'u` `vo'a` `vo'e` `vo'i` `vo'o` `vo'u` `mi'ai` `miahi` `mi'a` `ma'a` `do'o` `dei'ei` `dei'e` `dei'o` `dei'u` `dei` `di'u` `dihu` `de'u` `dehu` `da'u` `dahu` `mi` `do'ei` `do'i` `do` `ti` `ta` `tu'oi` `tu` `ri'au` `ri` `ra` `ru` `ko` `mai'i` `ma` `da'ai` `da'au` `da'e` `da` `de'e` `de` `di'au` `di'ei` `di'e` `di'oi` `di` `ce'u` `cehu` `zi'oi` `zi'o` `ziho` `ca'au` `kau'a` `kau'e` `kau'i` `lau'e` `lau'u` `nau'u` `nei'o` `zai'o` `zu'ai` `bo'a` `bo'e` `bo'i` `bo'o` `bo'u` `xai` | ✅ |
| KU | `ku` | ✅ |
| KUhO | `ku'o` `kuho` | ✅ |
| LAhE | `tu'a` `tuha` `la'e'au` `la'e` `lu'e` `lu'au` `lu'a` `du'au` `lai'e` `tau'e` `zo'ei` `lu'i` `lu'o` `vu'i` | ✅ |
| LE | `le'i` `lo'i` `la'i` `lei'e` `lei'i` `lei` `loi'e` `loi'i` `loi` `lai` `lo'ei` `lo'e` `lehe` `le'ei` `le'e` `le` `lo` `la'ei` `la` `moi'oi` `me'ei` `mo'oi` `ri'oi` `zo'au` | ✅ |
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
| NU | `je'i` `mu'e` `pu'u` `zu'o` `li'i` `su'u` `si'o` `du'u` `nu` `kai'ei` `kai'u` `ka'ei` `ka` `ni'ai` `ni` `bu'ai` `poi'i` `za'i` `jei` | ✅ |
| PA | `dau` `fei` `rei` `vai` `so'a` `so'e` `so'i` `so'u` `so'o` `soho` `za'u` `su'o` `suho` `suhe` `su'e` `daha` `da'a` `ji'i` `jihi` `xo` `pi` `rau` `du'e` `duhe` `mo'a` `moha` `te'o` `teho` `ka'o` `kaho` `tu'o` `pai` `pa` `re` `ci'i` `ci` `vo` `mu` `xa` `ze` `bi` `so` `no` `ro` `ki'o` `ma'u` `ni'u` `ce'i` `cehi` `fi'u` `fihu` | ✅ |
| ROI | `roi` `re'u` `rehu` | ✅ |
| SE | `se'o'e` `se'u'o` `se` `te` `ve` `xe` `re'au'e` `su'ei` `tau'o` `to'ai` `vo'ai` `xo'ai` | ✅ |
| SEI | `sei` | ✅ |
| SEhU | `se'u` `sehu` | ✅ |
| TO | `to` | ✅ |
| TOI | `toi` | ✅ |
| UI | `u'i` `u'u` `ru'e` `ju'oi` `ju'o` `pe'i` `sa'e` `ta'oi` `ta'o` `e'o` `e'e` `ehu` `e'u` `ohu` `o'u` `a'e` `i'a` `buho` `bu'o` `kuhi` `ku'i` `ja'o` `po'o` `dahi` `da'i` `jehu` `je'u` `laha` `la'a` `zaha` `za'a` `gahi` `ga'i` `uho` `u'o` `ihi` `i'i` `oha` `o'a` `ehi` `e'i` `kahu` `ka'u` `kau` `ruha` `ru'a` `jiha` `ji'au` `ji'a` `zuhu` `zu'u` `baha` `ba'a` `ai` `au'u` `au` `kiaha` `ki'a'au'u'au'i` `ki'a` `zo'o` `a'u` `o'o` `u'ai` `u'a` `u'e` `cai` `ui` `oi'a` `oi'o` `oi'u` `oi` `ie'i` `ie` `ii` `uu` `uai` `uau` `ua` `ue'i` `uei'e` `ue` `uo` `ia'u` `ia` `iu` `ei` `xu'u'i` `xu` `i'e` `be'e` `be'u` `di'ai` `fau'u` `ge'ei` `ge'e` `li'a` `ni'au` `pei'a` `pei'e` `pei'o` `pei` `o'i` `su'a` `a'i` `ahi` `a'o` `aho` `ca'e` `cahe` `dai'i` `dai'o` `dai` `e'a` `eha` `io` `ju'a` `juha` `ke'u` `kehu` `le'o` `leho` `li'oi` `li'o` `liho` `o'e` `ohe` `pau` `pa'e` `pahe` `ra'u` `rahu` `ro'a` `roha` `ro'o` `roho` `se'a` `seha` `si'au` `si'a` `siha` `ta'u` `tahu` `ti'e` `tihe` `to'u` `tohu` `va'i` `vahi` `vu'e` `vuhe` `sahe` `ohi` `taho` `pehi` `juho` `uhi` `uhu` `ruhe` `eho` `ehe` `ahe` `iha` `zoho` `ahu` `oho` `uha` `uhe` `ihe` `behe` `behu` `dihai` `fauhu` `gehe` `liha` `nihau` `suha` `sai` `cu'i` `cuhi` `ci'au'u'au'i` `cu'ei'ai` `cu'ei'ei` `cu'ei'oi` `cu'ei'a` `cu'ei'e` `cu'ei'i` `cu'ei'o` `cu'ei'u` `fu'ei'a` `fu'ei'e` `fu'ei'i` `fu'ei'o` `fu'ei'u` `ra'i'au` `xau'e'o` `xau'o'o` `bu'a'a` `ke'e'u` `te'i'o` `xa'a'a` `bo'oi` `cau'i` `ci'ai` `cu'ei` `dau'a` `dau'i` `de'ai` `de'au` `de'oi` `do'ai` `doi'a` `fai'a` `fu'au` `je'au` `jei'u` `ji'ei` `kai'a` `kai'e` `ko'oi` `koi'e` `lai'i` `mau'i` `mau'u` `me'ai` `moi'i` `na'oi` `ne'au` `pe'ai` `sei'i` `ta'ei` `toi'e` `toi'o` `vei'i` `xai'a` `zai'a` `zi'ai` `ba'u` `bi'a` `bi'u` `do'a` `fu'i` `jo'a` `mi'u` `mu'a` `na'i` `ne'e` `pe'a` `re'e` `ri'e` `ro'e` `ro'i` `ro'u` `sa'a` `sa'u` `se'i` `xa'a` `xa'i` `xo'o` `xy'y` `zi'a` `a'a` `i'o` `i'u` | ✅ |
| VAU | `vau` | ✅ |
| VUhO | `vu'o` `vuho` | ✅ |
| Y |  | ✅ |
| NAI | `nai` | ✅ |
| MAI | `pamai` `remai` `cimai` `vomai` `mumai` `xamai` `zemai` `bimai` `somai` `nomai` `mai` `mo'o` `moho` `ba'ai` | ✅ |
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
| BAhE | `bahe` `ba'ei` `ba'e` `za'e` `zai'e` `ba'ei` | ✅ |
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
