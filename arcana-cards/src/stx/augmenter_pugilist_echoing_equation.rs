//! Augmenter Pugilist // Echoing Equation
//!
//! Front: Creature — Troll Druid {1}{G}{G} 3/3 (green)
//!   Trample
//!   As long as you control eight or more lands, this creature gets +5/+5.
//! Back: Sorcery
//!   Choose target creature you control. Each other creature you control becomes a copy of it until end of turn, except those creatures aren't legendary.
//! GAP: Conditional static P/T boost based on land count not modeled
//! GAP: "Each other creature becomes a copy of target creature" not in Effect catalog

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Augmenter Pugilist");
    let back_name = reg.interner_mut().intern("Echoing Equation");
    let troll = reg.interner_mut().intern("Troll");
    let druid = reg.interner_mut().intern("Druid");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(troll);
    subtypes.insert(druid);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let back_ability = SpellAbilityDef {
        text: "Choose target creature you control. Each other creature you control becomes a copy of it until end of turn, except those creatures aren't legendary.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: back_resolve,
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: Some(back_ability),
        }),
    )
}

fn back_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each other creature becomes a copy of target creature until end of turn" not in Effect catalog
    Vec::new()
}
