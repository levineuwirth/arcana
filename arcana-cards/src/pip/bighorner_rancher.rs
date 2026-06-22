//! Bighorner Rancher — `{4}{G}` 2/5 Human Ranger with Vigilance.
//!
//! * Vigilance — keyword.
//! * "{T}: Add an amount of {G} equal to the greatest power among creatures
//!   you control." — tap mana ability; amount computed at resolution.
//! * "Sacrifice this creature: You gain life equal to the greatest
//!   toughness among other creatures you control." — sacrifice-self
//!   activated ability; amount computed at resolution.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bighorner Rancher");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add an amount of {G} equal to the greatest power among creatures you control.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_greatest_power,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature: You gain life equal to the greatest toughness among other creatures you control.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_life_greatest_toughness,
            }),
    )
}

fn add_green_greatest_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let n = ids
        .iter()
        .map(|id| script::power_of(state, *id).max(0))
        .max()
        .unwrap_or(0) as usize;
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); n],
    }]
}

fn gain_life_greatest_toughness(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let n = ids
        .iter()
        .filter(|id| **id != ctx.source)
        .map(|id| script::toughness_of(state, *id).max(0))
        .max()
        .unwrap_or(0) as u32;
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: n,
    }]
}
