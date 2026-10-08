# A2.2's field: the 64 playable Pioneer decks, pinned

`manifest-PI.csv` pins the field the A2.2 gauntlet plays: the Pioneer decks
whose maindecks today's catalog covers in full at exactly 60 cards, in the
order `corpus_gauntlet_shard` indexes them, each with its MTGTop8 source id and
the SHA-256 of its converted decklist.

## Provenance

- `kaggle.zip`, sha256 `25c49afce58d8b4a4948311b6dbe50deb58d5169bdb21461e134456727958572`
  (the pin in `docs/gauntlet-results/README.md`), converted by
  `arcana-ai/scripts/kaggle_to_decklists.py --formats MO LE PI VI --cap 800`:
  800 distinct decks per format.
- The coverage scan on the catalog of 2026-10-08 matches
  `docs/gauntlet-results/coverage-snapshot.txt` exactly: 64 of 800 Pioneer
  decks playable.
- The listing came from `corpus_gauntlet_shard` with `CORPUS_DUELS=0`, which
  prints the playable decks and plays no game.

## Against July's field

July's headline gauntlet, `docs/gauntlet-results/pi_gauntlet_62.csv`, played
62 decks. Its file records archetype names only, and those repeat, so its
decks cannot be matched to these one by one. `docs/rl-status.md` records that
July's coverage scans differed by a couple of decks between its 64-deck and
62-deck snapshots. Any comparison between A2.2's field and July's is therefore
between two fields drawn from the same dump, whose exact overlap cannot be
reconstructed. It is not a deck-by-deck comparison.

## Regenerating

```bash
python3 -I arcana-ai/scripts/kaggle_to_decklists.py --zip kaggle.zip --out <dir> --formats MO LE PI VI --cap 800
KAGGLE_DECKS=<dir> CORPUS_DUELS=0 cargo test -p arcana-ai --release corpus_gauntlet_shard -- --ignored --nocapture
```
