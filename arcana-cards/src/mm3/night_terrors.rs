//! Night Terrors — `{2}{B}` sorcery.
//! "Target player reveals their hand. You choose a nonland card from it.
//! Exile that card."
//!
//! # GAP: "reveal opponent's hand and you choose a nonland card to exile"
//! (targeted discard with controller choice from target's hand) — the
//! Effect::Discard catalog uses DiscardChoice variants but does not expose
//! a "view-and-choose" reveal mechanic. Best effort: use Discard with
//! OpponentChooses as a placeholder; actual hand-reveal exile is a GAP.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Night Terrors");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player reveals their hand. You choose a nonland card from it. Exile that card.".into(),
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
    // GAP: hand-reveal + controller-selects-target's-nonland-card-to-exile not expressible.
    // Approximate with OpponentChooses discard (weaker).
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let player = match target {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    vec![Effect::Discard { player, count: 1, choice: DiscardChoice::OpponentChooses }]
}
