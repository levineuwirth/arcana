//! Dulcet Sirens — `{2}{U}` 1/3 Siren.
//! "{U}, {T}: Target creature attacks target opponent this turn if able."
//!   (modeled with Goad — "must attack each combat, can't attack you";
//!   the "attacks THAT specific opponent" steering is a fidelity GAP.)
//! "Morph {U}" — Morph is not an expressible KeywordAbility (GAP).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dulcet Sirens");
    let siren = reg.interner_mut().intern("Siren");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Morph {U} is not an expressible KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, {T}: Target creature attacks target opponent this turn if able.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_player(),
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: force_attack,
        }),
    )
}

fn force_attack(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "attacks target opponent" steering — Goad forces an attack but
    // can't pin it to the chosen opponent (it just can't attack the goader).
    vec![Effect::Goad {
        target: *id,
        goader: ctx.controller,
        duration: Duration::EndOfTurn,
    }]
}
