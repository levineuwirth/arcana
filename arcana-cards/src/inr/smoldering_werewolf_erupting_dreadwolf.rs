//! Smoldering Werewolf // Erupting Dreadwolf — `{2}{R}{R}` red Werewolf Horror 3/2 (front) /
//! Eldrazi Werewolf (back). Transform double-faced card.
//!
//! Front face (Smoldering Werewolf):
//!   When this creature enters, it deals 1 damage to each of up to two target creatures.
//!   {4}{R}{R}: Transform this creature.
//!
//! Back face (Erupting Dreadwolf):
//!   Whenever this creature attacks, it deals 2 damage to any target.
//!   GAP: back-face-only triggered ability (attack deals 2 damage) not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smoldering Werewolf");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf_sub);
    subtypes.0.insert(horror_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Erupting Dreadwolf — Eldrazi Werewolf
    let back_name = reg.interner_mut().intern("Erupting Dreadwolf");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi_sub);
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: when this creature enters, deals 1 damage to each of up to two
            // target creatures.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // Front face: {4}{R}{R}: Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}{R}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0), // front face only
                effect: activated_transform,
            }),
        // GAP: back-face-only triggered ability (whenever this creature attacks, deals 2 damage
        //      to any target) not auto-installed on transform.
    )
}

/// When Smoldering Werewolf enters: deal 1 damage to each of up to two target creatures.
fn etb_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| {
            let id = match t {
                TargetChoice::Object(id) => *id,
                _ => return None,
            };
            Some(Effect::DealDamage {
                target: DamageTarget::Object(id),
                amount: 1,
                source: trig.source,
            })
        })
        .collect()
}

/// {4}{R}{R}: Transform (front face → back face).
fn activated_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
