//! Desperate Measures — `{B}` instant. "Target creature gets +1/-1
//! until end of turn. When it dies under your control this turn,
//! draw two cards."
//!
//! GAP: "when target dies under your control" rider (a one-shot
//! delayed dies-trigger with a draw side effect) isn't expressible
//! via DelayedAction (action set: Sacrifice / Exile / ReturnToHand /
//! ReturnFromExileToBattlefield — no Draw); emit the pump only.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Desperate Measures");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +1/-1 until end of turn. When it dies under your control this turn, draw two cards.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(t) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = t else { return Vec::new(); };
    // GAP: dies-trigger draw side effect not modeled.
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
