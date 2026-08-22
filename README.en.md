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
    / `co` inversion / tanru connectives (`melbi je cmalu`, `je bo`) /
    bridi-tail chains (`gi'e` `gi'a` …)
  - `be … bei … be'o` linked sumti
  - Abstractions (`nu ka ni zu'o …`) / `ke … ke'e` grouping
  - Quotations (`lu … li'u` text quotes / `zo` word quotes as sumti;
    `lo'u … le'u` error quotes as free modifiers)
  - Relative clauses (`poi / noi`) / possessives (`pe / po / goi`)
  - Free modifiers (attitudinals such as `ui`, `xu` questions, `sei` inserts,
    `to … toi` parentheticals; chains (`mu'o ge'e coi`) and insertion between
    terms and predicates (`xu do su'a djica`) are also accepted)
  - Vocatives (`coi …`) / sentence connectives (`.i`, `ni'o`, including joined
    forms such as `.ije` `.ijanai`)
  - Forethought connectives (`ge … gi` for sumti and sentences)
  - **Tense & aspect** (PU `pu ca ba` / CAhA `ka'e ca'a …` / ZAhO `co'a ca'o ba'o …`
    / ZI `zi za zu` / VA `vi va vu` / TAhE `ta'e di'i na'o ru'i`)
  - Quantifier + selbri terms (`pa prenu cu klama`), quantifiers inside descriptions
    (`le ci gerku`)
  - `me` predicates, term-only fragments (bare `mi`), bare attitudinals (`.ui`)
  - Forethought selbri connectives (GUhA `gu'e … gi`, optionally with NAhE)
    and GAhO interval endpoints (`ga'o bi'o ke'i`)
- **Output**: pretty-printed tree / S-expressions / JSON
  (`{"rule","text","children"}` format)

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
```

### Library API

```rust
use lojban::{parse, tree};

let pairs = parse("mi viska le gerku")?;
println!("{}", tree::to_sexpr(pairs));
```

## Development

```console
$ cargo test      # all tests (88 = 84 unit + 4 doc; includes 223 corpus sentences)
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
than the reference implementation camxes.js (JavaScript). See
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
| `tests/corpus.rs` | **160 real Tatoeba sentences** (CC BY 2.0 FR) + 63 curated CLL-style examples |

The real-sentence corpus uses Lojban sentences from [Tatoeba](https://tatoeba.org).

## Known Limitations & Roadmap

- Connectives are supported in their basic forms (including `bo` grouping,
  BIhI interval connectives `bi'o bi'i mi'i`, and forethought selbri GUhA).
  Forethought operators (MAhO) and other mex details are not implemented
- mex arithmetic is supported as LI…LOhO sumti and inside description
  quantifiers (`le re su'i ci gerku`) with `vei … ve'o` parentheses and
  `ki'o` `ma'u` `ni'u`. Operators form simple left-associative chains
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
- Tags: FA and BAI (`bau`, `mu'i`, …; attached to terms and sentence fronts)
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
