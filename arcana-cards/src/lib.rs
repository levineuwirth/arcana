//! Card catalog for the Arcana engine.
//!
//! This crate is a thin catalog: every card is a `pub fn register(reg:
//! &mut CardRegistry) -> CardId` that constructs and registers a
//! [`arcana_core::registry::CardDefinition`]. The engine lives entirely
//! in [`arcana_core`]; this crate owns no types of its own.
//!
//! # Organization
//!
//! Modules are named by **set code** (Scryfall's three-letter codes:
//! `lea` = Limited Edition Alpha, `lrw` = Lorwyn, etc.). Each card
//! lives in its **canonical-printing** set — the earliest set that
//! printed the card, matching Scryfall's default scheme. Reprints do
//! not duplicate; `arcana-gen` will emit one module per canonical
//! printing and handle reprints via card-id aliasing.
//!
//! Example: Lightning Bolt was printed in LEA, so it lives at
//! [`lea::lightning_bolt`]. When Bolt reprints in M11, M12, or any
//! other set, the Scryfall id is aliased to the same `CardId` the
//! LEA module registered.
//!
//! # Why set-code
//!
//! `arcana-gen` consumes Scryfall bulk data which is set-tagged; the
//! generator writes one module per card keyed on canonical set. Flat
//! organization (`arcana-cards/src/lightning_bolt.rs`) would be
//! simpler for a hand-written catalog but would diverge from the
//! generator's output shape, creating friction at the hand-generated
//! boundary. Function-based organization (`burn/`, `removal/`) is
//! fuzzy — Cryptic Command is a counterspell *and* a bounce spell
//! *and* a tap spell — so it's rejected.

pub mod lea;
pub mod lrw;
pub mod isd;
pub mod po2;
pub mod aer;
pub mod mh2;
pub mod rav;
pub mod m11;
pub mod hou;
pub mod m15;
pub mod zen;
pub mod ons;
pub mod tor;
pub mod eld;
pub mod znr;
pub mod apc;
pub mod ktk;
pub mod akh;
pub mod eve;
pub mod m14;
pub mod mrd;

pub mod fdn;

pub mod m13;

pub mod s10e;

pub mod xln;

pub mod mmq;

pub mod gs1;

pub mod rix;

pub mod bok;

pub mod rna;

pub mod por;

pub mod m20;

pub mod dgm;

pub mod ice;

pub mod m12;

pub mod s7ed;

pub mod ptk;

pub mod jmp;

pub mod tsb;

pub mod war;

pub mod me4;

pub mod thb;

pub mod bbd;

pub mod tpr;

pub mod s9ed;

pub mod me3;

pub mod dft;

pub mod s99;

pub mod bng;

pub mod dtk;

pub mod tdc;

pub mod nph;

pub mod soi;

pub mod tsp;

pub mod tle;

pub mod m10;

pub mod ths;

pub mod leg;

pub mod som;

pub mod s8ed;

pub mod s00;

pub mod frf;

pub mod grn;

pub mod mir;

pub mod shm;

pub mod ala;

pub mod ust;

pub mod arb;

pub mod mm3;

pub mod s6ed;

pub mod w17;

pub mod chr;

pub mod cns;

pub mod p02;

pub mod oana;

pub mod m21;

pub mod mom;

pub mod m19;

pub mod vis;

pub mod gtc;

pub mod cmm;

pub mod kld;

pub mod stx;

pub mod ori;

pub mod cn2;

pub mod roe;

pub mod gpt;

pub mod a25;

pub mod iko;

pub mod dom;

pub mod tsr;

pub mod jou;

pub mod wwk;

pub mod rtr;

pub mod ema;

pub mod dka;

pub mod s5ed;

pub mod ogw;

pub mod con;

pub mod j25;

pub mod mbs;

pub mod ody;

pub mod fut;

pub mod chk;

pub mod csp;

pub mod me1;

pub mod me2;

pub mod bfz;

pub mod td2;

pub mod avr;

pub mod hml;

pub mod ddp;

pub mod plst;

pub mod drk;

pub mod ddr;

pub mod khm;

pub mod dmc;

pub mod usg;

pub mod evg;

pub mod s4ed;

pub mod s2xm;

pub mod pcy;

pub mod uds;

pub mod tmt;

pub mod clb;

pub mod wth;

pub mod dmu;

pub mod dis;

pub mod tmp;

pub mod hbg;

pub mod ddn;

pub mod afr;

pub mod s5dn;

pub mod mm2;

pub mod tdm;

pub mod blb;

pub mod gn2;

pub mod sok;

pub mod clu;

pub mod scg;

pub mod lci;

pub mod bro;

pub mod hop;

pub mod all;

pub mod ima;

pub mod emn;

pub mod rvr;

pub mod dmr;

pub mod one;

pub mod eoe;

pub mod nem;

pub mod cmr;

pub mod dst;

pub mod sos;

pub mod ddq;

pub mod ddg;

pub mod inr;

pub mod ddi;

pub mod ddh;

pub mod c13;

pub mod lgn;

pub mod s40k;

pub mod c20;

pub mod jud;

pub mod uma;

pub mod ddo;

pub mod plc;

pub mod mh1;

pub mod ddk;

pub mod gvl;

pub mod neo;

pub mod pca;

pub mod mma;

pub mod arn;

pub mod snc;

pub mod dds;

pub mod ddl;

pub mod mor;

pub mod unk;

pub mod ddt;

pub mod prm;

pub mod e01;

pub mod fin;

pub mod dvd;

pub mod gnt;

pub mod otj;

pub mod pls;

pub mod cmb2;

pub mod s2x2;

pub mod anb;

pub mod inv;

pub mod ltr;

