//! Jermane, Pride of the Circus — `{G}{G}` 2/3 Legendary Creature —
//! Cat Performer.
//!
//! {G}, {T}: Put two +1/+1 counters on target creature with four or more legs.
//! {1}{G}{G}, {T}: Until end of turn, all creatures able to block target
//! creature with four or more legs do so.
//!
//! "with four or more legs" has no ObjectFilter attribute, so both abilities
//! target any creature (the legs restriction is a documented fidelity gap on
//! the target filter).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jermane, Pride of the Circus");
    let cat = reg.interner_mut().intern("Cat");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(performer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, {T}: Put two +1/+1 counters on target creature with four or more legs."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP (fidelity): "with four or more legs" — no legs filter.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}{G}, {T}: Until end of turn, all creatures able to block target \
                       creature with four or more legs do so."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP (fidelity): "with four or more legs" — no legs filter.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lure_target,
            }),
    )
}

fn add_two_counters(
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
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn lure_target(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "all creatures able to block target creature do so" (Lure/
    //      must-block) — no must-be-blocked effect in the engine catalog.
    Vec::new()
}
