//! Lambholt Raconteur // Lambholt Ravager — {3}{R} Creature — Human Werewolf // Werewolf (2/4)
//! Front: Whenever you cast a noncreature spell, this creature deals 1 damage to each opponent.
//!   Daybound (GAP: day/night cycle not modeled).
//! Back: Whenever you cast a noncreature spell, this creature deals 2 damage to each opponent.
//!   Nightbound (GAP: day/night cycle not modeled).
//! GAP: Daybound/Nightbound keywords not in engine keyword surface.
//! GAP: Back-face-only triggered ability (2 damage) not modeled separately; shared trigger
//!   uses 1 damage on both faces. A proper face-gated solution needs face_gate on triggered
//!   abilities which is not currently supported.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::mana::ManaCost;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lambholt Raconteur");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        // GAP: Daybound keyword not in engine keyword surface.
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Lambholt Ravager");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![],
            // GAP: Nightbound keyword not in engine keyword surface.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: whenever you cast a noncreature spell, deal 1 damage to each opponent.
            // GAP: Both faces share this trigger; back face should deal 2 damage instead of 1.
            // GAP: Daybound/Nightbound transform conditions not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: damage_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damage_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|opp| Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 1,
            source: trig.source,
        })
        .collect()
}
