//! Opal Lake Gatekeepers — `{3}{U}` 2/4 Vedalken Soldier. "When this creature
//! enters, if you control two or more Gates, you may draw a card."
//!
//! Intervening-if "if you control two or more Gates" modeled via
//! `conditions::you_control_subtype_at_least` (CR 603.4 gate at fire time).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Opal Lake Gatekeepers");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(iif_two_or_more_gates),
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_two_or_more_gates(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype_at_least(state, reg, you, "Gate", 2)
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // Intervening-if "two or more Gates" gates the trigger; emit the draw here.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
