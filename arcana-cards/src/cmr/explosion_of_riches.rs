//! Explosion of Riches — `{5}{R}` sorcery. "Draw a card, then each other
//! player may draw a card. Whenever a card is drawn this way, Explosion
//! of Riches deals 5 damage to target opponent chosen at random from
//! among your opponents." The 'whenever a card is drawn this way'
//! trigger isn't expressible with the catalog (no per-draw on-resolution
//! trigger primitive). We emit the deterministic draws and gap the
//! triggered damage.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Explosion of Riches");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card, then each other player may draw a card. Whenever a card is drawn this way, Explosion of Riches deals 5 damage to target opponent chosen at random from among your opponents.".into(),
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
    let mut effects = vec![Effect::DrawCards {
        player: entry.controller,
        count: 1,
    }];
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::DrawCards { player: opp, count: 1 });
    }
    // GAP: 'whenever a card is drawn this way' damage rider — no
    // ad-hoc per-event trigger primitive on a one-shot resolution.
    effects
}
