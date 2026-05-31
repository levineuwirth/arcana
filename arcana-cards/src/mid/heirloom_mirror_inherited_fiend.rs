//! Heirloom Mirror // Inherited Fiend (transforming DFC, layout "transform")
//!
//! Front face: Heirloom Mirror — {1}{B} Artifact.
//!   {1}, {T}, Pay 1 life, Discard a card: Draw a card, mill a card, then put a
//!     ritual counter on this artifact. Then if it has three or more ritual
//!     counters on it, remove them and transform it. Activate only as a sorcery.
//! Back face: Inherited Fiend — Creature — Demon, 5/5 with Flying (B).
//!   {2}{B}: Exile target creature card from a graveyard. Put a +1/+1 counter on this creature.
//!
//! GAP: the front activation's "then if it has three or more ritual counters on
//!   it, remove them and transform it" tail is not expressible — Effect::Conditional's
//!   Condition has no source-counter-count variant, and Custom can't read the
//!   source object. Emitting the draw + mill + add-ritual-counter portion only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heirloom Mirror");
    let ritual = reg.interner_mut().intern("Ritual");
    let demon = reg.interner_mut().intern("Demon");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Inherited Fiend");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(demon);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    let _ = ritual;

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: {1},{T},Pay 1 life,Discard a card: Draw, mill, add ritual counter.
            // Sorcery speed (is_instant_speed: false). Front-only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Pay 1 life, Discard a card: Draw a card, mill a card, then put a ritual counter on this artifact. Then if it has three or more ritual counters on it, remove them and transform it. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    life: 1,
                    discard_other: Some(ObjectFilter::default()),
                    discard_other_count: 1,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: front_activation,
            })
            // Back: {2}{B}: Exile target creature card from a graveyard.
            //   Put a +1/+1 counter on this creature. Back-only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Exile target creature card from a graveyard. Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: back_activation,
            }),
    )
}

fn front_activation(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "then if it has three or more ritual counters, remove them and
    // transform it" tail is not expressible (no source-counter Condition).
    let mut effects = vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Mill {
            player: ctx.controller,
            count: 1,
        },
    ];
    if let Some(ritual) = reg.interner().lookup("Ritual") {
        effects.push(Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Named(ritual),
            count: 1,
        });
    }
    effects
}

fn back_activation(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let _ = reg;
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
