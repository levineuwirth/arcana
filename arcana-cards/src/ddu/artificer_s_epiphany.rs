//! Artificer's Epiphany — `{2}{U}` instant. "Draw two cards. If you
//! control no artifacts, discard a card."
//!
//! Conditional discard wired via count_matching over artifacts you
//! control: discard only if that count is zero.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artificer's Epiphany");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards. If you control no artifacts, discard a card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let artifacts = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = vec![Effect::DrawCards { player: entry.controller, count: 2 }];
    if artifacts == 0 {
        effects.push(Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects
}
