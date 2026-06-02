//! Explosive Revelation — `{3}{R}{R}` sorcery. "Choose any target. Reveal
//! cards from the top of your library until you reveal a nonland card.
//! Explosive Revelation deals damage equal to that card's mana value to that
//! permanent or player. Put the nonland card into your hand and the rest on
//! the bottom of your library in any order."
//!
//! The reveal-until-nonland half (card to hand, rest to bottom) is expressed
//! with `Effect::RevealUntil`. The damage half is DYNAMIC — it equals the
//! revealed nonland card's mana value, a quantity discovered only mid-reveal
//! that none of the `script::*` helpers can read — and `RevealUntil` does not
//! emit the damage. Per the dynamic-amount rule, that half is GAP'd rather
//! than hardcoded.

use arcana_core::effects::{Effect, DigRest, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Explosive Revelation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose any target. Reveal cards from the top of your library until you reveal a nonland card. Explosive Revelation deals damage equal to that card's mana value to that permanent or player. Put the nonland card into your hand and the rest on the bottom of your library in any order.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    // GAP: the damage equals the revealed nonland card's mana value, a dynamic
    // amount discovered only during the reveal that no script helper can read,
    // and `RevealUntil` itself does not deal damage. Only the reveal-to-hand /
    // rest-to-bottom half is expressible.
    vec![Effect::RevealUntil {
        player: entry.controller,
        filter: ObjectFilter::new().without_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Hand,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}
