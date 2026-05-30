//! Vengeful Strangler // Strangling Grasp
//!
//! Front: {1}{B} Creature — Human Rogue 2/1
//! This creature can't block.
//! When this creature dies, return it to the battlefield transformed under your control
//! attached to target creature or planeswalker an opponent controls.
//!
//! Back: Enchantment — Aura
//! Enchant creature or planeswalker an opponent controls.
//! At the beginning of your upkeep, enchanted permanent's controller sacrifices a nonland
//! permanent of their choice, then that player loses 1 life.
//!
//! GAP: "sacrifice a nonland permanent of their choice" — the Sacrifice effect takes
//! ObjectFilter::permanent().without_types(TypeLine::LAND.into()) but the chooser is the
//! enchanted permanent's controller, not trig.controller. chooser-selection for Sacrifice
//! is not expressible; emit the lose-life portion only and GAP the sacrifice.
//! GAP: back-face-only triggered ability (upkeep trigger) not modeled on back face.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Strangler");
    let sub_human = reg.interner_mut().intern("Human");
    let sub_rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "This creature can't block" — static cant-block not modeled as a keyword
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Strangling Grasp");
    let sub_aura = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_aura);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front: when this creature dies, return it transformed attached to a target creature
    // or planeswalker an opponent controls.
    // The ZoneChange trigger fires when it moves to the graveyard.
    // Effect::Transform + Attach represent the return-and-attach semantics.
    // GAP: "return it to the battlefield transformed attached to target creature or planeswalker"
    // — ReturnFromGraveyardToBattlefield + Transform + Attach is approximate; targeting
    // constraint (opponent's creature or planeswalker) not enforced in trigger.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return it to the battlefield transformed attached to target creature or
    // planeswalker an opponent controls" — full semantics require targeting at trigger
    // resolution; emitting Transform only as approximate skeleton.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
