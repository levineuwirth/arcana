//! Beanstalk Giant // Fertile Footsteps — `{6}{G}` / `{2}{G}` Adventure
//!
//! Creature: `{6}{G}` Creature — Giant (*/*)
//!   Power and toughness each equal to the number of lands you control.
//!   (GAP: dynamic P/T based on land count — no continuous-effect engine
//!   for "P/T = count"; emitting 0/0 as placeholder.)
//!
//! Adventure: `{2}{G}` Sorcery — Fertile Footsteps
//!   Search your library for a basic land card, put it onto the battlefield,
//!   then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beanstalk Giant");
    let giant_sub = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: dynamic P/T; 0/0 placeholder
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Fertile Footsteps");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Search your library for a basic land card, put it onto the battlefield, then shuffle.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        tapped: false,
    }]
}
