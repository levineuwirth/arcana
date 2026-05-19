//! Binding Negotiation — `{1}{B}` sorcery. "Target opponent reveals their hand.
//! You may choose a nonland card from it. If you do, they discard it.
//! Otherwise, you may put a face-up exiled card they own into their graveyard."
//!
//! GAP: hand-reveal + caster-chooses-specific-card-to-discard (targeted
//! discard of a card chosen by the caster from opponent's hand).
//! GAP: put face-up exiled card into graveyard (no Effect for moving exile to
//! graveyard).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Binding Negotiation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You may choose a nonland card from it. If you do, they discard it. Otherwise, you may put a face-up exiled card they own into their graveyard.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: hand-reveal + caster-chooses-which-card-to-discard (OpponentChooses is wrong here — it should be controller chooses a card from opponent's hand)
    // GAP: put face-up exiled card into graveyard
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses }]
}
