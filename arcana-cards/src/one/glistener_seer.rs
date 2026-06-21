//! Glistener Seer — `{U}` 0/3 Creature — Phyrexian Advisor.
//!
//! Oracle:
//! * "This creature enters with three oil counters on it." →
//!   `EntersWithSpec::Counters { kind: Named("oil"), count: 3 }`.
//! * "{T}, Remove an oil counter from this creature: Scry 1." → an
//!   activated ability with a tap + remove-oil-counter cost.
//!
//! (Scryfall lists "Scry" as a keyword, but it is the activated
//! ability's effect, not a keyword-line ability — there is no Scry
//! `KeywordAbility` variant; modelled as the activation effect.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glistener Seer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let advisor = reg.interner_mut().intern("Advisor");
    let oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Named(oil),
                count: 3,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove an oil counter from this creature: Scry 1.".into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(oil), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: scry_one,
            }),
    )
}

fn scry_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}
