# Pioneer capsule — a frozen deckbuilding research domain

The first closed deckbuilding loop runs **inside this capsule**: a fixed legal
card pool + a fixed opponent field. **All claims here apply only inside this
capsule** — it is a deliberately narrow domain for getting the
build → evaluate → diagnose loop honest, not a Pioneer metagame.

## Field (opponent decks) — `seeds/`

Five coverage-verified MTGTop8 Pioneer archetypes, chosen to span the referee
bias measured in `../gauntlet-results/referee-sensitivity.txt`:

| Archetype | Role |
|---|---|
| Red Deck Wins | aggro — **VMC-favored** |
| Rakdos Aggro | aggro/midrange |
| Hardened Scales | +1/+1 synergy — **VMC-underrated** |
| Golgari Scales | +1/+1 synergy — **VMC-underrated** |
| UW Spirit Aggro | spirits / tempo |

Each is a real, fully-covered 60-card maindeck (maindeck-only; sideboards
dropped). Exact lists are in `seeds/*.txt` (Arena format with an archetype
`Name` header).

## Legal card pool — `card-pool.txt`

The union of the seeds' cards (57 distinct). The optimizer may build any legal
60-card maindeck from this pool: **≤4 copies of a nonbasic, unlimited basics**,
with a land count in **[17, 27]** (a mana-sanity band; the land base IS part of
the search, not fixed).

## Objective / referee

Fitness = mean **point-rate** (draws ½) of a candidate deck vs all five field
decks, `CAPSULE_GAMES` seat-alternating games each, under the
**`VmcMaterial`** referee (`ValueMc(MaterialValue)`, the cheap material-in-search
policy — small budget). This is the objective whose bias we are probing.

## Known approximations

Several pool cards are GAP-approximated (so "playable" ≠ "faithful") — see
`../gauntlet-results/approximated-cards.txt`. Relevant here: **Hardened Scales /
Golgari Scales** synergy payoffs are partly elided, which is *exactly* the
direction that depresses those decks under VmcMaterial — keep this in mind when
reading what the optimizer rewards.

## Run

```bash
CAPSULE_DIR=$(pwd)/docs/capsule-pioneer/seeds CAPSULE_ITERS=120 CAPSULE_GAMES=5 \
  cargo test -p arcana-ai --release deckbuild_capsule_run -- --ignored --nocapture
```

Optimizer: a simple (1+1) hill-climb (`arcana-ai/src/deckbuild.rs`) — the
simplest thing that closes the loop. **Its purpose is diagnostic:** does the
objective reward recognizable Magic structure, or aggro-abuse / mana-nonsense /
a narrow referee exploit?

Results:
- `results.txt` — the optimizer diagnostic + the PIMC-validation A/B (v1/v2/PIMC):
  the VmcMaterial gain *inverts* under PIMC; v2 is a better judge but still a
  gameable target (Goodhart).
- `pimc-select.txt` — cheap candidate generation + PIMC selection: selection helps
  (recovers a coherent mid-field deck the single-shot optimizer missed), with a
  ceiling below the top seed decks.
