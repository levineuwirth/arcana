//! Quicksilver Elemental — `{3}{U}{U}` 3/4 blue Elemental.
//!
//! `{U}`: This creature gains all activated abilities of target creature
//! until end of turn. (Effect is GAP'd — no "gain all activated
//! abilities" primitive; the activation shape + target are emitted.)
//! You may spend blue mana as though it were mana of any color to pay
//! the activation costs of this creature's abilities. (GAP — no
//! mana-substitution static.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quicksilver Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "You may spend blue mana as though it were mana of any color to
    // pay the activation costs of this creature's abilities." — no
    // mana-substitution static.

    reg.register(
        CardDefinition::new(name, chars)
            // "{U}: This creature gains all activated abilities of target
            // creature until end of turn."
            // GAP: no "gain all activated abilities of target creature"
            // effect exists; the activation shape + target are emitted with
            // an empty effect.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: This creature gains all activated abilities of target creature until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_all_abilities,
            }),
    )
}

fn gain_all_abilities(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: copying all activated abilities of the target is not expressible.
    Vec::new()
}
