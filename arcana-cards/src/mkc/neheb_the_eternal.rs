//! Neheb, the Eternal — `{3}{R}{R}` 4/6 Legendary Zombie Minotaur Warrior.
//! "Afflict 3 (Whenever this creature becomes blocked, defending player loses 3 life.)"
//! "At the beginning of each of your postcombat main phases, add {R} for each 1
//! life your opponents have lost this turn."
//!
//! Afflict 3 is not in the usable KeywordAbility surface; it is modeled as a
//! SelfBecomesBlocked trigger, but the "defending player loses 3 life" body is a
//! GAP — no accessor exposes the defending player for a SelfBecomesBlocked event.
//! The postcombat-main mana ability is fully wired.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Neheb, the Eternal");
    let zombie = reg.interner_mut().intern("Zombie");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: keyword — Afflict 3 not in the usable KeywordAbility surface;
        // modeled as a SelfBecomesBlocked trigger below.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: afflict_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PostCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_red_for_life_lost,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn afflict_3(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Afflict 3 — "defending player loses 3 life". No accessor exposes the
    // defending player for a SelfBecomesBlocked event (defending_player() only
    // resolves attack events), so the target player of the life loss is not
    // recoverable here.
    Vec::new()
}

fn add_red_for_life_lost(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "add {R} for each 1 life your opponents have lost this turn".
    let n: u32 = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| script::life_lost_this_turn(state, p))
        .sum();
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); n as usize],
    }]
}
