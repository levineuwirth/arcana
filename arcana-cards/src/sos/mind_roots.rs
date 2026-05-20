//! Mind Roots — `{1}{B}{G}` sorcery. "Target player discards two
//! cards. Put up to one land card discarded this way onto the
//! battlefield tapped under your control." The discard is
//! expressible; recovering a land from the just-discarded cards has
//! no primitive (GAP-noted, partial).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mind Roots");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player discards two cards. Put up to one land card discarded this way onto the battlefield tapped under your control.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put up to one land card discarded this way onto the
    // battlefield" — no primitive references the just-discarded cards.
    let Some(target) = _entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::OpponentChooses,
    }]
}