pub mod ecc;

pub mod tmc;

pub mod ecl;

pub mod mkm;

pub mod unf;

pub mod vow;

pub mod soc;

pub mod woe;

pub mod ydsk;

pub mod dsk;

pub mod mid;

pub mod mkc;

pub mod otc;

pub mod und;

pub mod pip;

pub mod who;

pub mod spm;

pub mod ulg;

pub mod mat;

pub mod mh3;

pub mod ylci;

pub mod ymid;

pub mod unh;

pub mod tla;

pub mod moc;

pub mod nec;

pub mod vma;

pub mod eoc;

pub mod c19;

pub mod cmd;

pub mod dsc;

pub mod acr;

pub mod cm2;

pub mod arc;

pub mod ugl;

pub mod mb2;


pub mod ddm;

pub mod ydmu;

pub mod dde;

pub mod cma;

pub mod rex;

pub mod mic;

pub mod lcc;

pub mod yneo;

pub mod j21;

pub mod c17;

pub mod ddj;

pub mod c21;

pub mod ybro;


pub mod c15;

pub mod c14;

pub mod c18;

pub mod ysnc;

pub mod c16;

pub mod j22;

pub mod yblb;

pub mod yecl;

pub mod ncc;

pub mod jvc;

pub mod spe;

pub mod khc;

pub mod voc;

pub mod fic;

pub mod afc;

pub mod ddu;

pub mod yotj;

pub mod w16;

pub mod sth;

pub mod ydft;

pub mod register_all;

pub mod ltc;

pub mod exo;

pub mod msc;

pub mod hob;

pub mod znc;

pub mod drc;

pub mod ddf;

pub mod brc;

pub mod m3c;

pub mod ymkm;

pub mod fem;

pub mod ywoe;

pub mod ytdm;

pub mod ph17;

pub mod atq;

pub mod gn3;

pub mod sum;

pub mod psdg;

pub mod ph19;

pub mod msh;

pub mod blc;

pub mod bot;

pub mod hho;

pub mod h17;

pub mod ph18;

pub mod slx;

pub mod ptg;

pub mod tfth;

pub mod tdag;

pub mod woc;

pub mod cc2;

pub mod sld;

pub mod thp3;

pub mod big;

pub mod ph20;

pub mod pf25;

pub mod s2ed;

pub mod yeoe;

pub mod ph21;

pub mod ph23;

pub mod ph22;

pub mod phtr;

pub mod ppc1;

pub mod fra;

pub mod pcel;

/// Staging area for arcana-gen card generations. See the module
/// docs — this is intermediate storage, not a stable public API.
pub mod generated;

use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;

/// The Tier 1–3 seed set. Tier 1 = five basic lands + Lightning Bolt
/// + Grizzly Bears (mana, combat, targeted instant). Tier 2 adds
/// Counterspell (stack targeting), Murder (destroy), Elvish
/// Visionary (ETB-draw trigger), Glorious Anthem (layer-7c static),
/// Disintegrate (X-cost damage). Tier 3 adds Walking Ballista
/// (X-in-P/T via `EntersWithSpec::CountersFromX` + counter-removal-
/// as-activation-cost). The keyword-stress pack adds Serra Angel
/// (Flying + Vigilance), Giant Spider (Reach), and Typhoid Rats
/// (Deathtouch) so the already-wired evergreen combat keywords get
/// exercised via real seed cards rather than only synthetic combat
/// tests. Abrade (modal), Chandra, Pyromaster (loyalty), and Burst
/// Lightning (Kicker) anchor the Phase 2 mechanics each in a real
/// printed card. Preordain (Scry 2, then draw a card) anchors
/// sequential multi-effect resolution — the engine must park the
/// draw while the scry's `OrderCards` prompt is open and resume it
/// when the placements come in. `CardId`s returned for test
/// convenience.
#[derive(Clone, Copy, Debug)]
pub struct SeedIds {
    pub plains: CardId,
    pub island: CardId,
    pub swamp: CardId,
    pub mountain: CardId,
    pub forest: CardId,
    pub grizzly_bears: CardId,
    pub lightning_bolt: CardId,
    pub counterspell: CardId,
    pub murder: CardId,
    pub elvish_visionary: CardId,
    pub glorious_anthem: CardId,
    pub disintegrate: CardId,
    pub walking_ballista: CardId,
    pub snapcaster_mage: CardId,
    pub murktide_regent: CardId,
    pub chord_of_calling: CardId,
    pub serra_angel: CardId,
    pub giant_spider: CardId,
    pub typhoid_rats: CardId,
    pub abrade: CardId,
    pub chandra_pyromaster: CardId,
    pub burst_lightning: CardId,
    pub tranquil_thicket: CardId,
    pub fiery_temper: CardId,
    pub bonecrusher_giant: CardId,
    pub tangled_florahedron: CardId,
    pub fire_ice: CardId,
    pub monastery_swiftspear: CardId,
    pub ahn_crop_crasher: CardId,
    pub slippery_bogle: CardId,
    pub servo_exhibition: CardId,
    pub young_pyromancer: CardId,
    pub bonesplitter: CardId,
    pub preordain: CardId,
}

