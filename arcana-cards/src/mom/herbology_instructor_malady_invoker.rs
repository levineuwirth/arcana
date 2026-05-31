//! Herbology Instructor // Malady Invoker — {1}{G} Creature — Treefolk Druid 1/3 (green).
//!
//! Front (Herbology Instructor):
//!   When this creature enters, you gain 3 life.
//!   {6}{B/P}: Transform this creature. Activate only as a sorcery.
//! Back (Malady Invoker, Creature — Phyrexian Treefolk):
//!   When this creature transforms into Malady Invoker, target creature an
//!   opponent controls gets -0/-X until end of turn, where X is this
//!   creature's power.
//!
//! GAP: "When this creature transforms into Malady Invoker" has no expressible
//! TriggerCondition (the trigger enum has no transforms-into event). The back
//! face's on-transform -0/-X debuff is therefore not wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herbology Instructor");
    let treefolk_sub = reg.interner_mut().intern("Treefolk");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk_sub);
    subtypes.0.insert(druid_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Malady Invoker — Phyrexian Treefolk creature.
    let back_name = reg.interner_mut().intern("Malady Invoker");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_treefolk_sub = reg.interner_mut().intern("Treefolk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_treefolk_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front ETB: you gain 3 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            // {6}{B/P}: Transform this creature. Activate only as a sorcery (front face).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{B/P}: Transform this creature. Activate only as a sorcery."
                    .to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{B/P}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            }),
    )
}

fn etb_gain_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 3,
    }]
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
