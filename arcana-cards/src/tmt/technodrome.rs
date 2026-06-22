//! Technodrome — `{2}` 3/3 Artifact Creature — Construct.
//!
//! Oracle:
//! * Reach, trample — `KeywordAbility::Reach`, `KeywordAbility::Trample`.
//! * "This creature can't attack or block unless its power is 6 or greater."
//!   — a static combat restriction with no expressible primitive; GAP'd.
//! * "{T}, Sacrifice another artifact: Draw a card. Put a +1/+1 counter on
//!   this creature." — a tap + sacrifice-another-artifact activation that
//!   draws then adds a +1/+1 counter to itself.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Technodrome");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static — "can't attack or block unless its power is 6 or greater"
    // (no power-gated combat-restriction primitive).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice another artifact: Draw a card. Put a +1/+1 counter on this creature."
                .into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::ARTIFACT.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_and_grow,
        }),
    )
}

fn draw_and_grow(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
