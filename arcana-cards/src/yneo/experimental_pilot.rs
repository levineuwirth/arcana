//! Experimental Pilot — `{U}` 1/2 Human Pilot with Ward {2}.
//! "{U}, Discard two cards: Draft a card from Experimental Pilot's spellbook."
//!   (spellbook draft is an Alchemy/Arena mechanic — effect GAP'd)
//! "Experimental Pilot crews Vehicles as though its power were 2 greater."
//!   (static — GAP'd)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Experimental Pilot");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: static "crews Vehicles as though its power were 2 greater" not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, Discard two cards: Draft a card from Experimental Pilot's spellbook."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                discard_other: Some(ObjectFilter::default()),
                discard_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draft_from_spellbook,
        }),
    )
}

fn draft_from_spellbook(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Draft a card from a spellbook" is an Alchemy/Arena-only mechanic
    // (no Effect::Conjure / spellbook draft). Cost is faithful; effect omitted.
    Vec::new()
}
