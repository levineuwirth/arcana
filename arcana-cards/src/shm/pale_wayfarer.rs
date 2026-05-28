//! Pale Wayfarer — `{5}{W}{W}` 4/4 white Spirit Giant. "{2}{W}{W}, {Q}: Target
//! creature gains protection from the color of its controller's choice until
//! end of turn. ({Q} is the untap symbol.)"
//!
//! GAP: "protection from the color of its controller's choice" — player choice
//! of color not expressible. Also {Q} untap cost not supported.

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
    let name = reg.interner_mut().intern("Pale Wayfarer");
    let spirit = reg.interner_mut().intern("Spirit");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}{W}, {Q}: Target creature gains protection from chosen color until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                // GAP: {Q} untap cost not supported; using mana-only.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: protection_gap,
            }),
    )
}

fn protection_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "protection from color of controller's choice" not expressible.
    Vec::new()
}
