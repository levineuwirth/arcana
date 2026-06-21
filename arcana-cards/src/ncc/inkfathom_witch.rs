//! Inkfathom Witch — `{1}{U/B}` 1/1 Merfolk Wizard with Fear.
//!
//! * Fear (keyword).
//! * `{2}{U}{B}: Each unblocked creature has base power and toughness
//!   4/1 until end of turn.`

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
    let name = reg.interner_mut().intern("Inkfathom Witch");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{B}: Each unblocked creature has base power and toughness 4/1 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_unblocked_pt,
            }),
    )
}

fn set_unblocked_pt(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each unblocked creature" — no documented ObjectFilter for combat
    // unblocked-status to enumerate the affected creatures.
    Vec::new()
}
