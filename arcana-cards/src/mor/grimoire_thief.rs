//! Grimoire Thief — `{U}{U}` 2/2 Merfolk Rogue.
//! "Whenever this creature becomes tapped, exile the top three cards of
//!  target opponent's library face down."
//! "You may look at cards exiled with this creature." (static)
//! "{U}, Sacrifice this creature: Turn all cards exiled with this creature
//!  face up. Counter all spells with those names."
//!
//! The becomes-tapped trigger and the sacrifice ability both rely on an
//! exile-pile linked to this creature, which is not modeled; their effects
//! are GAP'd. The "you may look" line is a pure static with no effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grimoire Thief");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: on_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, Sacrifice Grimoire Thief: Turn all cards exiled with Grimoire Thief face up. Counter all spells with those names.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: turn_face_up_and_counter,
            }),
    )
}

fn on_tapped(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile the top three cards of target opponent's library face down"
    // (linked to this creature) is not expressible — no exile-from-library
    // effect with a source-linked face-down pile.
    Vec::new()
}

fn turn_face_up_and_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "turn all cards exiled with this creature face up; counter all
    // spells with those names" depends on the source-linked exile pile, which
    // is not modeled.
    Vec::new()
}
