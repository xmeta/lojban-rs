English | [日本語](README.md)

# lojban — A Lojban PEG Parser in Rust

A PEG parser for Lojban (the constructed language) text.
The parser engine is [pest](https://pest.rs); the grammar in
`src/grammar/lojban.pest` is a Rust/pest port of guskant's
**zantufa-1.9999.peg**.

## Features

- **Morphology**: full word-shape recognition ported from zantufa
  - cmavo (structure words) / brivla (content words: gismu, lujvo, fu'ivla) / cmevla (names)
  - Syllable structure, diphthongs, and consonant-cluster legality checks
    (zantufa's consonant rules, ported)
  - Stress marking via uppercase vowels (e.g. `GERku`)
  - lujvo recognition through rafsi decomposition (including the slinku'i guard)
- **Syntax** (core subset)
  - Sentences: terms + `cu` + predicate (bridi_tail)
  - sumti: pronouns (KOhA) / descriptions (`le lo la…`) / quantified descriptions
    (`ro lo remna`) / numerals / letterals (BY: `xy`, `abu`, …; usable as sumti and
    as mex operands) / connectives (`e` `a` `o` `u` + `nai`, JOI series)
  - selbri: tanru / `na` negation / `na'e to'e` scale flipping / `se te ve xe` conversion
    / `ja'a` affirmation / `co` inversion / tanru connectives
    (`melbi je cmalu`, `je bo`) / bridi-tail chains (`gi'e` `gi'a` …,
    `gi'e bo`) / JAI conversion (`jai gau …`)
  - `be … bei … be'o` linked sumti
  - Abstractions (`nu ka ni zu'o …`, SE conversion `se du'u` and the joined
    form `sedu'u`) / `ke … ke'e` grouping
  - Quotations (`lu … li'u` text quotes / `zo` word quotes as sumti;
    `lo'u … le'u` error quotes as free modifiers)
  - Relative clauses (`poi / noi`) / possessives (`pe / po / goi`)
  - Free modifiers (attitudinals such as `ui` with the intensity scale
    `cai sai ru'e cu'i`, discursives `ku'i` `ja'o` `po'o` `da'i` `je'u` etc.,
    `xu` questions, `sei` inserts, `to … toi` parentheticals,
    `soi … vo'a vo'e` swaps, utterance ordinals `pamai`, subscripts `xi re`,
    `da'o`, etc.; chains
    (`mu'o ge'e coi`) and insertion between terms and predicates
    (`xu do su'a djica`) are also accepted)
  - **ZEI compounds** (`zdani zei sinxa`): full words joined into a
    lujvo-equivalent unit
  - Vocatives (`coi …`) / sentence connectives (`.i`, `ni'o`, including joined
    forms such as `.ije` `.ijanai` `.ibo` `.ijebo`, and `.i bo` grouping)
  - Forethought connectives (`ge … gi` for sumti and sentences, including
    joined NAI forms `ganai … ginai` and separated `ga nai … gi nai`)
  - **Tense & aspect** (PU `pu ca ba` / CAhA `ka'e ca'a …` / ZAhO `co'a ca'o ba'o …`
    / ZI `zi za zu` / VA `vi va vu` / TAhE `ta'e di'i na'o ru'i`)
  - **Spatial & motion tense** (FAhA directions `ca'u ti'a zu'a ga'u ni'a …`,
    MOhI motion marking as in `mo'i ca'u`), **time intervals** (ZEhA as in
    `pu bi'o ba` / `ca bi'i ba`), **space intervals** (VEhA/VIhA as in
    `ve'i ne'i le zdani` / `vi'a ca'u`), and **tense tags taking a sumti**
    (`mi ca le cabdei cu klama` / `vi ne'i le zdani` / durations `ze'a lo cacra`)
  - **Term reinforcement**: LAhE reference (`la'e di'u` / `lu'e le cukta`),
    term-position negation with `naku` (NA KU), KOhA completion
    (`mi'a` `ma'a` `do'o` `di'u` `tu'a` `dei`, etc.),
    description articles `lo'e` / `le'e`
  - Quantifier + selbri terms (`pa prenu cu klama`), quantifiers inside descriptions
    (`le ci gerku`)
  - `me` predicates (with elidable `me'u`), number predicates MOI
    (`mi re moi` / `mi ci mei`), term-only fragments (bare `mi`),
    bare attitudinals (`.ui`)
  - Forethought selbri connectives (GUhA `gu'e … gi`, optionally with NAhE)
    and GAhO interval endpoints (`ga'o bi'o ke'i`)
- **Output**: pretty-printed tree / S-expressions / JSON
  (`{"rule","text","children"}` format)
- **lujvo construction & decomposition** (per CLL 4.11/4.12): assembles new
  words from rafsi sequences applying hyphen rules (r/n/y), the tosmabru test,
  and medial cluster legality (CLL 3.6), with official scoring
  (`--build-lujvo` / `lojban::lujvo::build()`). The reverse direction is also
  supported (`--split-lujvo` / `lojban::lujvo::decompose()`); the
  build → decompose roundtrip is covered by tests

## Usage

```console
# Pretty-printed tree
$ lojban "mi tavla do"

# S-expression output
$ lojban "le mlatu cu cadzu" --sexpr

# JSON output
$ lojban "le mlatu cu cadzu" --json

# Read from stdin
$ echo "coi la alis." | lojban

# Build a lujvo (CLL 4.11 hyphen rules + tosmabru test + CLL 4.12 scoring)
$ lojban --build-lujvo "zba sai"
zbasai (score 5847)

# Split a lujvo into its rafsi
$ lojban --split-lujvo "sairzbata'u"
sai (Cvv)
-r- [hyphen]
zba (Ccv)
ta'u (CvvApo)
```

### Library API

```rust
use lojban::{parse, tree};

let pairs = parse("mi viska le gerku")?;
println!("{}", tree::to_sexpr(pairs));
```

## Development

```console
$ cargo test      # all tests (150 = 146 unit + 4 doc; includes 283 corpus sentences)
$ cargo clippy --all-targets
$ cargo run -- "mi klama"
$ cargo bench    # performance benchmarks (criterion)
```

## Architecture

```
src/
├── lib.rs            # public API (parse / preprocessing pipeline)
├── main.rs           # CLI (clap)
├── tree.rs           # parse tree → S-expr / tree / JSON rendering
└── grammar/
    ├── mod.rs        # LojbanParser (pest_derive)
    └── lojban.pest   # Lojban grammar (ported from zantufa)
```

Sections of the grammar file:

| Section | Contents |
|---|---|
| Characters & phonology | vowel/consonant rules (with cluster legality), syllables, stress lookahead |
| Morphology | cmavo_form, cmevla (jbocme/zifcme), brivla (full rafsi set) |
| cmavo classes | vocabulary lists per selmaho (`*_core`) + word-boundary wrappers (`*_clause`) |
| Syntax | text → content → sentence → terms/sumti/selbri/tanru → free |

## Performance

In a simple same-sentence benchmark this parser runs about **5–8× faster**
than the reference implementation camxes.js (JavaScript) — measured at v0.9;
parsing costs have since grown with the expanded feature set, so see
`cargo bench` for current absolute numbers. See
[docs/comparison.md](docs/comparison.md) for details and caveats.
Reproduce with: `cargo run --release --example speed_check`

## References

- **Primary source**: [guskant/gerna_cipra](https://github.com/guskant/gerna_cipra) `zantufa-1.9999.peg`
- For differences: [lojban/ilmentufa](https://github.com/lojban/ilmentufa) `camxes.peg` (current standard grammar)
- Technique reference: [lojban/lensisku](https://github.com/lojban/lensisku) `src/grammar/*.peg`

## Tests

| File | Contents |
|---|---|
| `tests/morphology.rs` | Word recognition (gismu/lujvo/cmevla/stress, etc.) |
| `tests/syntax.rs` | Syntactic structure verification |
| `tests/fuzz.rs` | Lightweight fuzzing (random input, mutations, nesting sweep). Heavy variants: `cargo test -- --ignored` |
| `tests/corpus.rs` | **283 real-world sentences** (220 Tatoeba sentences, CC BY 2.0 FR + 63 curated CLL-style examples) |

The real-sentence corpus uses Lojban sentences from [Tatoeba](https://tatoeba.org).

## Known Limitations & Roadmap

- Connectives are supported in their basic forms (including `bo` grouping,
  BIhI interval connectives, forethought selbri GUhA, forethought
  `ganai … ginai` and separated `ga nai … gi nai`, and MAhO operators).
  FUhE/FUhO forethought logic and termsets (NUhI) are not implemented
- mex arithmetic is supported as LI…LOhO sumti and inside description
  quantifiers (`le re su'i ci gerku`) with `vei … ve'o` parentheses and
  `ki'o` `ma'u` `ni'u`. Operators form simple left-associative chains and
  support SE conversion (`se pi'i`), NAhU-derived operators (`na'u zmadu`),
  MAhO (`ma'o ny`), BIhI intervals, mo'e+sumti operands (`mo'e ti`), and the
  forethought form (`peho su'i re ci [kuhe]`)
- lerfu supports BY words (`by`, `xy`, `abu`, …) and BU conversion
  (any word followed by `bu`)
- Erasure (SI/SU): semantics are applied before parsing (`si` erases the
  previous word, `su` erases back to the start of the utterance; content
  inside quotes and the word after `zo` are protected). The parse tree
  reflects the text after erasure
- Quotations: `lu … li'u` (nestable), `zo`, `lo'u … le'u`, and
  `zoi DELIM body DELIM`. For ZOI, delimiter matching is validated by a
  pre-parse scan and the body is normalized to `zo'e` in the parse tree
  (pest has no backreferences; unclosed/mismatched delimiters are errors)
- Tags: FA and BAI (`bau`, `mu'i`, …; attached to terms and sentence fronts,
  negatable with NAI as in `ri'a nai`),
  SE-converted modals (separated `se ki'u …` and joined `sepi'o` `seva'u`
  `semu'i`, …), FIhO modal tags with elidable `fe'u`
  (`fi'o dunda [fe'u] do`), and tense-mark chains followed by a sumti
  (`pu le cabdei ku` / `vi ne'i le zdani`), in term position either before or
  after the selbri. Selbri marks include the affirmative `ja'a` alongside
  negator `na` (as in `ja'a go'i`)
- cmavo vocabulary covers the main standard CLL words (experimental cmavo are
  not included; extend by adding alternatives to each `*_core` in `lojban.pest`)
- Pause-less adjacent words allowed by zantufa (e.g. `mibroda` = mi+broda)
  are NOT accepted; this parser always requires pauses (whitespace, `.`,
  `,`, `!`, `?`) between words
- Vowel-only words (e.g. `iii`) are accepted as fu'ivla, following zantufa
- Nesting of quotes (lu / lo'u) and mex parentheses (vei) is limited to a
  depth of 8; deeper nesting is rejected quickly (resource guard against
  exponential PEG backtracking)
- Rejecting non-word tokens (e.g. typos) can take on the order of 100 ms due
  to rafsi-decomposition backtracking

## License

This project is dual-licensed under [MIT OR Apache-2.0](LICENSE-MIT).

The reference grammars zantufa / camxes remain under their respective licenses.
