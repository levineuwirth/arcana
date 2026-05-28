//! Keeper of the Crown // Coronation of the Wilds — `{2}{L}` / `{2}{G}` Adventure
//!
//! Creature: `{2}{L}` Creature — Human Noble (3/4)
//!   ({L} can be paid with one mana from a legendary source — not modeled.)
//!   Other legendary creatures you control get +1/+1 and have indestructible.
//!   (GAP: static buff/keyword-grant to other legendaries not modeled.)
//!
//! Adventure: `{2}{G}` Sorcery — Coronation of the Wilds
//!   Target creature you control becomes a legendary Noble in addition to its
//!   other types and gains "{T}: Add one mana of any color." Draw a card.
//!   (GAP: type-granting, supertype-granting, and activated-ability-granting
//!   not expressible; emitting DrawCards as best effort.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keeper of the Crown");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(noble_sub);
    // {L} hybrid-legendary cost; closest approximate is {2}{G}
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Coronation of the Wilds");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target creature you control becomes a legendary Noble in addition to its other types and gains \"{T}: Add one mana of any color.\" Draw a card.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
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
    // GAP: type-granting, supertype-granting, and activated-ability-granting
    // not expressible; emitting only DrawCards
    vec![Effect::DrawCards {
        player: entry.controller,
        count: 1,
    }]
}
