//! Devour Intellect — `{B}` sorcery, "Target opponent discards a card.
//! If mana from a Treasure was spent to cast this spell, instead that
//! player reveals their hand, you choose a nonland card, then discards
//! that card." The Treasure-spent conditional / caster-chosen discard
//! is not expressible; the base discard is.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devour Intellect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target opponent discards a card. If mana from a Treasure \
                   was spent to cast this spell, instead that player \
                   reveals their hand, you choose a nonland card from it, \
                   then that player discards that card."
                .into(),
            target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: the "if Treasure mana was spent" upgraded caster-chosen
    // discard is not expressible.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
