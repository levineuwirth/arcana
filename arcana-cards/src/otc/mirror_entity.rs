//! Mirror Entity — `{2}{W}` 1/1 Shapeshifter with Changeling.
//! "{X}: Until end of turn, creatures you control have base power and
//! toughness X/X and gain all creature types."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirror Entity");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}: Until end of turn, creatures you control have base power and toughness X/X and gain all creature types.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: anthem_x,
        }),
    )
}

fn anthem_x(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the X-scaled board pump "creatures you control have base P/T X/X
    // and gain all creature types." The generic-{X} announced value is NOT
    // threaded into ActivationContext::x_value for activated abilities (only
    // the planeswalker remove_loyalty_x path populates it), so X is
    // unreadable here. Emitting a literal would be a materially wrong card,
    // so the whole effect is gapped while the {X} cost is recorded faithfully.
    Vec::new()
}
