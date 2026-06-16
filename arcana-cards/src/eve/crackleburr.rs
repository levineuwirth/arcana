//! Crackleburr — `{1}{U/R}{U/R}` 2/2 Elemental.
//! {U/R}{U/R}, {T}, Tap two untapped red creatures you control: This
//! creature deals 3 damage to any target.
//! {U/R}{U/R}, {Q}, Untap two tapped blue creatures you control: Return
//! target creature to its owner's hand.
//!
//! Ability 1's "Tap two untapped red creatures you control" maps to
//! `tap_other`. Ability 2's {Q} (untap symbol) and "Untap two tapped blue
//! creatures" cost components have no `ActivationCost` fields — GAP those
//! pieces; the {U/R}{U/R} mana cost and bounce effect are emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crackleburr");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U/R}{U/R}, {T}, Tap two untapped red creatures you control: This creature deals 3 damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U/R}{U/R}").expect("valid cost"),
                    tap: true,
                    tap_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        colors: Some(ColorSet::red()),
                        ..ObjectFilter::default()
                    }),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U/R}{U/R}, {Q}, Untap two tapped blue creatures you control: Return target creature to its owner's hand.".into(),
                // GAP: {Q} (untap-symbol) cost + "Untap two tapped blue creatures" cost component.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U/R}{U/R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_creature,
            }),
    )
}

fn deal_three(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 3,
    }]
}

fn bounce_creature(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
