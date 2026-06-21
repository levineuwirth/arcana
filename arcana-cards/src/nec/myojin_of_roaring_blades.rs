//! Myojin of Roaring Blades — `{5}{R}{R}{R}` 7/4 Legendary Creature — Spirit.
//!
//! * "Enters with an indestructible counter on it if you cast it from your
//!   hand." — GAP: a cast-from-hand-conditional ETB counter is not expressible
//!   (no enters-with-counter effect nor a cast-zone predicate in the surface).
//! * "Remove an indestructible counter from Myojin of Roaring Blades: It deals
//!   7 damage to each of up to three targets." — wired: an activated ability
//!   whose cost removes a Named("indestructible") counter, dealing 7 to each
//!   chosen target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Roaring Blades");
    let spirit = reg.interner_mut().intern("Spirit");
    let indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "enters with an indestructible counter if cast from hand".
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove an indestructible counter from Myojin of Roaring Blades: It deals 7 damage to each of up to three targets.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(indestructible), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_seven_to_each,
            }),
    )
}

fn deal_seven_to_each(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .map(|t| {
            let dt = match t {
                TargetChoice::Object(id) => DamageTarget::Object(*id),
                TargetChoice::Player(p) => DamageTarget::Player(*p),
                TargetChoice::ObjectOrPlayer(o) => match o {
                    ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                    ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
                },
            };
            Effect::DealDamage {
                source: ctx.source,
                target: dt,
                amount: 7,
            }
        })
        .collect()
}
