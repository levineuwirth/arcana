//! Constrained deckbuilding optimizer inside a frozen **capsule** — the first
//! closed deckbuilding loop: real corpus → covered card pool → mutate/build
//! decklists → evaluate vs a fixed field → see what the objective rewards.
//!
//! A [`Capsule`] freezes a research domain: a legal card pool (card → max
//! copies) and a fixed opponent field. The optimizer searches valid 60-card
//! maindecks (≤4 nonbasic copies, a mana-sanity land band) to maximize the
//! **VmcMaterial gauntlet score vs the field**. It is deliberately the simplest
//! thing that closes the loop (a (1+1) hill-climb), and its real purpose is
//! diagnostic: does it converge to recognizable Magic structure, or to
//! aggro-abuse / mana-nonsense / a narrow referee exploit? Claims apply only
//! inside the capsule.
//!
//! Caveats: fitness is noisy (few games/eval), so a hill-climb on it ratchets on
//! lucky evals — the *structure* of the output deck is the signal, not its exact
//! score. The land base is part of the search (bounded by the band), not fixed.

use std::collections::BTreeMap;

use arcana_core::registry::CardRegistry;
use arcana_core::types::{CardId, SupertypeSet};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::deckeval::Deck;
use crate::search::{win_rate, StatePolicy};

// =============================================================================
// Capsule
// =============================================================================

/// A frozen deckbuilding domain.
pub struct Capsule {
    pub name: String,
    /// The opponent decks the candidate is scored against.
    pub field: Vec<Deck>,
    /// Legal pool: `(card, max copies)`. Basics get a large cap; nonbasics ≤ 4.
    pub pool: Vec<(CardId, u32)>,
    pub deck_size: u32,
    pub min_lands: u32,
    pub max_lands: u32,
}

fn is_land(reg: &CardRegistry, c: CardId) -> bool {
    reg.get(c)
        .map(|d| d.initial_characteristics().types.is_land())
        .unwrap_or(false)
}

fn is_basic(reg: &CardRegistry, c: CardId) -> bool {
    reg.get(c)
        .map(|d| {
            let ch = d.initial_characteristics();
            ch.types.is_land() && (ch.supertypes.0 & SupertypeSet::BASIC != 0)
        })
        .unwrap_or(false)
}

impl Capsule {
    /// Build a capsule from seed decks: the pool is the union of their cards
    /// (legal-copy caps applied — 4 for nonbasics, `deck_size` for basics), and
    /// the field is the seeds themselves.
    pub fn from_decks(
        name: impl Into<String>,
        seeds: Vec<Deck>,
        reg: &CardRegistry,
        deck_size: u32,
        min_lands: u32,
        max_lands: u32,
    ) -> Self {
        let mut distinct: BTreeMap<CardId, ()> = BTreeMap::new();
        for d in &seeds {
            for &c in &d.cards {
                distinct.insert(c, ());
            }
        }
        let pool: Vec<(CardId, u32)> = distinct
            .keys()
            .map(|&c| (c, if is_basic(reg, c) { deck_size } else { 4 }))
            .collect();
        Capsule {
            name: name.into(),
            field: seeds,
            pool,
            deck_size,
            min_lands,
            max_lands,
        }
    }

    fn land_pool(&self, reg: &CardRegistry) -> Vec<(CardId, u32)> {
        self.pool.iter().copied().filter(|&(c, _)| is_land(reg, c)).collect()
    }
    fn spell_pool(&self, reg: &CardRegistry) -> Vec<(CardId, u32)> {
        self.pool.iter().copied().filter(|&(c, _)| !is_land(reg, c)).collect()
    }
}

// =============================================================================
// Candidate decks (count map)
// =============================================================================

/// A candidate maindeck as `card → count`.
type Counts = BTreeMap<CardId, u32>;

