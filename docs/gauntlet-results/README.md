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

# 4. Placement/conversion validation artifacts.
python3 arcana-ai/scripts/placement_strength.py --zip kaggle.zip --format PI --min-entries 30 \
    > docs/gauntlet-results/metagame-placement-PI.txt
python3 arcana-ai/scripts/placement_bucket_bootstrap.py --zip kaggle.zip --format PI \
    --samples 2000 --seed 1 > docs/gauntlet-results/validation-target-audit-PI.txt
python3 arcana-ai/scripts/placement_vs_gauntlet.py --zip kaggle.zip --format PI \
    --gauntlet docs/gauntlet-results/pi_gauntlet_62.csv
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
| `metagame-placement-PI.txt` | Pioneer MTGTop8 recorded-finish conversion proxy (top-finish-censored, not ground truth) |
| `metagame-placement-MO.txt` | same, Modern |
| `validation-target-audit-PI.txt` | Pioneer field-size census + event-bootstrap CIs for the repaired validation proxy |
| `subset-provenance-PI.txt` | provenance repair for historical subset CSVs; old source ids unrecoverable, future corpus CSVs include source ids |
| `gauntlet-vs-real-PI.txt` | **external validation check** — gauntlet vmc point-rate vs MTGTop8 recorded-conversion proxy (ρ=-0.20/-0.30 after alias repair) |
| `pimc-vs-real-PI.txt` | **direct PIMC-vs-validation** — small-PIMC also fails the MTGTop8 conversion proxy (ρ≈-0.10/-0.50); implicates fidelity + field + validation target, not only referee |
| `fidelity-subset-PI.txt` | **fidelity control** — 0/65 covered decks are zero-GAP; field collapses; blockers concentrate on synergy payoffs (worklist by leverage) |
| `fidelity32_vmc.csv` | VMC gauntlet on the recovered 32-deck fidelity-fixed relaxed field |
| `fidelity-vmc-vs-real-PI.txt` | **fidelity payoff check** — after the grind, VMC still fails the conversion proxy (ρ=-0.40/-0.20); Spirits are near target post-fix, but causal attribution is not controlled |
| `pimc-arm-prereg-PI.txt` | **pre-registered** predictions for the referee arm (committed before the result) |
| `fidelity_sub8_{vmc,pimc}.csv` | VMC + higher-budget-PIMC (16/160) gauntlets on the same 8-deck fidelity-fixed spanning subset |
| `pimc-arm-result-PI.txt` | **referee arm** — mixed/partial strict prereg score; Gruul↓ and Scales↑, but thresholds mostly miss, Sultai is n=2 down, and regression-to-mean is an uncalibrated null |
| `control-arm-PI.txt` | **same-referee control (plan/runbook)** — regression-to-mean null for the referee arm: VMC re-run at two seeds on a fresh source-auditable spanning-8; manifest + commands + pre-registered verdict rule (results appended when run) |

The placement artifacts are built from the dump's recorded finish rows
(`df_events_v2.csv` for each event's format; `events/<id>/players_info.csv` for
each player's `player_result` finish + `player_title` archetype) via
`arcana-ai/scripts/placement_strength.py`, `placement_vs_gauntlet.py`, and
`placement_bucket_bootstrap.py`.

CSV columns: `deck,games,wins,draws,losses,score,point_rate,ci_lo,ci_hi`
(`score`/`point_rate` count a draw as ½; CIs are block-bootstrap 95%).

## Reading the numbers — caveats

- **Coverage ≠ fidelity.** A deck counted "playable" has every maindeck card
  *registered*, but some are **approximated** (e.g. Tarmogoyf is a fixed 4/5;
  various modal/triggered riders are GAP'd). A fidelity/GAP-card score per deck
  is a planned addition; until then "playable" means "resolves", not "faithful".
- **MTGTop8 validation target is censored.** The Kaggle rows used here are top
  finishers only; for Pioneer, all 528 usable events have only 2-8 recorded
  players. `recorded_strength` is within-recorded-finish conversion, and
  `event_win_share` is event wins / recorded rows. They are not full-field
  archetype strength or P(make top 8).
- **Referee bias.** Rankings are under the cheap **ValueMc(MaterialValue)**
  referee (and, where named, *small-budget* PIMC) — "strength under this referee",
  which flatters aggro and penalizes synergy/control.
- **Selection bias.** The field is the decks we can fully represent, not the real
  metagame (Modern's playable field is ~all Jund — see `mo_gauntlet_43.csv`).
- **Historical subset provenance.** Older subset CSVs were emitted before source
  ids were preserved in deck names, so duplicate same-archetype rows cannot be
  traced back to exact Kaggle deck files. See `subset-provenance-PI.txt`. Future
  corpus gauntlets append `[source_id]` to archetype names.
- **Maindeck-only.** The converter emits maindecks only; sideboards are dropped,
  so sideboard cards never affect coverage.
