//! Myojin of Cryptic Dreams — `{5}{U}{U}{U}` 3/3 Legendary Spirit.
//!
//! * "Enters with an indestructible counter on it if you cast it from
//!   your hand." — the cast-from-hand condition has no ETB accessor;
//!   GAP'd.
//! * "Remove an indestructible counter from Myojin of Cryptic Dreams:
//!   Copy target permanent spell you control three times. (The copies
//!   become tokens.)" — fully expressed (counter-removal cost +
//!   three `CopySpell` effects targeting a permanent spell you control).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Cryptic Dreams");
    let spirit = reg.interner_mut().intern("Spirit");
    let indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    // "target permanent spell you control" — a spell on the stack that is
    // a permanent (creature/artifact/enchantment/planeswalker/land) you
    // control.
    let permanent_spell_filter =
        ObjectFilter::permanent().controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove an indestructible counter from Myojin of Cryptic Dreams: Copy target permanent spell you control three times.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(indestructible), 1)),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(permanent_spell_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: copy_thrice,
        }),
    )
}

fn copy_thrice(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::CopySpell { target: *id },
        Effect::CopySpell { target: *id },
        Effect::CopySpell { target: *id },
    ]
}
