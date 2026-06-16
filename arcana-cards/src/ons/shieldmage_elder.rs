//! Shieldmage Elder — `{5}{W}` 2/3 Human Cleric Wizard.
//! "Tap two untapped Clerics you control: Prevent all damage target creature
//!   would deal this turn." — the tap-two-Clerics cost and the creature
//!   target are wired; the effect (prevent damage a *source* would deal) is
//!   GAP'd, as PreventDamage only prevents damage dealt *to* a target.
//! "Tap two untapped Wizards you control: Prevent all damage target spell
//!   would deal this turn." — likewise GAP'd at the effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shieldmage Elder");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let cleric_filter = script::subtype_filter(reg, "Cleric")
        .controlled_by(ControllerConstraint::You);
    let wizard_filter = script::subtype_filter(reg, "Wizard")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Clerics you control: Prevent all damage \
                       target creature would deal this turn."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(cleric_filter),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_creature_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped Wizards you control: Prevent all damage \
                       target spell would deal this turn."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(wizard_filter),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::new()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_spell_damage,
            }),
    )
}

fn prevent_creature_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Prevent all damage target creature would deal this turn" — no
    // effect prevents damage a chosen SOURCE would deal (only damage dealt
    // TO a target / by a source-filter).
    Vec::new()
}

fn prevent_spell_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Prevent all damage target spell would deal this turn" — not
    // expressible (no per-spell damage-prevention effect).
    Vec::new()
}
