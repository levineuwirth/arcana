//! Armor Thrull — `{2}{B}` 1/3 black Thrull.
//! "{T}, Sacrifice this creature: Put a +1/+2 counter on target creature."
//! The +1/+2 counter is wired as `CounterKind::Named("+1/+2")` plus a
//! permanent +1/+2 continuous effect for its P/T contribution (layer 7d
//! only sums +1/+1 / -1/-1 counters).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armor Thrull");
    let thrull = reg.interner_mut().intern("Thrull");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("+1/+2");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this creature: Put a +1/+2 counter on target creature.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_add_armor_counter,
            }),
    )
}

fn sac_add_armor_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(kind) = reg.interner().lookup("+1/+2").map(CounterKind::Named) else {
        return Vec::new();
    };
    // Named "+1/+2" counter plus a permanent +1/+2 continuous effect for
    // its P/T contribution; narrowed GAP: removing the counter later
    // would not remove the P/T boost.
    vec![
        Effect::AddCounters { target: *id, kind, count: 1 },
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 2,
            duration: Duration::Permanent,
            keywords: vec![],
        },
    ]
}
