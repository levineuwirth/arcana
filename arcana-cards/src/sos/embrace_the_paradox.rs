//! Embrace the Paradox — `{3}{G}{U}` instant. "Draw three cards. You may put
//! a land card from your hand onto the battlefield tapped."
//!
//! GAP: "You may put a land card from your hand onto the battlefield tapped"
//! requires an optional player choice (put from hand to battlefield tapped),
//! which is not in the Effect catalog. The draw is rendered; the optional
//! land-drop from hand is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embrace the Paradox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. You may put a land card from your hand onto the battlefield tapped.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: No effect for optional "put a land from hand onto battlefield tapped".
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
