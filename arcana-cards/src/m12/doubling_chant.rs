//! Doubling Chant — `{5}{G}` sorcery, "For each creature you control, you may search your
//! library for a creature card with the same name as that creature. Put those cards onto the
//! battlefield, then shuffle."
//!
//! Implementation: iterate over all creatures you control, capture each creature's name, and
//! emit a name-filtered TutorToBattlefield for each. The "you may" optional aspect is simplified
//! (treated as unconditional per creature — the player may decline by not picking a card during
//! the search, which the engine allows).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doubling Chant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each creature you control, you may search your library for a creature card with the same name as that creature. Put those cards onto the battlefield, then shuffle.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let my_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = Vec::new();
    for id in my_creatures {
        if let Some(nm) = script::name_of(state, id) {
            effects.push(Effect::TutorToBattlefield {
                player: entry.controller,
                filter: ObjectFilter { name: Some(nm), ..ObjectFilter::creature() },
                tapped: false,
            });
        }
    }
    effects
}