/// Register every seed card. Convenience for tests and tooling;
/// production code can register selectively per set/module.
pub fn register_seed(reg: &mut CardRegistry) -> SeedIds {
    SeedIds {
        plains: lea::plains::register(reg),
        island: lea::island::register(reg),
        swamp: lea::swamp::register(reg),
        mountain: lea::mountain::register(reg),
        forest: lea::forest::register(reg),
        grizzly_bears: lea::grizzly_bears::register(reg),
        lightning_bolt: lea::lightning_bolt::register(reg),
        counterspell: lea::counterspell::register(reg),
        murder: isd::murder::register(reg),
        elvish_visionary: lrw::elvish_visionary::register(reg),
        glorious_anthem: po2::glorious_anthem::register(reg),
        disintegrate: lea::disintegrate::register(reg),
        walking_ballista: aer::walking_ballista::register(reg),
        snapcaster_mage: isd::snapcaster_mage::register(reg),
        murktide_regent: mh2::murktide_regent::register(reg),
        chord_of_calling: rav::chord_of_calling::register(reg),
        serra_angel: lea::serra_angel::register(reg),
        giant_spider: lea::giant_spider::register(reg),
        typhoid_rats: m11::typhoid_rats::register(reg),
        abrade: hou::abrade::register(reg),
        chandra_pyromaster: m15::chandra_pyromaster::register(reg),
        burst_lightning: zen::burst_lightning::register(reg),
        tranquil_thicket: ons::tranquil_thicket::register(reg),
        fiery_temper: tor::fiery_temper::register(reg),
        bonecrusher_giant: eld::bonecrusher_giant::register(reg),
        tangled_florahedron: znr::tangled_florahedron::register(reg),
        fire_ice: apc::fire_ice::register(reg),
        monastery_swiftspear: ktk::monastery_swiftspear::register(reg),
        ahn_crop_crasher: akh::ahn_crop_crasher::register(reg),
        slippery_bogle: eve::slippery_bogle::register(reg),
        servo_exhibition: aer::servo_exhibition::register(reg),
        young_pyromancer: m14::young_pyromancer::register(reg),
        bonesplitter: mrd::bonesplitter::register(reg),
        preordain: m11::preordain::register(reg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_seed_produces_distinct_card_ids() {
        let mut reg = CardRegistry::new();
        let ids = register_seed(&mut reg);
        let as_slice = [
            ids.plains, ids.island, ids.swamp, ids.mountain, ids.forest,
            ids.grizzly_bears, ids.lightning_bolt,
            ids.counterspell, ids.murder, ids.elvish_visionary,
            ids.glorious_anthem, ids.disintegrate, ids.walking_ballista,
            ids.snapcaster_mage, ids.murktide_regent, ids.chord_of_calling,
            ids.serra_angel, ids.giant_spider, ids.typhoid_rats,
            ids.abrade, ids.chandra_pyromaster, ids.burst_lightning,
            ids.tranquil_thicket, ids.fiery_temper, ids.bonecrusher_giant,
            ids.tangled_florahedron, ids.fire_ice, ids.monastery_swiftspear,
            ids.ahn_crop_crasher, ids.slippery_bogle,
            ids.servo_exhibition, ids.young_pyromancer,
            ids.bonesplitter, ids.preordain,
        ];
        let unique: std::collections::HashSet<_> = as_slice.iter().collect();
        assert_eq!(unique.len(), as_slice.len(),
            "every card in the seed set must get a distinct CardId");
    }

    #[test]
    fn every_basic_land_has_one_mana_ability() {
        let mut reg = CardRegistry::new();
        let ids = register_seed(&mut reg);
        for id in [ids.plains, ids.island, ids.swamp, ids.mountain, ids.forest] {
            let def = reg.get(id).unwrap();
            assert!(def.base_characteristics.types.is_land(),
                "basic must be land");
            assert!(def.base_characteristics.supertypes.is_basic(),
                "basic must have Basic supertype");
            assert_eq!(def.activated_abilities.len(), 1,
                "basic must have exactly one activated ability");
            assert!(def.activated_abilities[0].is_mana_ability,
                "basic's ability must be a mana ability");
        }
    }

    #[test]
    fn lightning_bolt_has_any_target_requirement() {
        let mut reg = CardRegistry::new();
        let id = lea::lightning_bolt::register(&mut reg);
        let def = reg.get(id).unwrap();
        let sa = def.spell_ability.as_ref().expect("Bolt has a spell ability");
        assert_eq!(sa.target_requirements.len(), 1);
    }

    #[test]
    fn grizzly_bears_is_2_2_green_creature() {
        use arcana_core::types::PtValue;
        let mut reg = CardRegistry::new();
        let id = lea::grizzly_bears::register(&mut reg);
        let def = reg.get(id).unwrap();
        assert!(def.base_characteristics.types.is_creature());
        assert_eq!(def.base_characteristics.power, Some(PtValue::Fixed(2)));
        assert_eq!(def.base_characteristics.toughness, Some(PtValue::Fixed(2)));
        assert!(def.base_characteristics.colors.contains(arcana_core::types::Color::Green));
    }
    #[test]
    fn register_all_builds_the_whole_catalog() {
        // C1 standing attestation: every catalog card's `register`
        // runs, the registry builds, and the duplicate-name invariant
        // holds across the entire catalog (not just the 35-card seed).
        let mut reg = arcana_core::registry::CardRegistry::new();
        let n = crate::register_all::register_all(&mut reg);
        assert!(n >= 3000, "expected the full catalog, got {n}");
        assert_eq!(reg.len(), n,
            "every register() must yield a distinct CardId —              reg.len()={} != calls={n}", reg.len());
    }

    /// Integration stress test: play full random games to completion,
    /// asserting no panic, no stuck decision (empty legal-action set),
    /// and termination within a step cap. Exercises the turn/combat/
    /// stack/priority/SBA machinery and multi-card interactions that the
    /// isolated behavioral probe never touches. `#[ignore]` (slow); run:
    /// `cargo test -p arcana-cards random_games -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn random_games_play_to_completion() {
        // 400 games (200 uniform + 200 biased) exercises every KNOWN_OPEN
        // seed in ~20s. Running with a larger GAMES is a valid deeper sweep
        // and may surface NEW findings beyond the recorded baseline.
        const GAMES: u64 = 500;
        const STEP_CAP: usize = 8000;
        // Safety guards — a fuzz harness must never be able to OOM the
        // machine. Biased aggressive play can reach large boards where the
        // engine's combat-enumeration Cartesian products (a documented
        // legal_actions DEBT) blow up. If the live object count or a single
        // legal-action set crosses these caps, abort the game and record it
        // as a *bounded* outcome (not a test failure) so the cliff is
        // surfaced without crashing the run.
        // Total live objects (token-loop backstop).
        const OBJ_CAP: usize = 4000;
        // Battlefield creatures across both players. The combat-enumeration
        // explosion this once guarded (2^blockers, k! orderings) is now
        // bounded engine-side by MAX_COMBAT_ENUM, and target-selection by
        // MAX_TARGET_SELECTIONS, so this is no longer an OOM gate — it just
        // bounds per-game complexity / step time. Kept generous so bigger
        // boards (and the deeper combat they drive) get exercised.
        const CREATURE_CAP: usize = 64;
        const LEGAL_CAP: usize = 200_000;
        // Stack depth. No honest game stacks hundreds of objects; a runaway
        // stack means a free, repeatable, instant-speed ability is being
        // spammed (e.g. a card whose activation cost is unmodeled — Cephalid
        // Inkshrouder's "Discard a card:" is registered as cost-free, so the
        // biased picker stacks it thousands deep and the phase never empties
        // the stack to advance). Abort as a bounded outcome rather than
        // livelock; the underlying fix is modeling the missing cost.
        const STACK_CAP: usize = 500;

        // Seeds known to hit still-open engine bugs that biased combat
        // play surfaces — each pinned to its panic site. A failure on a
        // listed seed whose detail still contains the recorded site is
        // expected (logged, not fatal); a failure anywhere else, or a
        // listed seed failing for a *different* reason, is a NEW finding
        // and fails the test. Remove an entry when its bug is fixed.
        const KNOWN_OPEN: &[(u64, &str)] = &[];

        // Capture the panic *site* per game (and suppress the default
        // backtrace spam) so failures are actionable.
        thread_local! {
            static PANIC_LOC: std::cell::RefCell<Option<String>> =
                const { std::cell::RefCell::new(None) };
        }
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|info| {
            PANIC_LOC.with(|l| *l.borrow_mut() =
                info.location().map(|loc| format!("{}:{}", loc.file(), loc.line())));
        }));

        let mut reg = arcana_core::registry::CardRegistry::new();
        let n = crate::register_all::register_all(&mut reg) as u32;
        // CardIds are a sparse HashMap key space — sample only VALID ids.
        let valid: Vec<u32> = (0..n).filter(|&c| reg.get(c).is_some()).collect();
        // Basic-land CardIds (so mana flows and casting/combat happen).
        let basics: Vec<u32> = ["Plains","Island","Swamp","Mountain","Forest"].iter()
            .filter_map(|name| reg.interner().lookup(name).and_then(|sym|
                valid.iter().copied().find(|&c| reg.get(c).map(|d| d.name) == Some(sym))))
            .collect();
        assert!(basics.len() == 5, "expected all 5 basic lands, got {}", basics.len());

        // Deterministic LCG so a failure is replayable from its seed.
        struct Lcg(u64);
        impl Lcg { fn next(&mut self, m: usize) -> usize {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 33) as usize) % m.max(1)
        }}

        // Biased action selection. Pure-random play almost always passes
        // priority and declares "no attacks" (the empty declaration sits
        // at index 0 alongside one action per attacker), so games rarely
        // reach combat or low-life states. Weighting by action kind drives
        // play toward casting / attacking / blocking — exercising the
        // combat/damage/SBA machinery and the `life <= 0` invariant arm
        // that uniform play barely touches. Concede is weighted to zero so
        // games play out; mulligans stay reachable (the deep-mulligan path
        // that found the bottom-cards bug) but biased toward keeping.
        fn action_weight(a: &arcana_core::actions::Action) -> u32 {
            use arcana_core::actions::Action::*;
            match a {
                PassPriority => 1,
                CastSpell { .. } => 8,
                PlayLand { .. } => 8,
                ActivateAbility { .. } => 3,
                DeclareAttackers { attackers } =>
                    if attackers.is_empty() { 1 } else { 10 },
                DeclareBlockers { blockers } =>
                    if blockers.is_empty() { 1 } else { 8 },
                OrderBlockers { .. } | AssignCombatDamage { .. } => 4,
                MakeChoice(_) | SubmitResolutionChoice { .. } => 4,
                MulliganKeep => 4,
                MulliganAgain => 2,
                BottomCards(_) => 4,
                Concede => 0,
            }
        }

        // Pick an action index. `bias=false` is uniform random (kept for
        // half the games — it found the deep-mulligan bug). `bias=true`
        // is weighted by `action_weight`; a zero total (only zero-weight
        // actions legal) falls back to uniform.
        fn pick(rng: &mut Lcg, actions: &[arcana_core::actions::Action],
                bias: bool) -> usize {
            if !bias { return rng.next(actions.len()); }
            let weights: Vec<u32> = actions.iter().map(action_weight).collect();
            let total: u32 = weights.iter().sum();
            if total == 0 { return rng.next(actions.len()); }
            let mut r = rng.next(total as usize) as u32;
            for (i, w) in weights.iter().enumerate() {
                if r < *w { return i; }
                r -= *w;
            }
            actions.len() - 1
        }

        // Mid-game state invariants, checked at every decision point.
        // Returns Err with a human-readable reason on the first breach.
        fn game_invariants(
            state: &arcana_core::state::GameState,
            context: &arcana_core::actions::DecisionContext,
        ) -> Result<(), String> {
            use arcana_core::actions::DecisionContext;

            // (1) Context-INDEPENDENT invariants — valid at any decision.
            // Object identity: every live arena object has a unique id. A
            // duplicate means a re-id/clone bug (the London-bottom re-id
            // path could have introduced one).
            let mut seen = std::collections::HashSet::new();
            for o in state.objects.iter() {
                if !seen.insert(o.id) {
                    return Err(format!("duplicate object id {} in arena", o.id));
                }
            }
            // Runaway-mana ceiling. Floating mana empties at end of step
            // (CR 500.4) but legitimately floats WITHIN a step, so we don't
            // assert emptiness — only a loose ceiling that would catch a
            // mana-doubling loop (no honest game floats this much).
            for p in 0..state.num_players() {
                if state.player(p).mana_pool.total() > 10_000 {
                    return Err(format!(
                        "runaway mana: player {p} pool has {} units",
                        state.player(p).mana_pool.total()));
                }
            }

            // (2) State-based-action leaks. SBAs are only GUARANTEED applied
            // when a player would receive priority (CR 704.4). At casting
            // sub-steps, combat declarations, and mid-resolution choices a
            // loss/death condition can hold TRANSIENTLY before the action
            // finishes and SBAs run (e.g. a "draw 3" that decks a player
            // then prompts a choice). So only assert at a Priority decision
            // point — a genuine leak persists to the next priority check,
            // so nothing real is missed.
            if !matches!(context, DecisionContext::Priority) {
                return Ok(());
            }

            // Ask the engine's OWN aggregate predicate whether any SBA still
            // applies. Using the engine's predicate (not a hand-rolled copy)
            // means the check can never drift from what the SBA pass actually
            // does — the divergence trap an earlier raw-toughness version of
            // this fell into. Covers every implemented SBA at once: player
            // loss (704.5a/b/c), creature/PW death (704.5f/g/i), legend
            // (704.5j), ±1/±1 (704.5p), tokens (704.5d), equipment /
            // fortification / aura (704.5q/r/n), saga (704.5s), battle
            // (704.5t).
            if let Some(kind) = arcana_core::sba::pending_state_based_action_kind(state) {
                return Err(format!(
                    "SBA leak: {kind} still applies at a priority decision"));
            }
            // In a 2-player game a loss ends the game, so a player still
            // flagged `has_lost` at a pending (non-over) decision means
            // game-over detection lagged behind the SBA. (The aggregate
            // above won't flag an already-lost player — its loss predicate
            // is `!has_lost && ...` — so check this separately.)
            for p in 0..state.num_players() {
                if state.player(p).has_lost {
                    return Err(format!(
                        "player {p} has_lost but game still pending"));
                }
            }
            Ok(())
        }

        // Per-mode telemetry, indexed [uniform, biased]:
        //   games, min life seen, attacks, max objects, max legal-set, aborts.
        #[derive(Clone, Copy)]
        struct ModeAgg { games: u64, min_life: i32, attacks: u64,
                         max_obj: usize, max_legal: usize, aborts: u64 }

        // One game, fully self-contained and deterministic from its seed —
        // independent RNG, decks, and engine state — so games parallelize.
        // Returns a telemetry record or a failure (panic site / stuck /
        // non-termination / invariant breach). The per-game report is
        // (min life, attacks, max objects, max legal-set, aborted-by-cap?);
        // an abort is bounded, NOT a failure.
        enum GameResult {
            Done { profile: usize, bias: bool, min_life: i32, attacks: u64,
                   max_obj: usize, max_legal: usize, aborted: bool },
            Failed { profile: usize, seed: u64, detail: String },
        }
        fn run_game(
            reg: &arcana_core::registry::CardRegistry,
            valid: &[u32], basics: &[u32], archetype: Option<&[u32]>,
            profile: usize, seed: u64,
        ) -> GameResult {
            use arcana_core::engine::{new_game, step, EngineYield};
            let mut rng = Lcg(seed.wrapping_mul(2654435761).wrapping_add(1));
            // 40-card deck: 18 basics + 22 spells. A curated archetype draws
            // ~half its spells from the archetype pool and the rest from the
            // whole catalog (so the deck still functions — creatures to
            // attack, removal, etc.); the random profile draws all 22 from
            // the catalog (identical to before, preserving determinism).
            let deck = |rng: &mut Lcg| -> Vec<u32> {
                let mut d = Vec::with_capacity(40);
                for _ in 0..18 { d.push(basics[rng.next(5)]); }
                match archetype {
                    Some(pool) if !pool.is_empty() => {
                        for _ in 0..11 { d.push(pool[rng.next(pool.len())]); }
                        for _ in 0..11 { d.push(valid[rng.next(valid.len())]); }
                    }
                    _ => for _ in 0..22 { d.push(valid[rng.next(valid.len())]); }
                }
                d
            };
            let decks = vec![deck(&mut rng), deck(&mut rng)];
            // Even seeds play uniform-random; odd seeds use the biased
            // picker. Splitting keeps both coverage profiles.
            let bias = seed % 2 == 1;
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let (mut state, mut yld) = new_game(decks, reg, seed);
                // Driver-side liveness: a random agent will spin forever on
                // EV-neutral repeatable activations (Mana Screw's "{1}: flip
                // a coin, win adds {C}{C}" is a recurrent random walk — legal
                // Magic, but a human stops). Cap identical activations per
                // turn and filter capped ones out of the pick; this is agent
                // policy, not an engine-semantics change.
                let mut spam: std::collections::HashMap<(arcana_core::objects::ObjectId, usize), u32> =
                    Default::default();
                let mut spam_turn: u32 = 0;
                const SPAM_CAP: u32 = 30;
                let mut min_life = i32::MAX;
                let mut attacks: u64 = 0;
                let mut max_obj = 0usize;
                let mut max_legal = 0usize;
                for _ in 0..STEP_CAP {
                    match yld {
                        EngineYield::GameOver(_) =>
                            return Ok((min_life, attacks, max_obj, max_legal, false)),
                        EngineYield::PendingDecision { legal_actions, ref context, .. } => {
                            // Mid-game invariants. SBA-leak checks are gated
                            // to Priority contexts inside (CR 704.4); object-
                            // identity / mana checks run at every decision.
                            // A violation is an engine bug the panic / stuck /
                            // terminate checks would miss.
                            game_invariants(&state, context)?;
                            for p in 0..state.num_players() {
                                min_life = min_life.min(state.player(p).life);
                            }
                            // Safety caps — abort (not fail) before the
                            // board or an enumeration grows large enough to
                            // OOM the process.
                            let obj = state.objects.iter().count();
                            let creatures = state.objects
                                .objects_in_zone(arcana_core::zones::Zone::Battlefield)
                                .filter(|o| o.characteristics.types.is_creature())
                                .count();
                            max_obj = max_obj.max(obj);
                            max_legal = max_legal.max(legal_actions.len());
                            if obj > OBJ_CAP || creatures > CREATURE_CAP
                                || legal_actions.len() > LEGAL_CAP
                                || state.stack_entries().len() > STACK_CAP {
                                return Ok((min_life, attacks, max_obj, max_legal, true));
                            }
                            if legal_actions.is_empty() {
                                return Err("stuck: no legal actions".to_string());
                            }
                            if state.turn.turn_number != spam_turn {
                                spam_turn = state.turn.turn_number;
                                spam.clear();
                            }
                            let unspammy: Vec<arcana_core::actions::Action> = legal_actions
                                .iter()
                                .filter(|a| match a {
                                    arcana_core::actions::Action::ActivateAbility {
                                        source, ability_index, ..
                                    } => spam.get(&(*source, *ability_index))
                                        .is_none_or(|n| *n < SPAM_CAP),
                                    _ => true,
                                })
                                .cloned()
                                .collect();
                            let pool = if unspammy.is_empty() { &legal_actions } else { &unspammy };
                            let i = pick(&mut rng, pool, bias);
                            let action = pool[i].clone();
                            if let arcana_core::actions::Action::ActivateAbility {
                                source, ability_index, ..
                            } = &action {
                                *spam.entry((*source, *ability_index)).or_insert(0) += 1;
                            }
                            if matches!(&action,
                                arcana_core::actions::Action::DeclareAttackers { attackers }
                                if !attackers.is_empty()) {
                                attacks += 1;
                            }
                            let (s, y) = step(state, action, reg);
                            state = s; yld = y;
                        }
                    }
                }
                Err(format!("did not terminate in {STEP_CAP} steps (turn {}, life {:?})",
                    state.turn.turn_number,
                    (0..state.num_players()).map(|p| state.player(p).life).collect::<Vec<_>>()))
            }));
            match res {
                Ok(Ok((min_life, attacks, max_obj, max_legal, aborted))) =>
                    GameResult::Done { profile, bias, min_life, attacks, max_obj, max_legal, aborted },
                Ok(Err(detail)) => GameResult::Failed { profile, seed, detail },
                Err(_) => {
                    let loc = PANIC_LOC.with(|l| l.borrow_mut().take())
                        .unwrap_or_else(|| "?".to_string());
                    GameResult::Failed { profile, seed, detail: format!("PANIC at {loc}") }
                }
            }
        }

        // Build curated archetype pools (subsets of `valid` by card type) so
        // games drive subsystems that random 22-card decks rarely assemble —
        // and into the SBA coverage just added (saga 704.5s, battle 704.5t,
        // equipment/aura 704.5q/r/n, planeswalker loyalty 704.5i). An empty
        // pool (archetype absent from the catalog) is skipped.
        fn build_pool<F>(reg: &arcana_core::registry::CardRegistry,
                         valid: &[u32], pred: F) -> Vec<u32>
        where F: Fn(&arcana_core::registry::CardDefinition,
                    &arcana_core::types::StringInterner) -> bool {
            valid.iter().copied()
                .filter(|&c| reg.get(c).is_some_and(|d| pred(d, reg.interner())))
                .collect()
        }
        let candidate_profiles: [(&str, Vec<u32>); 5] = [
            ("saga", build_pool(&reg, &valid, |d, i|
                d.base_characteristics.types.is_enchantment()
                && d.base_characteristics.subtypes.contains_name(i, "Saga"))),
            ("planeswalker", build_pool(&reg, &valid, |d, _|
                d.base_characteristics.types.is_planeswalker())),
            ("battle", build_pool(&reg, &valid, |d, _|
                d.base_characteristics.types.is_battle())),
            ("equipment", build_pool(&reg, &valid, |d, i|
                d.base_characteristics.types.is_artifact()
                && d.base_characteristics.subtypes.contains_name(i, "Equipment"))),
            ("aura", build_pool(&reg, &valid, |d, i|
                d.base_characteristics.types.is_enchantment()
                && d.base_characteristics.subtypes.contains_name(i, "Aura"))),
        ];
        // profiles[0] is the random profile (no archetype pool); the rest are
        // non-empty curated archetypes.
        let mut profiles: Vec<(&str, Option<Vec<u32>>)> = vec![("random", None)];
        for (label, pool) in candidate_profiles {
            if pool.is_empty() {
                eprintln!("curated: no '{label}' cards in catalog — skipping");
            } else {
                eprintln!("curated: '{label}' pool = {} cards", pool.len());
                profiles.push((label, Some(pool)));
            }
        }

        // Work list: (profile index, seed). The random profile gets seeds
        // 0..GAMES (identical decks to before — preserves every regression
        // seed). Each curated archetype gets CURATED_PER games on a disjoint
        // high seed range so curated games never collide with random ones and
        // stay deterministic.
        const CURATED_PER: u64 = 150;
        const CURATED_SEED_BASE: u64 = 1_000_000;
        let mut work: Vec<(usize, u64)> = (0..GAMES).map(|s| (0usize, s)).collect();
        for pidx in 1..profiles.len() {
            for i in 0..CURATED_PER {
                work.push((pidx, CURATED_SEED_BASE + (pidx as u64) * CURATED_PER + i));
            }
        }
        let total = work.len();

        // Fan out games across the available cores. Each game only READS the
        // shared registry (resolution never mutates it — `step` takes `&reg`),
        // so `&reg` (and the profile pools) are shared by reference across
        // `std::thread::scope` workers; per-game RNG / decks / engine state
        // are independent. Work items are strided across threads; results
        // merge commutatively (min / sum / max), so thread scheduling never
        // changes the outcome.
        let nthreads = std::thread::available_parallelism()
            .map(|n| n.get()).unwrap_or(4).min(total.max(1)).max(1);
        let reg_ref = &reg;
        let (valid_ref, basics_ref) = (&valid[..], &basics[..]);
        let work_ref = &work[..];
        let profiles_ref = &profiles;
        let batches: Vec<Vec<GameResult>> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..nthreads).map(|t| {
                scope.spawn(move || {
                    let mut out = Vec::new();
                    let mut wi = t;
                    while wi < total {
                        let (pidx, seed) = work_ref[wi];
                        let arch = profiles_ref[pidx].1.as_deref();
                        out.push(run_game(reg_ref, valid_ref, basics_ref, arch, pidx, seed));
                        wi += nthreads;
                    }
                    out
                })
            }).collect();
            handles.into_iter()
                .map(|h| h.join().expect("harness game thread panicked outside catch_unwind"))
                .collect()
        });
        std::panic::set_hook(prev_hook);

        // Merge per-thread results, keyed by [profile][bias]. Failures sorted
        // by (profile, seed) for deterministic output regardless of scheduling.
        let mut failures: Vec<(usize, u64, String)> = Vec::new();
        let blank = ModeAgg { games: 0, min_life: i32::MAX, attacks: 0,
                              max_obj: 0, max_legal: 0, aborts: 0 };
        let mut agg: Vec<[ModeAgg; 2]> = vec![[blank; 2]; profiles.len()];
        for batch in batches {
            for r in batch {
                match r {
                    GameResult::Done { profile, bias, min_life, attacks, max_obj, max_legal, aborted } => {
                        let a = &mut agg[profile][bias as usize];
                        a.games += 1;
                        a.min_life = a.min_life.min(min_life);
                        a.attacks += attacks;
                        a.max_obj = a.max_obj.max(max_obj);
                        a.max_legal = a.max_legal.max(max_legal);
                        if aborted { a.aborts += 1; }
                    }
                    GameResult::Failed { profile, seed, detail } =>
                        failures.push((profile, seed, detail)),
                }
            }
        }
        failures.sort_by_key(|&(p, s, _)| (p, s));
        eprintln!("random-game harness: {total} games across {} profiles, {} failed",
            profiles.len(), failures.len());
        for (idx, (label, _)) in profiles.iter().enumerate() {
            let (u, b) = (&agg[idx][0], &agg[idx][1]);
            let games = u.games + b.games;
            if games == 0 { continue; }
            let min_life = u.min_life.min(b.min_life);
            eprintln!("  {label}: {games} games, {} aborted(cap), min life {}, \
                {} attack-decls, max objects {}, max legal-set {}",
                u.aborts + b.aborts,
                if min_life == i32::MAX { 0 } else { min_life },
                u.attacks + b.attacks, u.max_obj.max(b.max_obj), u.max_legal.max(b.max_legal));
        }
        // A failure is "known-open" iff its seed is listed AND the detail
        // still names the recorded site (so a known seed regressing to a
        // different panic is treated as NEW).
        let is_known = |seed: u64, detail: &str|
            KNOWN_OPEN.iter().any(|(s, site)| *s == seed && detail.contains(site));
        let novel: Vec<&(usize, u64, String)> = failures.iter()
            .filter(|(_, s, d)| !is_known(*s, d)).collect();
        for (p, s, d) in failures.iter().take(60) {
            let tag = if is_known(*s, d) { "known-open" } else { "NEW" };
            eprintln!("  [{}] seed {s}: {d} [{tag}]", profiles[*p].0);
        }
        assert!(novel.is_empty(),
            "{} NEW random-game failure(s) not in KNOWN_OPEN (of {} total)",
            novel.len(), failures.len());
    }

    /// CI GATE — behavioral audit. Resolves every spell card in a
    /// populated state and flags any whose resolver returns effects that
    /// change NOTHING observable: the silent-no-op class that bones/stub
    /// verify can't see (it hid the ForEach bug across ~294 cards and the
    /// subtype_filter land-scope bug). FAILS if:
    ///   * any resolver PANICS (always a real bug — caught Pox), or
    ///   * a spell silently no-ops and is NOT in the checked-in
    ///     allowlist baseline (a NEW such bug — e.g. a regression).
    /// The allowlist (behavioral_allowlist.txt) holds the current
    /// harness-limited false-positives; shrink it as the harness
    /// improves, never grow it without confirming the card isn't a bug.
    #[test]
    fn behavioral_audit_no_silent_noops() {
        use std::collections::HashSet;
        let allow: HashSet<&str> = include_str!("behavioral_allowlist.txt")
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();

        let mut reg = arcana_core::registry::CardRegistry::new();
        let n = crate::register_all::register_all(&mut reg);
        let mut suspects = Vec::new();
        let mut panicked = Vec::new();
        for cid in 0..n as u32 {
            let name = || reg.get(cid)
                .and_then(|d| reg.interner().resolve(d.name))
                .unwrap_or("?").to_string();
            // Per-card panic isolation — one bad resolver must not abort
            // the sweep; a panic is itself a finding. Probe BOTH the
            // spell resolver and every triggered ability.
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let spell = arcana_core::behavioral::probe_spell(&reg, cid)
                    .map(|r| r.is_silent_noop()).unwrap_or(false);
                let trig = arcana_core::behavioral::probe_triggered(&reg, cid)
                    .iter().any(|r| r.is_silent_noop());
                let act = arcana_core::behavioral::probe_activated(&reg, cid)
                    .iter().any(|r| r.is_silent_noop());
                spell || trig || act
            }));
            match res {
                Ok(true) => suspects.push(name()),
                Ok(false) => {}
                Err(_) => panicked.push(name()),
            }
        }

        // New silent no-ops = flagged but not in the allowlist baseline.
        let new_noops: Vec<&String> = suspects.iter()
            .filter(|s| !allow.contains(s.as_str())).collect();
        // Stale allowlist entries no longer fire — prune them (warn only).
        let live: HashSet<&str> = suspects.iter().map(String::as_str).collect();
        let stale: Vec<&&str> = allow.iter().filter(|a| !live.contains(*a)).collect();
        if !stale.is_empty() {
            eprintln!("note: {} allowlist entries no longer fire (prune them): {:?}",
                stale.len(), stale);
        }

        assert!(panicked.is_empty(),
            "resolver(s) PANICKED during behavioral probe (real bugs): {panicked:?}");
        assert!(new_noops.is_empty(),
            "NEW silent-no-op spell(s) — resolver returns effects but nothing changes \
             (cf. the ForEach / subtype_filter bugs). Fix the card, or if it's a genuine \
             harness false-positive add it to behavioral_allowlist.txt: {new_noops:?}");
    }

}

