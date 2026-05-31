//! Bumi's Feast Lecture — `{1}{G}` Sorcery — Lesson. "Create a Food
//! token. Then earthbend X, where X is twice the number of Foods you
//! control."
//!
//! The Food token is minted via the canonical commodity-token
//! primitive. The "earthbend X" rider (target a land you control,
//! animate it into a 0/0 haste creature that's still a land, put X
//! +1/+1 counters on it where X is twice the number of Foods you
//! control, and give it a dies/exiled-return delayed trigger) is not
//! expressible: there is no earthbend / land-animate-with-return
//! primitive, and the +1/+1-counter target is the chosen land, not a
//! spell target. The Food half is faithful; the earthbend half is
//! gapped rather than emitting a wrong fixed-size stand-in.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bumi's Feast Lecture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a Food token. Then earthbend X, where X is twice \
                   the number of Foods you control."
                .into(),
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
    // GAP: "earthbend X" (animate target land you control into a 0/0
    // haste creature that's still a land, put X = twice the number of
    // Foods you control +1/+1 counters on it, and return it tapped
    // when it dies or is exiled) has no engine primitive. The Food
    // token is created faithfully below.
    vec![Effect::CreateCommodityToken {
        controller: entry.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
