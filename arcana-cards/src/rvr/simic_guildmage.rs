//! Simic Guildmage — `{G/U}{G/U}` 2/2 Elf Wizard.
//! {1}{G}: Move a +1/+1 counter from target creature onto another
//!         target creature with the same controller.
//! {1}{U}: Attach target Aura attached to a permanent to another
//!         permanent with the same controller.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Simic Guildmage");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}: Move a +1/+1 counter from target creature onto another target creature with the same controller.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: move_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: Attach target Aura attached to a permanent to another permanent with the same controller.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reattach_aura,
            }),
    )
}

fn move_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect to MOVE a +1/+1 counter from one creature onto another;
    // RemoveCounters + AddCounters cannot atomically express "the same counter"
    // between two distinct chosen targets within the demonstrated API.
    Vec::new()
}

fn reattach_aura(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: re-attaching an already-attached Aura to a different chosen
    // permanent (Effect::Attach attaches THIS source, not an arbitrary aura
    // target) is not expressible with the demonstrated API.
    Vec::new()
}
