//! Beacon of Creation — `{3}{G}` sorcery. "Create a 1/1 green Insect
//! creature token for each Forest you control. Shuffle Beacon of Creation
//! into its owner's library."
//!
//! GAP: self-shuffle into library not expressible.
//! Counts Forest lands controlled using subtype filter + ids_matching.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beacon of Creation");
    let _insect = reg.interner_mut().intern("Insect");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 1/1 green Insect creature token for each Forest you control. Shuffle Beacon of Creation into its owner's library.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: self-shuffle into library not expressible
    let insect = reg.interner().lookup("Insect").expect("Insect interned during register()");
    let forest_filter = script::subtype_filter(reg, "Forest")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &forest_filter, entry.controller);
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let token = TokenDefinition {
        name: insect,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n).map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() }).collect()
}
