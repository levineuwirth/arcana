//! Necromantic Selection — `{4}{B}{B}{B}` sorcery, "Destroy all creatures,
//! then return a creature card put into a graveyard this way to the battlefield
//! under your control. It's a black Zombie in addition to its other colors and
//! types. Exile Necromantic Selection."
//! The "return one of the destroyed creatures" and the color/type modifier
//! ("black Zombie in addition to its other types") are not expressible.
//! The self-exile is also not expressible (no Effect for exiling the resolving
//! spell itself).
//!
//! # GAP: choose-from-newly-dead creatures to return to battlefield
//! # GAP: add color/type modifier ("becomes black Zombie in addition to…")
//! # GAP: self-exile on resolution

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necromantic Selection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures, then return a creature card put into a graveyard this way to the battlefield under your control. It's a black Zombie in addition to its other colors and types. Exile Necromantic Selection.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: choose-from-newly-dead creatures to return to battlefield
    // GAP: add color/type modifier ("becomes black Zombie in addition to…")
    // GAP: self-exile on resolution
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
