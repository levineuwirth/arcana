//! Dark Bargain — `{3}{B}` instant, "Look at the top three cards of your
//! library. Put two of them into your hand and the rest into your graveyard.
//! Dark Bargain deals 2 damage to you."
//!
//! GAP: look top N, choose M to keep and put rest to graveyard (selective draw)
//! is not expressible with the current Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Bargain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top three cards of your library. Put two of them into your hand and the rest into your graveyard. Dark Bargain deals 2 damage to you.".into(),
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
    // GAP: selective look-top-N and put chosen into hand/graveyard not expressible
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(entry.controller),
        amount: 2,
    }]
}
