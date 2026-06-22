//! Vicious Battlerager — `{3}{B}` 1/5 Dwarf Barbarian.
//! "When this creature enters, you take the initiative."
//! "Spiked Retribution — Whenever this creature becomes blocked by a
//!  creature, that creature's controller loses 5 life."
//!
//! Wired: the becomes-blocked trigger (the blocking creature's
//! controller loses 5 life). The ETB "you take the initiative" is GAP'd
//! (no Effect for the initiative mechanic). "Spiked Retribution" is an
//! ability word, not a keyword.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vicious Battlerager");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: effect — "you take the initiative" (no initiative Effect).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: take_initiative_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocker_controller_loses_5,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn take_initiative_gap(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: take the initiative (no Effect for the initiative mechanic).
    Vec::new()
}

fn blocker_controller_loses_5(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(blocker) = trig.other_combatant() else { return Vec::new(); };
    let p = script::target_controller(state, blocker, trig.controller);
    vec![Effect::LoseLife { player: p, amount: 5 }]
}
