//! Scourge of Kher Ridges — `{6}{R}{R}` 6/6 Dragon.
//!
//! Flying.
//! {1}{R}: This creature deals 2 damage to each creature without flying.
//! {5}{R}: This creature deals 6 damage to each other creature with
//! flying.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge of Kher Ridges");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature deals 2 damage to each creature without \
                       flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blast_no_flying,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{R}: This creature deals 6 damage to each other creature \
                       with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blast_flying,
            }),
    )
}

fn blast_no_flying(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().without_keyword(KeywordAbility::Flying),
        ctx.controller,
    );
    ids.into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 2,
        })
        .collect()
}

fn blast_flying(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
        ctx.controller,
    );
    ids.into_iter()
        .filter(|id| *id != ctx.source) // "each OTHER creature with flying"
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 6,
        })
        .collect()
}
