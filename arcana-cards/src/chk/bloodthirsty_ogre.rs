//! Bloodthirsty Ogre — `{2}{B}` 3/1 Creature — Ogre Warrior Shaman.
//! {T}: Put a devotion counter on this creature.
//! {T}: Target creature gets -X/-X until end of turn, where X is the
//! number of devotion counters on this creature. Activate only if you
//! control a Demon.
//!
//! "Devotion counter" is a named counter (`CounterKind::Named("devotion")`).
//! The second ability's "only if you control a Demon" gate is enforced via
//! `ActivationCost::activation_condition` (you control a Demon); X is read
//! at resolution as the count of devotion counters on this creature.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodthirsty Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);
    subtypes.0.insert(shaman);
    // Pre-intern the devotion-counter name + the "Demon" subtype so the
    // resolver / activation-condition closures can look them up.
    let _devotion = reg.interner_mut().intern("devotion");
    let _demon = reg.interner_mut().intern("Demon");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put a devotion counter on this creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_devotion_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature gets -X/-X until end of turn, where X is the number of devotion counters on this creature. Activate only if you control a Demon.".into(),
                cost: ActivationCost {
                    tap: true,
                    activation_condition: Some(|s, _src, you, reg| {
                        let demon = arcana_core::script::subtype_filter(reg, "Demon");
                        arcana_core::conditions::you_control_a(s, you, &demon)
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_x,
            }),
    )
}

fn add_devotion_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(devotion) = reg.interner().lookup("devotion") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Named(devotion),
        count: 1,
    }]
}

fn minus_x_x(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(devotion) = reg.interner().lookup("devotion") else {
        return Vec::new();
    };
    let x = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::Named(devotion)));
    vec![Effect::Pump {
        target: *id,
        power: -(x as i32),
        toughness: -(x as i32),
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
