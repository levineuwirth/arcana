//! Flame Channeler // Embodiment of Flame — `{1}{R}` Creature — Human Wizard
//! 2/2 (front), transforms to Creature — Elemental Wizard (back).
//!
//! Front: When a spell you control deals damage, transform this creature.
//! Back: Whenever a spell you control deals damage, put a flame counter on
//!       this creature. {1}, Remove a flame counter from this creature: Exile
//!       the top card of your library. You may play that card this turn.
//!
//! # Notes
//! - "When/Whenever a spell you control deals damage" is wired via a `DamageDealt`
//!   trigger whose `source_filter` matches an instant/sorcery spell you control
//!   (the Blaze Commando idiom). Front face → transform; back face → put a flame
//!   counter on this creature.
//! - The flame counter is `CounterKind::Named("flame")`. The back-face activated
//!   ability "{1}, Remove a flame counter: exile the top card of your library, you may
//!   play it this turn" is wired via `ImpulseExile` with a `remove_self_counter` cost.
//! - "Transform" Scryfall keyword is a marker; not a KeywordAbility variant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame Channeler");
    let flame = reg.interner_mut().intern("flame");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Embodiment of Flame");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let wizard_sub2 = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental_sub);
    back_subtypes.0.insert(wizard_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // "A spell you control deals damage" — a DamageDealt trigger whose source is an
    // instant/sorcery spell you control (the Blaze Commando idiom).
    let spell_you_control = || {
        ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
            .controlled_by(ControllerConstraint::You)
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: when a spell you control deals damage, transform this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: spell_you_control(),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: whenever a spell you control deals damage, put a flame counter
            // on this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: spell_you_control(),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: add_flame_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            // Back: "{1}, Remove a flame counter from this creature: Exile the top
            // card of your library. You may play that card this turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a flame counter from this creature: Exile the top card of your library. You may play that card this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::Named(flame), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: impulse_top,
            }),
    )
}

fn transform_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn add_flame_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(flame) = reg.interner().lookup("flame").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: flame,
        count: 1,
    }]
}

fn impulse_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: ctx.controller,
        count: 1,
    }]
}