#[cfg(test)]
mod behavioral_triage {
    #[test]
    #[ignore]
    fn classify_trigger_noops() {
        use std::collections::BTreeMap;
        let mut reg = arcana_core::registry::CardRegistry::new();
        let n = crate::register_all::register_all(&mut reg);
        let mut by_cond: BTreeMap<&str, usize> = BTreeMap::new();
        let mut examples: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for cid in 0..n as u32 {
            let Some(def) = reg.get(cid) else { continue; };
            if def.triggered_abilities.is_empty() { continue; }
            let results = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                arcana_core::behavioral::probe_triggered(&reg, cid))).unwrap_or_default();
            // results align with abilities that got a verdict (synthesizable)
            let mut ri = 0;
            for ab in &def.triggered_abilities {
                // mirror synth_event's None-only-for-Custom by checking the result stream
                if ri >= results.len() { break; }
                let r = results[ri]; ri += 1;
                if r.is_silent_noop() {
                    let c = cond_name(&ab.trigger_condition);
                    *by_cond.entry(c).or_default() += 1;
                    let e = examples.entry(c).or_default();
                    if e.len() < 4 {
                        e.push(reg.interner().resolve(def.name).unwrap_or("?").to_string());
                    }
                }
            }
        }
        for (c, n) in &by_cond {
            eprintln!("{n:4}  {c}   e.g. {:?}", examples[c]);
        }
    }
    fn cond_name(c: &arcana_core::triggers::TriggerCondition) -> &'static str {
        use arcana_core::triggers::TriggerCondition as T;
        match c {
            T::SelfEntersBattlefield => "ETB", T::SelfDies => "Dies",
            T::SelfAttacks => "Attacks", T::SelfAttacksUnblocked => "AttacksUnblocked",
            T::SelfBecomesBlocked => "BecomesBlocked", T::SelfBlocks => "Blocks",
            T::SelfBlocksOrBecomesBlocked => "BlocksOrBlocked", T::SelfBecomesTapped => "Tapped",
            T::BecomesTapped { .. } => "BecomesTapped(filtered)",
            T::SelfAttacksAlone => "AttacksAlone",
            T::AttacksAlone { .. } => "AttacksAlone(filtered)",
            T::SelfTransforms { .. } => "Transforms",
            T::SelfBecomesBlockedBy { .. } => "BecomesBlockedBy(filtered)",
            T::SelfBlocksOrBecomesBlockedBy { .. } => "BlocksOrBlockedBy(filtered)",
            T::SelfSpecializes => "Specializes", T::SelfBecomesTarget{..} => "BecomesTarget",
            T::SelfIsDealtDamage{..} => "IsDealtDamage", T::ZoneChange{..} => "ZoneChange",
            T::SpellCast{..} => "SpellCast", T::DamageDealt{..} => "DamageDealt",
            T::StepBegins{..} => "StepBegins", T::PhaseBegins{..} => "PhaseBegins",
            T::LifeGained{..} => "LifeGained", T::CounterAdded{..} => "CounterAdded(Saga)",
            T::CardDrawn{..} => "CardDrawn", T::CardDiscarded{..} => "CardDiscarded",
            T::CreatureAttacks{..} => "CreatureAttacks", T::Sacrificed{..} => "Sacrificed",
            T::AttachedCreatureDoes{..} => "AttachedCreatureDoes",
            T::Custom(_) => "Custom",
        }
    }
}
