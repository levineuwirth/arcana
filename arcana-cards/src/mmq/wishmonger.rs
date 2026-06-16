//! Wishmonger — `{3}{W}` 3/3 white Unicorn Monger.
//! "{2}: Target creature gains protection from the color of its controller's choice until end of turn.
//! Any player may activate this ability."
//!
//! GAP: Protection is not in the KeywordAbility set. Also "any player may activate" is not
//! expressible in ActivatedAbilityDef.

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
    let name = reg.interner_mut().intern("Wishmonger");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let monger = reg.interner_mut().intern("Monger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);
    subtypes.0.insert(monger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Target creature gains protection from the color of its controller's choice until end of turn. Any player may activate this ability.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_protection,
            }),
    )
}

fn grant_protection(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Protection keyword not in KeywordAbility set; color choice not expressible.
    Vec::new()
}
