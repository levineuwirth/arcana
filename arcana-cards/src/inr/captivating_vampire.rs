//! Captivating Vampire — `{1}{B}{B}` 2/2 Vampire.
//! "Other Vampire creatures you control get +1/+1."; "Tap five untapped Vampires
//!  you control: Gain control of target creature. It becomes a Vampire in
//!  addition to its other types."
//!
//! The "+1/+1" anthem is a pure static continuous ability (GAP). The activated
//! ability taps five untapped Vampires you control (tap_other + tap_other_count)
//! and gains permanent control of the target. "It becomes a Vampire" adds a
//! subtype, which Effect::AddType (TypeLine only) cannot express — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captivating Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    // GAP: static anthem — "other Vampire creatures you control get +1/+1".

    let vampire_filter = script::subtype_filter(reg, "Vampire");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Tap five untapped Vampires you control: Gain control of target creature. It becomes a Vampire in addition to its other types.".into(),
            cost: ActivationCost {
                tap_other: Some(vampire_filter),
                tap_other_count: 5,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_control,
        }),
    )
}

fn gain_control(
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
    // GAP: "it becomes a Vampire" adds a subtype, not expressible via AddType.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
