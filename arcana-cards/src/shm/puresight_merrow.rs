//! Puresight Merrow — `{W/U}{W/U}` 2/2 Merfolk Wizard.
//! `{W/U}, {Q}: Look at the top card of your library. You may exile that card.`
//! ({Q} is the untap symbol — the cost untaps this creature.)
//! GAP: No ActivationCost field for untap-self as cost; also "look at top card and maybe exile" GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Puresight Merrow");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W/U}, {Q}: Look at the top card of your library. You may exile that card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W/U}").unwrap(),
                    // GAP: no untap-self cost field ({Q} symbol)
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_and_maybe_exile,
            }),
    )
}

fn look_and_maybe_exile(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: No Effect for "look at top card and optionally exile it".
    // GAP: ActivationCost has no untap-self ({Q}) field.
    Vec::new()
}
