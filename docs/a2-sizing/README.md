# A2.0 sizing: what a capsule game costs on DTU

Checked-in rows of the A2.0 timing run, for auditability. They size the
cluster runs of A2; they are not a strength result. About four games per
pairing per deck cannot separate two policies, so the outcome column is
read only for the identity check below. The run was registered before it
ran, in the planning vault's "Arcana A2.0 sizing plan (2026-10-08)".

## Provenance

- Commit `c4611884` (`a2/sizing`), rustc 1.99.0, release.
- DTU `hpc` queue, 2026-10-08: build job 29655997 (four slots on
  `n-62-11-69`), timing array 29656365 (five one-slot tasks on five nodes).
- `benchmark::tests::capsule_timing` with its defaults: July's
  `vmc-material` (6 rollouts, depth 25, 10 candidates) in the capsule mirror
  against PIMC at 12/120, 16/160 and 32/320 and against itself, games 0 to 3
  of `pair_game`'s schedule, one deck per task (`CAP_DECK` 0 to 4).
- Laptop reference: oneiros, the same commit's source, `CAP_DECK=0
  CAP_TIMING_GAMES=2 CAP_TIMING_BUDGETS=12/120`.

## Files

| File | What |
|---|---|
| `timing-dtu.csv` | 80 games: deck, pairing, game index, outcome (`a` = `vmc-material` won, `b` = its opponent won, `draw`), wall seconds |
| `timing-laptop-deck0.csv` | the laptop's four reference games for deck 0 |

## Commands

```bash
# on the login node, through the control socket (the commit must be pushed)
ssh -o ControlPath=~/.ssh/cm-hpc hpc 'bash -s -- <commit>' < arcana-ai/scripts/dtu/setup.sh
# then, in a login shell there
cd ~/arcana-runs && bsub < ~/Repos/arcana/arcana-ai/scripts/dtu/build.lsf
cd ~/arcana-runs && bsub -w 'done(<build job>)' < ~/Repos/arcana/arcana-ai/scripts/dtu/timing.lsf
```

The run itself passed the commit with `bsub -env "all, ARCANA_COMMIT=c4611884…"`,
because DTU's LSF does not forward the submitting shell's environment; the
script now reads the commit from the checkout instead.

## Results

Identity: deck 0's games 0 and 1 at 12/120 and in the `vmc-material` mirror
ended the same on DTU as on the laptop, four of four.

Seconds per game on DTU, 20 games per pairing across the five decks:

| pairing | mean | median | max |
|---|---:|---:|---:|
| `vmc-material` vs PIMC 12/120 | 17.6 | 17.8 | 41.0 |
| `vmc-material` vs PIMC 16/160 | 24.3 | 19.5 | 47.9 |
| `vmc-material` vs PIMC 32/320 | 85.7 | 74.4 | 207.6 |
| `vmc-material` mirror | 3.0 | 2.8 | 4.9 |

UW Spirit Aggro is the slowest deck at every budget (136 s mean at 32/320).
A task peaked at 79 MB. The build compiled in 6 min 35 s on four slots, its
largest process (rustc on `arcana-cards`) at 6.08 GB.

Criterion medians (`arcana-core`'s `simulation` bench, seed-card deck,
`--warm-up-time 1 --measurement-time 4`) at this commit:

| bench | DTU | laptop |
|---|---:|---:|
| `full_game/random_vs_random` | 11.9 ms | 8.39 ms |
| `state_clone_midgame` | 29.9 µs | 21.0 µs |
| `legal_actions_midgame` | 8.15 µs | 8.32 µs |
| `single_step_midgame` | 24.9 µs | 34.1 µs |
| `clone_breakdown/full_state` | 30.4 µs | 20.4 µs |
