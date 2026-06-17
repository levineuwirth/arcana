//! Dread Presence — `{3}{B}` 3/3 Nightmare.
//! Whenever a Swamp you control enters, choose one —
//! • You draw a card and you lose 1 life.
//! • This creature deals 2 damage to any target and you gain 2 life.
//!
//! The trigger condition (a Swamp you control entering) is faithful.
//! Modal "choose one" is NOT expressible on a triggered ability (modal
//! is a spell-ability-only feature), so we resolve the non-targeted
//! first mode and GAP the choice / second (any-target damage) mode.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dread Presence");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);

    let swamp_filter = script::subtype_filter(reg, "Swamp")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: swamp_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_swamp_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_swamp_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one —" modal trigger is not expressible on a
    // triggered ability (modal is spell-ability-only). Resolving the
    // first mode (draw a card and lose 1 life); the second mode (deal 2
    // damage to any target, gain 2 life) cannot be offered as a choice.
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
