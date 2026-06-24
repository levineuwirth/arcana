//! Wrench Mind — `{B}{B}` sorcery. Target player discards two cards
//! unless they discard an artifact card.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrench Mind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player discards two cards unless they discard an artifact card.".into(),
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
    // GAP: "unless they discard an ARTIFACT card" — the payment is a TYPED
    // discard (discard a card of a specific type), but OptionalPaymentKind::
    // Discard(u32) only takes a count, with no card-type filter. The
    // discard-an-artifact payment mode isn't expressible; emit the base
    // discard-two penalty (over-applies when they hold an artifact to pitch).
    vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::ControllerChooses,
    }]
}
