//! Silvar, Devourer of the Free — `{3}{B}{R}` 4/2 Legendary Cat Nightmare
//! with Menace.
//!
//! Oracle:
//! * "Partner with Trynn, Champion of Freedom (...)" — Partner deck-
//!   construction keyword + partner-tutor ETB; not in the usable surface.
//!   GAP'd.
//! * Menace — keyword.
//! * "Sacrifice a Human: Put a +1/+1 counter on Silvar. It gains
//!   indestructible until end of turn." — a sacrifice-a-Human activation;
//!   add a +1/+1 counter to the source and grant it Indestructible EOT.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silvar, Devourer of the Free");
    let cat = reg.interner_mut().intern("Cat");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(nightmare);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Partner with Trynn, Champion of Freedom" — Partner keyword
        // + partner-tutor ETB not in the usable surface.
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let human_filter =
        arcana_core::targets::ObjectFilter::creature().with_subtypes_any(vec![human]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Human: Put a +1/+1 counter on Silvar. It gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(human_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_and_indestructible,
            }),
    )
}

fn counter_and_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
    ]
}
