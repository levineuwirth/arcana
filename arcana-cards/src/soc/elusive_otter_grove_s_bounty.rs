//! Elusive Otter // Grove's Bounty — `{U}` / `{X}{G}` Adventure
//!
//! Creature: `{U}` Creature — Otter (1/1)
//!   Prowess. (GAP: Prowess not in engine keyword list; emitting keywords:vec![].)
//!   Creatures with power less than this creature's power can't block it.
//!   (GAP: static "can't be blocked by lower-power" not modeled.)
//!
//! Adventure: `{X}{G}` Sorcery — Grove's Bounty
//!   Distribute X +1/+1 counters among any number of target creatures you
//!   control. (GAP: X is an integer chosen at cast; can't model X; emitting
//!   Vec::new().)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elusive Otter");
    let otter_sub = reg.interner_mut().intern("Otter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Prowess keyword not in engine
        keywords: vec![],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Grove's Bounty");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Distribute X +1/+1 counters among any number of target creatures you control.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            ),
            count: TargetCount::Any,
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
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X value not accessible from StackEntry; distribute counters not
    // expressible
    Vec::new()
}
