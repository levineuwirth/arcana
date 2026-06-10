//! Ash Barrens — nonbasic land.
//! "{T}: Add {C}." and "Basic landcycling {1}". Per the keyword
//! conventions, the landcycling variant is emitted as the generic
//! `Cycling` keyword with its printed cost — the engine synthesizes the
//! "[cost], discard this card: draw a card" activation; the basic-land
//! SEARCH variant of the cycling effect is a documented fidelity gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ash Barrens");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        // GAP (fidelity): 'Basic landcycling {1}' searches for a basic
        // land card; the generic Cycling keyword draws a card instead —
        // the type-search variant is not separately modeled.
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{1}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            },
        ),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
