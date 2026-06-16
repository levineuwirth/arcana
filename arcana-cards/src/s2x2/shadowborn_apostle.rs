//! Shadowborn Apostle — `{B}` 1/1 black Human Cleric.
//! "A deck can have any number of cards named Shadowborn Apostle.
//!  {B}, Sacrifice six creatures named Shadowborn Apostle: Search your library
//!  for a Demon creature card, put it onto the battlefield, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadowborn Apostle");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // "A deck can have any number..." is a deckbuilding rule with no in-game
    // effect — nothing to model.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let apostle_name = reg.interner().lookup("Shadowborn Apostle");

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}, Sacrifice six creatures named Shadowborn Apostle: Search your library for a Demon creature card, put it onto the battlefield, then shuffle.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                // GAP: cost requires sacrificing SIX named creatures; sacrifice_other
                // has no count field and sacrifices a single matching permanent, so
                // the six-count is under-modeled (one named-Apostle sacrifice).
                sacrifice_other: Some(ObjectFilter {
                    name: apostle_name,
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: fetch_demon,
        }),
    )
}

fn fetch_demon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: arcana_core::script::subtype_filter(reg, "Demon")
            .with_types(TypeLine::CREATURE.into()),
        tapped: false,
    }]
}