fn total(counts: &Counts) -> u32 {
    counts.values().sum()
}
fn land_total(counts: &Counts, reg: &CardRegistry) -> u32 {
    counts.iter().filter(|(&c, _)| is_land(reg, c)).map(|(_, &n)| n).sum()
}
fn flatten(counts: &Counts) -> Vec<CardId> {
    let mut v = Vec::new();
    for (&c, &n) in counts {
        for _ in 0..n {
            v.push(c);
        }
    }
    v
}

/// Fill `slots` cards from `pool` (each entry's cap respected), into `counts`.
fn fill_from(counts: &mut Counts, pool: &[(CardId, u32)], slots: u32, rng: &mut ChaCha8Rng) {
    if pool.is_empty() {
        return;
    }
    let mut added = 0;
    let mut guard = 0;
    while added < slots && guard < slots * 50 + 100 {
        guard += 1;
        let (c, cap) = pool[(rng.gen::<u64>() % pool.len() as u64) as usize];
        let cur = counts.get(&c).copied().unwrap_or(0);
        if cur < cap {
            *counts.entry(c).or_insert(0) += 1;
            added += 1;
        }
    }
}

/// A valid random deck: a land count chosen in the band, then lands + spells
/// filled from their sub-pools.
fn random_deck(cap: &Capsule, reg: &CardRegistry, rng: &mut ChaCha8Rng) -> Counts {
    let lands = cap.land_pool(reg);
    let spells = cap.spell_pool(reg);
    let l = cap.min_lands + (rng.gen::<u64>() % (cap.max_lands - cap.min_lands + 1) as u64) as u32;
    let mut c = Counts::new();
    fill_from(&mut c, &lands, l, rng);
    let need_spells = cap.deck_size.saturating_sub(total(&c));
    fill_from(&mut c, &spells, need_spells, rng);
    // top up with lands if the spell pool was too thin to reach deck_size
    let need_top = cap.deck_size.saturating_sub(total(&c));
    fill_from(&mut c, &lands, need_top, rng);
    c
}

/// One mutation: remove a random copy, add a random pool card (cap-respecting),
/// keeping the size and the land band (retry a few times to stay legal).
fn mutate(cap: &Capsule, reg: &CardRegistry, base: &Counts, rng: &mut ChaCha8Rng) -> Counts {
    for _ in 0..40 {
        let mut c = base.clone();
        // remove one copy of a present card
        let present: Vec<CardId> = c.keys().copied().collect();
        let victim = present[(rng.gen::<u64>() % present.len() as u64) as usize];
        if let Some(n) = c.get_mut(&victim) {
            *n -= 1;
            if *n == 0 {
                c.remove(&victim);
            }
        }
        // add one copy from the full pool (cap-respecting)
        fill_from(&mut c, &cap.pool, 1, rng);
        if total(&c) == cap.deck_size {
            let lt = land_total(&c, reg);
            if lt >= cap.min_lands && lt <= cap.max_lands {
                return c;
            }
        }
    }
    base.clone()
}

// =============================================================================
// Fitness + optimizer
// =============================================================================

