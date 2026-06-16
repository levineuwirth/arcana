//! Freyalise, Llanowar's Fury — `{3}{G}{G}` Legendary Planeswalker — Freyalise,
//! starting loyalty 3. Mono-green.
//!
//! Oracle:
//! +2: Create a 1/1 green Elf Druid creature token with "{T}: Add {G}."
//! −2: Destroy target artifact or enchantment.
//! −6: Draw a card for each green creature you control.
//! Freyalise, Llanowar's Fury can be your commander.
//!
//! # Scope
//! * +2 — partial: create a 1/1 green Elf Druid token. GAP: the token's
//!   "{T}: Add {G}" mana ability is not expressible on a TokenDefinition.
//! * −2 — modeled: destroy target artifact or enchantment.
//! * −6 — modeled: draw a card for each green creature you control (counted at
//!   resolution).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::effects::TokenDefinition;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Freyalise, Llanowar's Fury");
    let sub = reg.interner_mut().intern("Freyalise");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let _ = reg.interner_mut().intern("Elf");
    let _ = reg.interner_mut().intern("Druid");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let artifact_or_enchantment = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_types_any((TypeLine::ARTIFACT | TypeLine::ENCHANTMENT).into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Create a 1/1 green Elf Druid creature token with \
                       \"{T}: Add {G}.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_elf,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Destroy target artifact or enchantment.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![artifact_or_enchantment],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Draw a card for each green creature you \
                       control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_draw,
            }),
    )
}

/// `+2:` create a 1/1 green Elf Druid token (mana ability GAP'd).
fn plus_two_elf(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf").unwrap_or_default();
    let druid = reg.interner().lookup("Druid").unwrap_or_default();
    let mut t_subtypes = SubtypeSet::default();
    t_subtypes.0.insert(elf);
    t_subtypes.0.insert(druid);
    // GAP: token's "{T}: Add {G}" mana ability is not expressible.
    let token = TokenDefinition {
        name: elf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: t_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

/// `−2:` destroy target artifact or enchantment.
fn minus_two_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−6:` draw a card for each green creature you control.
fn minus_six_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::green());
    let n = script::count_matching(state, &filter, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: n,
    }]
}
