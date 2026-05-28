//! Demon's Herald — `{B}` 1/1 black Human Wizard.
//! `{2}{B}, {T}, Sacrifice a blue creature, a black creature, and a
//! red creature: Search your library for a card named Prince of
//! Thralls, put it onto the battlefield, then shuffle.`
//!
//! GAP: "sacrifice three specific-color creatures" — ActivationCost
//! cannot express sacrificing multiple distinct permanents by color.
//! Search by name ("Prince of Thralls") also not expressible.

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
    let name = reg.interner_mut().intern("Demon's Herald");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
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
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}, Sacrifice a blue creature, a black creature, and a red creature: Search your library for a card named Prince of Thralls, put it onto the battlefield, then shuffle.".into(),
                // GAP: sacrificing three specific-color creatures not expressible.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: search_prince_of_thralls,
            }),
    )
}

fn search_prince_of_thralls(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: search by name not expressible with TutorToBattlefield filter.
    Vec::new()
}
