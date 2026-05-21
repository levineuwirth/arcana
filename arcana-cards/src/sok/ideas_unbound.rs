//! Ideas Unbound — `{U}{U}` sorcery (Arcane). "Draw three cards.
//! Discard three cards at the beginning of the next end step."
//! Sorcery — Arcane subtype isn't expressible at the Characteristics
//! layer for sorceries; flag it but emit the rest.

use arcana_core::effects::{
    DelayedAction, DelayedWhen, DiscardChoice, Effect,
};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ideas Unbound");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: Sorcery — Arcane subtype isn't expressible at this layer.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. Discard three cards at the beginning of the next end step.".into(),
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
    // GAP: DelayedAction has no 'Discard N' variant — only Sacrifice/
    // Exile/ReturnToHand/ReturnFromExileToBattlefield. Emit the draw;
    // the parametrized end-step discard is unmodeled.
    let _ = DelayedWhen::NextEndStep;
    let _ = DelayedAction::Sacrifice;
    let _ = DiscardChoice::ControllerChooses;
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
