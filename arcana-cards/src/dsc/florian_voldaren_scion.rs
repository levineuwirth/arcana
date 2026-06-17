//! Florian, Voldaren Scion — `{1}{B}{R}` 3/3 Legendary Creature — Vampire Noble.
//! First strike.
//! At the beginning of each of your postcombat main phases, look at the top X
//! cards of your library, where X is the total amount of life your opponents
//! lost this turn. Exile one of those cards and put the rest on the bottom of
//! your library in a random order. You may play the exiled card this turn.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Florian, Voldaren Scion");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PostCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: impulse_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn impulse_x(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // X = total life opponents lost this turn.
    let x: u32 = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| script::life_lost_this_turn(state, p))
        .sum();
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: x,
    }]
}
