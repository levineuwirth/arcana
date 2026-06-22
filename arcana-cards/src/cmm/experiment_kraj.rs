//! Experiment Kraj — `{2}{G}{G}{U}{U}` 4/6 Legendary Ooze Mutant.
//!
//! Oracle:
//! * "Experiment Kraj has all activated abilities of each other creature
//!   with a +1/+1 counter on it." — a static ability-granting continuous
//!   effect; not a triggered/activated ability and not expressible. GAP'd.
//! * "{T}: Put a +1/+1 counter on target creature." — a tap activation
//!   adding a +1/+1 counter to a targeted creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Experiment Kraj");
    let ooze = reg.interner_mut().intern("Ooze");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static "has all activated abilities of each other creature with
    // a +1/+1 counter on it" — a continuous ability-granting static, not a
    // triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Put a +1/+1 counter on target creature.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter,
        }),
    )
}

fn add_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
