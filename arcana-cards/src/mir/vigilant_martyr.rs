//! Vigilant Martyr — `{W}` 1/1 white Human Cleric.
//!
//! Oracle:
//! * `Sacrifice this creature:` Regenerate target creature.
//! * `{W}{W}, {T}, Sacrifice this creature:` Counter target spell that
//!   targets an enchantment.
//!
//! Implemented:
//! * Both activated abilities. The "that targets an enchantment" restriction
//!   on the counter target is not expressible (ObjectFilter can't constrain
//!   a spell by what IT targets), so the second ability targets any spell
//!   (TargetFilter::Spell) — documented below.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vigilant Martyr");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: Regenerate target creature.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{W}, {T}, Sacrifice this creature: Counter target spell that targets an enchantment.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{W}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                // GAP: "that targets an enchantment" — ObjectFilter can't
                // constrain a spell by what IT targets; counters any spell.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::new()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: counter_target_spell,
            }),
    )
}

fn regenerate_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Regenerate { target: *id }]
}

fn counter_target_spell(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Counter { target: *id }]
}
