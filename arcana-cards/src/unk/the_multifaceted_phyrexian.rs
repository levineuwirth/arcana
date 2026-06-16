//! The Multifaceted Phyrexian — `{2}{B}{B}` 2/4 Legendary Phyrexian Ninja.
//!
//! Oracle:
//! * Protection from Elk. (Static protection-from-a-subtype — the
//!   `Protection` keyword has no `KeywordAbility` variant and the engine
//!   exposes no protection-from-subtype primitive; GAP'd below.)
//! * "Pay 1 life, Sacrifice another creature: Target creature an opponent
//!   controls becomes a 3/3 green Elk creature." — modeled as an activated
//!   ability whose cost is `life: 1` + `sacrifice_other` (a creature). The
//!   resolver sets the target's base P/T to 3/3, sets its color to green,
//!   and makes it a creature (the "Elk" creature SUBTYPE is not expressible
//!   — no add-subtype effect — so only the expressible parts are emitted).
//! * "Fixed commander ninjutsu — {B}{B}, Discard a card" — the commander
//!   ninjutsu cast mechanic is not modeled by the engine; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Multifaceted Phyrexian");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — "Protection from Elk" (no Protection KeywordAbility
        // variant; protection-from-subtype is not an expressible primitive).
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Pay 1 life, Sacrifice another creature: Target creature an
            // opponent controls becomes a 3/3 green Elk creature."
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay 1 life, Sacrifice another creature: Target creature an opponent controls becomes a 3/3 green Elk creature.".into(),
                cost: ActivationCost {
                    life: 1,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_elk,
            }),
            // GAP: ability — "Fixed commander ninjutsu — {B}{B}, Discard a
            // card" (the ninjutsu / commander-ninjutsu cast mechanic is not
            // modeled by the engine — no ninjutsu cost or alternate-cast
            // primitive).
    )
}

fn become_elk(
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
    // "becomes a 3/3 green Elk creature" (indefinitely). The "Elk" creature
    // SUBTYPE is not expressible (no add-subtype effect); set the
    // expressible parts: base 3/3, green, and a creature.
    vec![
        Effect::SetBasePT {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::Permanent,
        },
        Effect::SetColor {
            target: *id,
            colors: ColorSet::green(),
            duration: Duration::Permanent,
        },
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::Permanent,
        },
    ]
}
