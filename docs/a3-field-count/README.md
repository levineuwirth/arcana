# A3's field count: which missing cards gate real decks

The count behind the planning vault's "Arcana R4 field count (2026-10-09)",
kept in the repository by D14. That ruling makes A3 a targeted card phase,
scoping first the five staples that gate the most real decks: Urborg, Tomb of
Yawgmoth; Embercleave; Lightning Axe; Collected Company; and Wicked Wolf. The
count supports prioritization only. Its "decks made playable" figures are
potential catalog coverage, not a demonstrated faithful field. Card fidelity
is a separate question (see the last section).

## Inputs

| input | identity |
|---|---|
| MTGTop8 decklists | `kaggle.zip`, sha256 `25c49afce58d8b4a4948311b6dbe50deb58d5169bdb21461e134456727958572` (the pin in `docs/gauntlet-results/README.md`), converted by `arcana-ai/scripts/kaggle_to_decklists.py --formats MO LE PI VI --cap 800`, as in `docs/a2-field/README.md` |
| the catalog | `main` at `9d431371`, whose cards `a2/field`'s `5e82c4b7` shares |
| Scryfall oracle data | `oracle-cards` bulk file, updated 2026-10-09 09:01:54 UTC, `https://data.scryfall.io/oracle-cards/oracle-cards-20261009090154.jsonl.gz`, sha256 `0ca0d50138e5cf10e8d713e1169ebf2ffc928caaaae71292e0e62a793d1348be` (not kept here: 24.6 MB, and Scryfall replaces it daily) |

## Reproducing

```bash
python3 -I arcana-ai/scripts/kaggle_to_decklists.py --zip kaggle.zip --out <decks> --formats MO LE PI VI --cap 800
KAGGLE_DECKS=<decks> cargo test -q -p arcana-ai --lib deck_corpus::tests::corpus_unresolved \
  -- --ignored --exact --nocapture --test-threads=2 | grep '^{' > docs/a3-field-count/unresolved.jsonl
P=arcana-ai/scripts/r4_field_count.py; O=docs/a3-field-count; OR=<oracle-cards.jsonl.gz>
python3 -I $P families $O/unresolved.jsonl $OR > $O/families.txt
python3 -I $P cards $O/unresolved.jsonl $OR --format PI > $O/cards-PI.txt
python3 -I $P greedy $O/unresolved.jsonl --format PI > $O/greedy-PI.txt
python3 -I $P greedy $O/unresolved.jsonl --format ALL > $O/greedy-ALL.txt
python3 -I $P ltb $OR <decks>/PI docs/a2-field/manifest-PI.csv docs/capsule-pioneer/seeds > $O/ltb.txt
python3 -I arcana-ai/scripts/test_r4_field_count.py
```

`unresolved.jsonl` (sha256 `49a63667…`) is the extractor's output on these
inputs: one JSON object for each of 3,200 decks.

## Outputs

| file | what |
|---|---|
| `unresolved.jsonl` | every deck's format, source id, resolved and listed copies, and missing names with copies |
| `families.txt` | missing cards by R4 family, and the decks each family touches and unblocks alone, per format |
| `cards-PI.txt` | the missing cards touching the most Pioneer decks, each with the decks it alone blocks |
| `greedy-PI.txt`, `greedy-ALL.txt` | the cards that complete the most blocked 60-card decks, in order, and the totals after 5, 10, 20, 30 and 50 |
| `ltb.txt` | A1.0a's exposure in A2.2's field and the capsule |

In Pioneer, 64 decks are playable and 723 blocked 60-card decks remain. All
of R4's families together unblock 22 of them; across the four formats, 25 of
2,960. The first five greedy cards unblock 167 Pioneer decks, and fifty
unblock 331 (394 across the four formats).

## The deck-size error this guards

A first count filtered blocked decks on `LoadedDeck::main_count() == 60`.
`main_count` counts resolved cards only, so every deck with a missing card
fell out of the filter, and every "unblocked" figure was zero. A deck's size
is `listed_count()`, resolved plus missing copies. Two tests hold this:
`listed_count_counts_unresolved_copies` in `deck_corpus.rs`, and
`test_a_deck_is_sized_by_its_listed_copies` in `test_r4_field_count.py`.

## What `ltb.txt` measures

It counts cards whose oracle text has a trigger of their own on dying or
leaving the battlefield, the triggers A1.0a stopped resolving to nothing:
150 of 3,840 maindeck copies in 43 of A2.2's 64 decks, and 14 of 300 in four
of the capsule's five. That is oracle-ability exposure, not effects the fix
restored. A review found two of the six cards still unfaithful:

- **A1.5l:** Spell Queller's two triggers are no-ops marked GAP, and its leave
  trigger is modeled as `SelfDies` (`arcana-cards/src/inr/spell_queller.rs`).
- **A1.5m:** Hangarback Walker's dies trigger looks its object up in the live
  arena, where the departed object, re-identified on its zone change, has no
  counters, so it makes no Thopters (`arcana-cards/src/ori/hangarback_walker.rs`).

Both are recorded and left as they are inside A2.2's registered implementation.
