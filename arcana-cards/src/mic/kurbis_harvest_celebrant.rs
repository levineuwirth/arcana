//! Kurbis, Harvest Celebrant — `{X}{G}{G}` 0/0 Legendary Treefolk (green).
//!
//! * GAP (ETB counters): "Kurbis enters with a number of +1/+1 counters on it
//!   equal to the amount of mana spent to cast it." — there is no accessor
//!   for "mana spent to cast" (the X value) at resolution and no enters-with
//!   characteristic field, so the counter count can't be computed.
//! * "Remove a +1/+1 counter from Kurbis: Prevent all damage that would be
//!   dealt this turn to another target creature with a +1/+1 counter on it."
//!   → an activated ability (remove a +1/+1 counter as the cost) targeting a
//!   creature that has a +1/+1 counter, preventing all damage to it this
//!   turn. GAP: the "another" self-exclusion (no self-exclude target field).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kurbis, Harvest Celebrant");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a +1/+1 counter from Kurbis: Prevent all damage \
                   that would be dealt this turn to another target creature \
                   with a +1/+1 counter on it."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter {
                    has_counter: Some(CounterKind::PlusOnePlusOne),
                    ..ObjectFilter::creature()
                }),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: prevent_damage_to_target,
        }),
    )
}

fn prevent_damage_to_target(
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
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
