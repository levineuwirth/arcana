//! Solphim, Mayhem Dominus — `{2}{R}{R}` 5/4 Legendary Phyrexian Horror.
//!
//! Oracle:
//! * If a source you control would deal noncombat damage to an opponent
//!   or a permanent an opponent controls, it deals double that damage to
//!   that player or permanent instead. (Static replacement — GAP'd.)
//! * `{1}{R/P}{R/P}`, Discard two cards: Put an indestructible counter on
//!   Solphim.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solphim, Mayhem Dominus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let _indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: static noncombat-damage-doubling replacement is not a
        // triggered/activated ability and is not expressible here.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R/P}{R/P}, Discard two cards: Put an indestructible counter on Solphim.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R/P}{R/P}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
                    discard_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_indestructible_counter,
            }),
    )
}

fn add_indestructible_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = match reg.interner().lookup("indestructible") {
        Some(sym) => CounterKind::Named(sym),
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: 1,
    }]
}
