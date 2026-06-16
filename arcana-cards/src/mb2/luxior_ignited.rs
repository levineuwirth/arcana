//! Luxior, Ignited — `{4}` Legendary Artifact Planeswalker —
//! Equipment Luxior. Colorless.
//!
//! No printed loyalty is given in the spec; a nominal `loyalty: Some(3)`
//! is placed so the card is a legal planeswalker.
//!
//! Static: "Equipped creature gets +1/+1 for each counter on Luxior,
//! Ignited." — a dynamic equip anthem keyed off this permanent's counter
//! count; not a loyalty ability and beyond the demonstrated surface. GAP.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Attach this Equipment to up to one target creature you
//!   control. — expressible via `Effect::Attach` (equipment = source).
//! * `−2`: Equipped creature gets +2/+2 and gains double strike until end
//!   of turn. — GAP (no demonstrated accessor for "the creature this
//!   Equipment is attached to" from the resolver).

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
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Luxior, Ignited");
    let luxior = reg.interner_mut().intern("Luxior");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(luxior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::PLANESWALKER),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // No printed loyalty in the spec; nominal value placed.
        loyalty: Some(3),
        ..Default::default()
    };
    // GAP: "Equipped creature gets +1/+1 for each counter on Luxior"
    // dynamic equip anthem omitted.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Attach this Equipment to up to one target creature you \
                       control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_attach,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Equipped creature gets +2/+2 and gains double strike \
                       until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_pump,
            }),
    )
}

/// `+1`: attach this Equipment to up to one target creature you control.
fn plus_one_attach(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        // "up to one" — choosing zero is a clean no-op.
        return Vec::new();
    };
    vec![Effect::Attach {
        equipment_or_aura: ctx.source,
        target: *id,
    }]
}

/// `−2`: pump the equipped creature — GAP.
fn minus_two_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature gets +2/+2 and double strike" needs the
    // id of the creature this Equipment is attached to; no demonstrated
    // resolver accessor for the attachment target.
    Vec::new()
}
