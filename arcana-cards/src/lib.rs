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
        use arcana_core::engine::{new_game, step, EngineYield};
        // 400 games (200 uniform + 200 biased) exercises every KNOWN_OPEN
        // seed in ~20s. Running with a larger GAMES is a valid deeper sweep
        // and may surface NEW findings beyond the recorded baseline.
        const GAMES: u64 = 400;
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
        // Battlefield creatures across both players. The blocker-subset and
        // damage-distribution enumerations are exponential in creature count
        // (2^blockers, product of factorials), so this — not total objects —
        // is the real OOM gate; the explosion is reachable at a few dozen
        // creatures. Capping here keeps any single legal-action set bounded.
        const CREATURE_CAP: usize = 24;
        const LEGAL_CAP: usize = 200_000;

        // Seeds known to hit still-open engine bugs that biased combat
        // play surfaces — each pinned to its panic site. A failure on a
        // listed seed whose detail still contains the recorded site is
        // expected (logged, not fatal); a failure anywhere else, or a
        // listed seed failing for a *different* reason, is a NEW finding
        // and fails the test. Remove an entry when its bug is fixed.
        //   stack.rs:697   — finalize_resolved_spell: object vanished
        //                    from the arena before finalize.
        const KNOWN_OPEN: &[(u64, &str)] = &[
            (129, "stack.rs:697"),
        ];

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
        ) -> Result<(), String> {
            // (1) State-based actions are settled at a decision point.
            // No player may still satisfy a loss condition (CR 704):
            // life <= 0, drawn-from-empty-library, or >=10 poison. In a
            // 2-player game a loss also ends the game, so seeing a
            // `has_lost` flag at a *pending* (non-over) decision means
            // SBAs leaked a player who should already have lost.
            for p in 0..state.num_players() {
                let pl = state.player(p);
                if pl.has_lost {
                    return Err(format!(
                        "SBA leak: player {p} has_lost but game still pending"));
                }
                if pl.life <= 0 {
                    return Err(format!(
                        "SBA leak: player {p} at {} life, game still pending",
                        pl.life));
                }
                if pl.poison_counters >= 10 {
                    return Err(format!(
                        "SBA leak: player {p} at {} poison, game still pending",
                        pl.poison_counters));
                }
                if pl.has_drawn_from_empty_library {
                    return Err(format!(
                        "SBA leak: player {p} drew from empty library, pending"));
                }
            }
            // (2) Object identity: every live arena object has a unique
            // id. A duplicate means a re-id/clone bug (the kind the
            // London-bottom re-id path could have introduced).
            let mut seen = std::collections::HashSet::new();
            for o in state.objects.iter() {
                if !seen.insert(o.id) {
                    return Err(format!("duplicate object id {} in arena", o.id));
                }
            }
            Ok(())
        }

        let mut failures: Vec<(u64, String)> = Vec::new();
        // Per-mode telemetry, indexed [uniform, biased]:
        //   games, min life seen, attacks, max objects, max legal-set, aborts.
        #[derive(Clone, Copy)]
        struct ModeAgg { games: u64, min_life: i32, attacks: u64,
                         max_obj: usize, max_legal: usize, aborts: u64 }
        let mut agg = [ModeAgg { games: 0, min_life: i32::MAX, attacks: 0,
                                 max_obj: 0, max_legal: 0, aborts: 0 }; 2];
        for seed in 0..GAMES {
            let mut rng = Lcg(seed.wrapping_mul(2654435761).wrapping_add(1));
            // Two 40-card decks: 18 basics + 22 random catalog cards.
            let deck = |rng: &mut Lcg| -> Vec<u32> {
                let mut d = Vec::with_capacity(40);
                for _ in 0..18 { d.push(basics[rng.next(5)]); }
                for _ in 0..22 { d.push(valid[rng.next(valid.len())]); }
                d
            };
            let decks = vec![deck(&mut rng), deck(&mut rng)];
            // Even seeds play uniform-random; odd seeds use the biased
            // picker. Splitting keeps both coverage profiles.
            let bias = seed % 2 == 1;

            // Per-game telemetry: lowest life any player reached and how
            // many non-empty attacker declarations were committed. Lets
            // the harness self-report that biased play actually drives
            // combat / low-life states uniform play barely reaches.
            // Per-game report: (min life, attacks, max objects, max legal
            // set, aborted-by-cap?). `Err` = a real failure (panic / stuck
            // / non-termination / invariant breach); an abort is `Ok` with
            // the flag set — bounded, not a failure.
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let (mut state, mut yld) = new_game(decks, &reg, seed);
                let mut min_life = i32::MAX;
                let mut attacks: u64 = 0;
                let mut max_obj = 0usize;
                let mut max_legal = 0usize;
                for _ in 0..STEP_CAP {
                    match yld {
                        EngineYield::GameOver(_) =>
                            return Ok((min_life, attacks, max_obj, max_legal, false)),
                        EngineYield::PendingDecision { legal_actions, .. } => {
                            // Mid-game invariants: the engine checks SBAs
                            // and settles before handing back a decision,
                            // so the state must be internally consistent
                            // here. A violation is an engine bug the
                            // panic/stuck/terminate checks would miss.
                            game_invariants(&state)?;
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
                                || legal_actions.len() > LEGAL_CAP {
                                return Ok((min_life, attacks, max_obj, max_legal, true));
                            }
                            if legal_actions.is_empty() {
                                return Err("stuck: no legal actions".to_string());
                            }
                            let i = pick(&mut rng, &legal_actions, bias);
                            let action = legal_actions[i].clone();
                            if matches!(&action,
                                arcana_core::actions::Action::DeclareAttackers { attackers }
                                if !attackers.is_empty()) {
                                attacks += 1;
                            }
                            let (s, y) = step(state, action, &reg);
                            state = s; yld = y;
                        }
                    }
                }
                Err(format!("did not terminate in {STEP_CAP} steps"))
            }));
            let m = bias as usize;
            match res {
                Ok(Ok((ml, atk, mo, mleg, aborted))) => {
                    agg[m].games += 1;
                    agg[m].min_life = agg[m].min_life.min(ml);
                    agg[m].attacks += atk;
                    agg[m].max_obj = agg[m].max_obj.max(mo);
                    agg[m].max_legal = agg[m].max_legal.max(mleg);
                    if aborted { agg[m].aborts += 1; }
                }
                Ok(Err(msg)) => failures.push((seed, msg)),
                Err(_) => {
                    let loc = PANIC_LOC.with(|l| l.borrow_mut().take())
                        .unwrap_or_else(|| "?".to_string());
                    failures.push((seed, format!("PANIC at {loc}")));
                }
            }
        }
        std::panic::set_hook(prev_hook);
        eprintln!("random games: {} played, {} failed", GAMES, failures.len());
        for (m, label) in [(0usize, "uniform"), (1usize, "biased")] {
            let a = agg[m];
            eprintln!("  {label}: {} games, {} aborted(cap), min life {}, \
                {} attack-decls, max objects {}, max legal-set {}",
                a.games, a.aborts,
                if a.min_life == i32::MAX { 0 } else { a.min_life },
                a.attacks, a.max_obj, a.max_legal);
        }
        // A failure is "known-open" iff its seed is listed AND the detail
        // still names the recorded site (so a known seed regressing to a
        // different panic is treated as NEW).
        let is_known = |seed: u64, detail: &str|
            KNOWN_OPEN.iter().any(|(s, site)| *s == seed && detail.contains(site));
        let novel: Vec<&(u64, String)> = failures.iter()
            .filter(|(s, d)| !is_known(*s, d)).collect();
        for (s, d) in failures.iter().take(60) {
            let tag = if is_known(*s, d) { "known-open" } else { "NEW" };
            eprintln!("  seed {s}: {d} [{tag}]");
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
            T::SelfSpecializes => "Specializes", T::SelfBecomesTarget{..} => "BecomesTarget",
            T::SelfIsDealtDamage{..} => "IsDealtDamage", T::ZoneChange{..} => "ZoneChange",
            T::SpellCast{..} => "SpellCast", T::DamageDealt{..} => "DamageDealt",
            T::StepBegins{..} => "StepBegins", T::PhaseBegins{..} => "PhaseBegins",
            T::LifeGained{..} => "LifeGained", T::CounterAdded{..} => "CounterAdded(Saga)",
            T::CardDrawn{..} => "CardDrawn", T::CardDiscarded{..} => "CardDiscarded",
            T::CreatureAttacks{..} => "CreatureAttacks", T::Sacrificed{..} => "Sacrificed",
            T::Custom(_) => "Custom",
        }
    }
}
