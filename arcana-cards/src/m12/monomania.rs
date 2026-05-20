//! Monomania — `{3}{B}{B}` sorcery. "Target player chooses a card in
//! their hand and discards the rest." Discarding hand-minus-one is
//! dynamic on the target's hand size; we make the target discard
//! (hand_size - 1) cards, chosen by them.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monomania");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player chooses a card in their hand and discards the rest.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    let hand = script::hand_size(state, *p);
    let to_discard = hand.saturating_sub(1);
    if to_discard == 0 {
        return Vec::new();
    }
    vec![Effect::Discard {
        player: *p,
        count: to_discard,
        choice: DiscardChoice::ControllerChooses,
    }]
}