/// Mean point-rate of `deck` vs every field deck under the referee `mk`
/// (`games_per_opp` seat-alternating games each). Draws count ½.
pub fn fitness(
    deck: &[CardId],
    cap: &Capsule,
    reg: &CardRegistry,
    games_per_opp: u32,
    max_steps: u32,
    seed: u64,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> f32 {
    let mut score = 0.0f32;
    let mut games = 0u32;
    for (i, opp) in cap.field.iter().enumerate() {
        let (w, _l, d) = win_rate(
            deck, &opp.cards, reg, games_per_opp, max_steps, mk, mk,
        );
        // seed is folded into win_rate via game index internally; vary per opp
        let _ = seed.wrapping_add(i as u64);
        score += w as f32 + 0.5 * d as f32;
        games += games_per_opp;
    }
    if games == 0 { 0.0 } else { score / games as f32 }
}

/// One optimizer run's result.
pub struct OptimizeResult {
    pub best: Vec<CardId>,
    pub best_fitness: f32,
    pub start_fitness: f32,
    /// Accepted-improvement fitness trajectory.
    pub history: Vec<f32>,
}

/// (1+1) hill-climb: from a random valid deck, mutate and accept strict fitness
/// improvements. `iters` candidate evaluations; the referee is the injected `mk`
/// (e.g. [`fixed_policy`] for VmcMaterial, [`crate::deckeval::fixed_policy_v2`]
/// for the rebalanced v2 leaf).
pub fn optimize(
    cap: &Capsule,
    reg: &CardRegistry,
    iters: u32,
    games_per_opp: u32,
    max_steps: u32,
    seed: u64,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> OptimizeResult {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut cur = random_deck(cap, reg, &mut rng);
    let mut cur_fit = fitness(&flatten(&cur), cap, reg, games_per_opp, max_steps, seed, mk);
    let start_fitness = cur_fit;
    let mut history = vec![cur_fit];
    for it in 0..iters {
        let child = mutate(cap, reg, &cur, &mut rng);
        let f = fitness(&flatten(&child), cap, reg, games_per_opp, max_steps,
                        seed.wrapping_add(it as u64 + 1), mk);
        if f > cur_fit {
            cur = child;
            cur_fit = f;
            history.push(cur_fit);
        }
    }
    OptimizeResult {
        best: flatten(&cur),
        best_fitness: cur_fit,
        start_fitness,
        history,
    }
}

/// Human-readable structure summary of a decklist (for the bias diagnostic):
/// counts, land count, average mana value of nonlands, and the top cards.
pub fn describe(deck: &[CardId], reg: &CardRegistry) -> String {
    let mut counts: Counts = Counts::new();
    for &c in deck {
        *counts.entry(c).or_insert(0) += 1;
    }
    let lands = land_total(&counts, reg);
    let nonland: Vec<(&str, u32, u32)> = counts
        .iter()
        .filter(|(&c, _)| !is_land(reg, c))
        .map(|(&c, &n)| {
            let d = reg.get(c);
            let name = d.and_then(|d| reg.interner().resolve(d.name)).unwrap_or("?");
            let mv = d.map(|d| d.initial_characteristics().mana_value()).unwrap_or(0);
            (name, n, mv)
        })
        .collect();
    let nl_copies: u32 = nonland.iter().map(|(_, n, _)| n).sum();
    let avg_mv = if nl_copies == 0 {
        0.0
    } else {
        nonland.iter().map(|(_, n, mv)| (*n * *mv) as f32).sum::<f32>() / nl_copies as f32
    };
    let mut s = format!(
        "{} cards: {} lands, {} nonland (distinct nonland {}), avg nonland MV {:.2}\n",
        deck.len(), lands, nl_copies, nonland.len(), avg_mv
    );
    let mut sorted = nonland.clone();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    for (name, n, mv) in sorted.iter().take(16) {
        s.push_str(&format!("  {n}x {name} (MV {mv})\n"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deckeval::{fixed_policy, mono_color_creature_deck};
    use crate::search::RandomStatePolicy;

    #[test]
    fn optimizer_runs_and_returns_a_valid_deck() {
        let reg = arcana_cards::build_catalog();
        // tiny capsule from two mono decks; random referee for speed.
        let seeds = vec![
            mono_color_creature_deck(&reg, 'R', 6, 22, 18, 4),
            mono_color_creature_deck(&reg, 'G', 6, 22, 18, 4),
        ];
        let cap = Capsule::from_decks("smoke", seeds, &reg, 40, 14, 20);
        assert!(!cap.pool.is_empty());
        // optimize with the cheap random referee + tiny budget.
        let mk = |s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) };
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let d = random_deck(&cap, &reg, &mut rng);
        assert_eq!(total(&d), 40, "random deck respects deck_size");
        let lt = land_total(&d, &reg);
        assert!((14..=20).contains(&lt), "land band respected: {lt}");
        // mutation preserves size + band
        let m = mutate(&cap, &reg, &d, &mut rng);
        assert_eq!(total(&m), 40);
        // fitness runs and is in range
        let f = fitness(&flatten(&d), &cap, &reg, 1, 2000, 0, &mk);
        assert!((0.0..=1.0).contains(&f));
        // describe doesn't panic
        assert!(describe(&flatten(&d), &reg).contains("cards:"));
    }

    /// EXAMPLE TEMPLATE: run the optimizer on a real Pioneer capsule. Build the
    /// capsule from chosen archetype decklists (see capsule_pioneer below / the
    /// docs/gauntlet-results capsule files), then:
    /// `cargo test -p arcana-ai --release deckbuild_capsule_run -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn deckbuild_capsule_run() {
        use arcana_core::deck::parse_deck_text;
        let base = std::env::var("CAPSULE_DIR").expect("set CAPSULE_DIR (dir of *.txt seed decks)");
        let reg = arcana_cards::build_catalog();
        let mut seeds = Vec::new();
        let mut paths: Vec<_> = std::fs::read_dir(&base).unwrap()
            .filter_map(|e| e.ok()).map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
        paths.sort();
        for p in paths {
            let txt = std::fs::read_to_string(&p).unwrap();
            let parsed = parse_deck_text(&txt, &reg);
            if parsed.unresolved.is_empty() {
                let cards: Vec<CardId> = parsed.main.iter().flat_map(|(c, n)| std::iter::repeat(*c).take(*n as usize)).collect();
                let name = if parsed.name.is_empty() { "seed".into() } else { parsed.name };
                seeds.push(Deck { name, cards });
            }
        }
        println!("capsule field: {} seed decks", seeds.len());
        let cap = Capsule::from_decks("pioneer-capsule", seeds, &reg, 60, 17, 27);
        println!("legal pool: {} distinct cards", cap.pool.len());
        let iters: u32 = std::env::var("CAPSULE_ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
        let games: u32 = std::env::var("CAPSULE_GAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
        let referee = std::env::var("CAPSULE_REFEREE").unwrap_or_default();
        let mk: Box<dyn Fn(u64) -> Box<dyn StatePolicy>> = if referee == "v2" {
            Box::new(|s| crate::deckeval::fixed_policy_v2(s))
        } else {
            Box::new(|s| fixed_policy(s))
        };
        println!("referee: {}", if referee == "v2" { "VmcMaterial-v2" } else { "VmcMaterial" });
        let res = optimize(&cap, &reg, iters, games, 4000, 0, mk.as_ref());
        println!("\nfitness: start {:.3} -> best {:.3} (accepted improvements: {})",
                 res.start_fitness, res.best_fitness, res.history.len() - 1);
        println!("\noptimized deck:\n{}", describe(&res.best, &reg));
    }

    /// Validation arm (reviewer #5): does the optimizer's VMC gain survive a
    /// STRONGER referee? Reproduces the optimized deck (deterministic seed 0),
    /// then scores it AND each seed deck vs the field under both VmcMaterial and
    /// a higher-budget PIMC. If the optimized deck tops the field under VMC but
    /// drops to/below the seeds under PIMC, the VMC gain was a referee exploit.
    /// `cargo test -p arcana-ai --release deckbuild_capsule_validate -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn deckbuild_capsule_validate() {
        use arcana_core::deck::parse_deck_text;
        use arcana_core::state::GameResult;
        use arcana_core::types::PlayerId;
        use crate::search::{play_match, PimcPolicy};

        let base = std::env::var("CAPSULE_DIR").expect("set CAPSULE_DIR (dir of *.txt seed decks)");
        let reg = arcana_cards::build_catalog();
        let mut seeds = Vec::new();
        let mut paths: Vec<_> = std::fs::read_dir(&base).unwrap()
            .filter_map(|e| e.ok()).map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
        paths.sort();
        for p in paths {
            let txt = std::fs::read_to_string(&p).unwrap();
            let parsed = parse_deck_text(&txt, &reg);
            if parsed.unresolved.is_empty() {
                let cards: Vec<CardId> = parsed.main.iter().flat_map(|(c, n)| std::iter::repeat(*c).take(*n as usize)).collect();
                let name = if parsed.name.is_empty() { "seed".into() } else { parsed.name };
                seeds.push(Deck { name, cards });
            }
        }
        // Reproduce the committed optimized deck deterministically (seed 0).
        let cap = Capsule::from_decks("pioneer-capsule", seeds.clone(), &reg, 60, 17, 27);
        let iters: u32 = std::env::var("CAPSULE_ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
        let games: u32 = std::env::var("CAPSULE_GAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
        // Optimize once under EACH referee (deterministic seed 0). v1 reproduces
        // the committed deck; v2 is the rebalanced-leaf optimizer's output.
        let res_v1 = optimize(&cap, &reg, iters, games, 4000, 0, &|s| fixed_policy(s));
        let res_v2 = optimize(&cap, &reg, iters, games, 4000, 0,
                              &|s| crate::deckeval::fixed_policy_v2(s));
        println!("v1-optimized (must match results.txt):\n{}", describe(&res_v1.best, &reg));
        println!("v2-optimized:\n{}", describe(&res_v2.best, &reg));

        // Higher-budget PIMC than the gauntlet screening budget (few decks scored).
        let pimc_samples: u32 = std::env::var("CAPSULE_PIMC_SAMPLES").ok().and_then(|s| s.parse().ok()).unwrap_or(16);
        let pimc_cap: u32 = std::env::var("CAPSULE_PIMC_CAP").ok().and_then(|s| s.parse().ok()).unwrap_or(160);
        let val_games: u32 = std::env::var("CAPSULE_VAL_GAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(4);
        let max_steps = 4000u32;

        // One matchup's point-rate for `cand` vs `opp`, seat-alternating, under
        // referee `r` (0 = VmcMaterial v1, 1 = v2, 2 = PIMC). PIMC policies get the
        // CORRECT physical seat decks each game (determinization).
        let matchup = |cand: &[CardId], opp: &[CardId], r: u8| -> f32 {
            let mut score = 0.0f32;
            for g in 0..val_games {
                let cand_seat = (g % 2) as PlayerId;
                let seat_decks: Vec<Vec<CardId>> = if cand_seat == 0 {
                    vec![cand.to_vec(), opp.to_vec()]
                } else {
                    vec![opp.to_vec(), cand.to_vec()]
                };
                let mk = |s: u64| -> Box<dyn StatePolicy> {
                    match r {
                        1 => crate::deckeval::fixed_policy_v2(s),
                        2 => Box::new(PimcPolicy::with_budget(s, seat_decks.clone(), pimc_samples, pimc_cap, 8)),
                        _ => fixed_policy(s),
                    }
                };
                let mut p0 = mk(g as u64 * 2 + 1);
                let mut p1 = mk(g as u64 * 2 + 2);
                let mut slots: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];
                let gr = play_match(seat_decks.clone(), &reg, g as u64, &mut slots, max_steps);
                score += match gr {
                    GameResult::Win(p) if p == cand_seat => 1.0,
                    GameResult::Win(_) => 0.0,
                    GameResult::Eliminated(p) if p == cand_seat => 0.0,
                    GameResult::Eliminated(_) => 1.0,
                    GameResult::Draw => 0.5,
                };
            }
            score / val_games as f32
        };
        // Mean point-rate vs the field (a deck never plays an identical list).
        let field_pr = |cand: &[CardId], r: u8| -> f32 {
            let (mut tot, mut n) = (0.0f32, 0u32);
            for opp in &seeds {
                if opp.cards == cand { continue; }
                tot += matchup(cand, &opp.cards, r);
                n += 1;
            }
            if n == 0 { 0.0 } else { tot / n as f32 }
        };

        // The question: does referee v2 rank the field more like PIMC than v1 does,
        // and does the v2-OPTIMIZED deck hold up under PIMC (where v1's cratered)?
        println!("\nfield point-rate ({val_games} games/opp), vmc-v1 / vmc-v2 / pimc \
                  (samples {pimc_samples}, cap {pimc_cap}):");
        println!("{:<26} {:>7} {:>7} {:>7}", "deck", "vmc-v1", "vmc-v2", "pimc");
        let row = |label: &str, cards: &[CardId], tag: &str| {
            println!("{:<26} {:>7.3} {:>7.3} {:>7.3}   {}", label,
                     field_pr(cards, 0), field_pr(cards, 1), field_pr(cards, 2), tag);
        };
        row("v1-optimized", &res_v1.best, "<- maxed vmc-v1");
        row("v2-optimized", &res_v2.best, "<- maxed vmc-v2");
        for s in &seeds {
            let name: String = s.name.chars().take(24).collect();
            row(&name, &s.cards, "");
        }
    }

    /// PIMC-SELECT experiment (the "a single cheap target is gameable" fix): a
    /// single cheap referee is exploitable, so generate a POOL of candidate decks
    /// by cheap optimization (both v1 & v2 leaves, several seeds each) and pick the
    /// final deck by a small PIMC tournament vs the field — "PIMC only on optimizer
    /// outputs" (the reviewer's idea). The question: does PIMC-selection find a
    /// candidate meaningfully better under PIMC than the cheap judge's own argmax
    /// (which games the proxy) — or is candidate GENERATION, not selection, the
    /// bottleneck (every cheap-optimized deck is a proxy-corner that PIMC tanks)?
    /// `cargo test -p arcana-ai --release deckbuild_capsule_pimc_select -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn deckbuild_capsule_pimc_select() {
        use arcana_core::deck::parse_deck_text;
        use arcana_core::state::GameResult;
        use arcana_core::types::PlayerId;
        use crate::search::{play_match, PimcPolicy};

        let base = std::env::var("CAPSULE_DIR").expect("set CAPSULE_DIR (dir of *.txt seed decks)");
        let reg = arcana_cards::build_catalog();
        let mut seeds = Vec::new();
        let mut paths: Vec<_> = std::fs::read_dir(&base).unwrap()
            .filter_map(|e| e.ok()).map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
        paths.sort();
        for p in paths {
            let txt = std::fs::read_to_string(&p).unwrap();
            let parsed = parse_deck_text(&txt, &reg);
            if parsed.unresolved.is_empty() {
                let cards: Vec<CardId> = parsed.main.iter().flat_map(|(c, n)| std::iter::repeat(*c).take(*n as usize)).collect();
                let name = if parsed.name.is_empty() { "seed".into() } else { parsed.name };
                seeds.push(Deck { name, cards });
            }
        }
        let cap = Capsule::from_decks("pioneer-capsule", seeds.clone(), &reg, 60, 17, 27);
        let iters: u32 = std::env::var("CAPSULE_ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
        let games: u32 = std::env::var("CAPSULE_GAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(5);
        let cands: u32 = std::env::var("CAPSULE_CANDIDATES").ok().and_then(|s| s.parse().ok()).unwrap_or(3);
        let pimc_samples: u32 = std::env::var("CAPSULE_PIMC_SAMPLES").ok().and_then(|s| s.parse().ok()).unwrap_or(16);
        let pimc_cap: u32 = std::env::var("CAPSULE_PIMC_CAP").ok().and_then(|s| s.parse().ok()).unwrap_or(160);
        let val_games: u32 = std::env::var("CAPSULE_VAL_GAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(6);
        let max_steps = 4000u32;

        // Candidate pool: cheap optimization under BOTH leaves, `cands` seeds each.
        let mut pool: Vec<(String, Vec<CardId>)> = Vec::new();
        for k in 0..cands {
            let r1 = optimize(&cap, &reg, iters, games, max_steps, k as u64, &|s| fixed_policy(s));
            pool.push((format!("v1#{k}"), r1.best));
            let r2 = optimize(&cap, &reg, iters, games, max_steps, k as u64,
                              &|s| crate::deckeval::fixed_policy_v2(s));
            pool.push((format!("v2#{k}"), r2.best));
        }

        // Matchup point-rate for `cand` vs `opp` under referee r (1 = v2 cheap judge,
        // 2 = PIMC), seat-alternating with correct determinization decks.
        let matchup = |cand: &[CardId], opp: &[CardId], r: u8| -> f32 {
            let mut score = 0.0f32;
            for g in 0..val_games {
                let cand_seat = (g % 2) as PlayerId;
                let seat_decks: Vec<Vec<CardId>> = if cand_seat == 0 {
                    vec![cand.to_vec(), opp.to_vec()]
                } else {
                    vec![opp.to_vec(), cand.to_vec()]
                };
                let mk = |s: u64| -> Box<dyn StatePolicy> {
                    if r == 2 {
                        Box::new(PimcPolicy::with_budget(s, seat_decks.clone(), pimc_samples, pimc_cap, 8))
                    } else {
                        crate::deckeval::fixed_policy_v2(s)
                    }
                };
                let mut p0 = mk(g as u64 * 2 + 1);
                let mut p1 = mk(g as u64 * 2 + 2);
                let mut slots: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];
                let gr = play_match(seat_decks.clone(), &reg, g as u64, &mut slots, max_steps);
                score += match gr {
                    GameResult::Win(p) if p == cand_seat => 1.0,
                    GameResult::Win(_) => 0.0,
                    GameResult::Eliminated(p) if p == cand_seat => 0.0,
                    GameResult::Eliminated(_) => 1.0,
                    GameResult::Draw => 0.5,
                };
            }
            score / val_games as f32
        };
        let field_pr = |cand: &[CardId], r: u8| -> f32 {
            let (mut tot, mut n) = (0.0f32, 0u32);
            for opp in &seeds {
                if opp.cards == cand { continue; }
                tot += matchup(cand, &opp.cards, r);
                n += 1;
            }
            if n == 0 { 0.0 } else { tot / n as f32 }
        };

        // Score every candidate under the cheap v2 judge AND under PIMC.
        println!("\ncandidate pool ({} decks): cheap v2-judge vs PIMC ({val_games} games/opp, \
                  pimc {pimc_samples}/{pimc_cap})", pool.len());
        println!("{:<8} {:>8} {:>8}", "cand", "v2", "pimc");
        let mut scored: Vec<(String, f32, f32, usize)> = Vec::new();
        for (i, (label, deck)) in pool.iter().enumerate() {
            let cheap = field_pr(deck, 1);
            let strong = field_pr(deck, 2);
            println!("{:<8} {:>8.3} {:>8.3}", label, cheap, strong);
            scored.push((label.clone(), cheap, strong, i));
        }
        let cheap_pick = scored.iter().cloned()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        let pimc_pick = scored.iter().cloned()
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        println!("\ncheap(v2)-argmax : {} (v2 {:.3}, PIMC {:.3})", cheap_pick.0, cheap_pick.1, cheap_pick.2);
        println!("PIMC-argmax      : {} (v2 {:.3}, PIMC {:.3})", pimc_pick.0, pimc_pick.1, pimc_pick.2);
        println!("value of PIMC-select (PIMC-pick − cheap-pick, under PIMC): {:.3}",
                 pimc_pick.2 - cheap_pick.2);
        println!("(field baseline: best seed under PIMC was RDW ≈0.75 in the committed A/B run.)");
        println!("\nPIMC-selected deck:\n{}", describe(&pool[pimc_pick.3].1, &reg));
    }
}
