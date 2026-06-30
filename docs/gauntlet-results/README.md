# Gauntlet results & reproducibility

Checked-in snapshots of the deck-gauntlet experiments described in
`docs/rl-status.md`, plus exact commands to regenerate them. These are
**generated artifacts** committed for auditability, not source.

## Dataset

- **Source:** Kaggle — *Magic: the Gathering — Top8 some decks and events*
  (camilonunez). A ~125k-deck MTGTop8 dump (per-player deck JSONs +
  `df_events_v2.csv` for the format of each event).
- **Local file:** `kaggle.zip` at the repo root — **gitignored** (90 MB), not
  committed.
- **sha256(kaggle.zip):** `25c49afce58d8b4a4948311b6dbe50deb58d5169bdb21461e134456727958572`

## Pipeline (exact commands)

```bash
# 1. Convert the dump -> de-doubled, archetype-named Arena decklists per format
#    (Standard + EDH skipped; dedup by maindeck multiset; cap 800/format).
#    NOTE: writes MAINDECK ONLY.
python3 arcana-ai/scripts/kaggle_to_decklists.py \
    --zip kaggle.zip --out /tmp/kaggle_decks --formats MO LE PI VI --cap 800

# 2. Coverage snapshot (which real decks our catalog can fully represent).
KAGGLE_DECKS=/tmp/kaggle_decks \
    cargo test -p arcana-ai --release coverage_scan -- --ignored --nocapture

# 3. A gauntlet on the playable subset of one format. Referee/duels via env:
#    CORPUS_REFEREE = random | vmc | pimc   (default vmc = ValueMc(MaterialValue))
#    CORPUS_DUELS   = paired duels per pair (each duel = 2 seat-swapped games)
KAGGLE_DECKS=/tmp/kaggle_decks CORPUS_FORMAT=PI CORPUS_REFEREE=vmc CORPUS_DUELS=1 \
    cargo test -p arcana-ai --release corpus_gauntlet -- --ignored --nocapture
```

The catalog name list used by the worklist analysis is dumped from
`arcana_cards::build_catalog()` (one row per registered card name).

## Files

| File | What |
|---|---|
| `coverage-snapshot.txt` | per-format mean coverage + fully-playable counts + top-missing cards |
| `missing-card-worklist.txt` | cards blocking the most *near-playable* (≤4-short) decks, per format |
| `pi_gauntlet_62.csv` | Pioneer 62-deck gauntlet, vmc-material, 1 duel/pair (122 games/deck) |
| `mo_gauntlet_43.csv` | Modern 43-deck gauntlet, vmc-material, 1 duel/pair (84 games/deck) |
| `referee-sensitivity.txt` | rankings under Random / VmcMaterial / small-PIMC + rank correlations |
| `pi_subset12_vmc.csv` | 12-deck spanning subset under vmc (referee-sensitivity arm) |
| `pi_subset12_pimc.csv` | same 12-deck subset under small-PIMC |

CSV columns: `deck,games,wins,draws,losses,score,point_rate,ci_lo,ci_hi`
(`score`/`point_rate` count a draw as ½; CIs are block-bootstrap 95%).

## Reading the numbers — caveats

- **Coverage ≠ fidelity.** A deck counted "playable" has every maindeck card
  *registered*, but some are **approximated** (e.g. Tarmogoyf is a fixed 4/5;
  various modal/triggered riders are GAP'd). A fidelity/GAP-card score per deck
  is a planned addition; until then "playable" means "resolves", not "faithful".
- **Referee bias.** Rankings are under the cheap **ValueMc(MaterialValue)**
  referee (and, where named, *small-budget* PIMC) — "strength under this referee",
  which flatters aggro and penalizes synergy/control.
- **Selection bias.** The field is the decks we can fully represent, not the real
  metagame (Modern's playable field is ~all Jund — see `mo_gauntlet_43.csv`).
- **Maindeck-only.** The converter emits maindecks only; sideboards are dropped,
  so sideboard cards never affect coverage.
