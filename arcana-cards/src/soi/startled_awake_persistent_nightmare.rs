//! Startled Awake // Persistent Nightmare (transforming DFC).
//!
//! Front (Startled Awake — Sorcery): Target opponent mills thirteen cards.
//! {3}{U}{U}: Put this card from your graveyard onto the battlefield transformed.
//!   Activate only as a sorcery. (GAP — graveyard-cast/return-transformed activation
//!   not expressible.)
//! Back (Persistent Nightmare — Creature — Nightmare, 1/1): Skulk.
//!   When this creature deals combat damage to a player, return it to its owner's hand.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, CardFace, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Startled Awake");
    let back_name = reg.interner_mut().intern("Persistent Nightmare");
    let nightmare = reg.interner_mut().intern("Nightmare");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(nightmare);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Skulk],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: "{3}{U}{U}: Put this card from your graveyard onto the battlefield
    // transformed. Activate only as a sorcery." — graveyard-activated
    // return-onto-battlefield-transformed is not expressible with the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent mills thirteen cards.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
                modal: None,
                effect: mill_thirteen,
            })
            .with_transform_back(back)
            // Back-face trigger: combat damage to a player -> return to hand.
            // GAP (fidelity): DamageDealt has no self-source scoping; this fires
            // on any creature's combat damage to a player rather than only this
            // creature's. (No self-combat-damage trigger condition in the engine.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: return_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn mill_thirteen(_: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::Mill {
        player: *p,
        count: 13,
    }]
}

fn return_self(_: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: trig.source }]
}
