//! Garland, Knight of Cornelia // Chaos, the Endless — {B}{R}
//!
//! Front: Legendary Creature — Human Knight 3/2
//! Whenever you cast a noncreature spell, surveil 1.
//! {3}{B}{B}{R}{R}: Return this card from your graveyard to the battlefield transformed.
//!   Activate only as a sorcery.
//!
//! Back: Legendary Creature — Demon 6/6
//! Flying
//! When Chaos dies, put it on the bottom of its owner's library.
//!
//! GAP: "Return this card from your graveyard to the battlefield transformed" as an activated
//!      ability is not expressible — activated abilities can't target cards in the graveyard
//!      and self-transform from graveyard is not modeled. Omitting that activation.
//! GAP: Back-face-only triggered ability (dies -> bottom of library) not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garland, Knight of Cornelia");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Chaos, the Endless");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(demon_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Noncreature spell cast -> surveil 1
    // SpellCast filter: ObjectFilter without creature type
    let noncreature_filter = ObjectFilter::new()
        .without_types(TypeLine::CREATURE.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(noncreature_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_noncreature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: {3}{B}{B}{R}{R} graveyard-to-battlefield-transformed activation not modeled
        // GAP: back-face dies-to-bottom-of-library triggered ability not modeled
    )
}

fn on_noncreature_cast(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: trig.controller,
        count: 1,
    }]
}
