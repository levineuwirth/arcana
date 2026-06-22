//! Rustvine Cultivator — `{G}` 1/2 Phyrexian Elf Druid.
//! `{T}: Put an oil counter on this creature.`
//! `{T}, Remove an oil counter from this creature: Untap target land.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rustvine Cultivator");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let oil = reg.interner_mut().intern("oil");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put an oil counter on this creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_oil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove an oil counter from this creature: Untap target land."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(oil), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_land,
            }),
    )
}

fn add_oil(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = reg
        .interner()
        .lookup("oil")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: 1,
    }]
}

fn untap_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Untap { target: *id }]
}
