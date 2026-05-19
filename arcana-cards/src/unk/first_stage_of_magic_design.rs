//! First Stage of Magic Design — `{W}{U}{B}{R}{G}` instant. "Gain 3 life, draw 3 cards, add
//! {B}{B}{B}, deal 3 damage to any target, and target creature gets +3/+3 until end of turn."
//!
//! # GAP: Add mana effect not in engine catalog.
//! # GAP: Multiple disparate targets (player for life/draw/damage, creature for pump) on one spell.
//! Partial implementation: gain 3 life + draw 3 cards only; damage, mana, and pump are omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("First Stage of Magic Design");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Gain 3 life, draw 3 cards, add {B}{B}{B}, deal 3 damage to any target, and target creature gets +3/+3 until end of turn.".into(),
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
    // GAP: Add mana effect not in engine catalog
    // GAP: DealDamage to any target and Pump on creature require separate targets
    vec![
        Effect::GainLife { player: entry.controller, amount: 3 },
        Effect::DrawCards { player: entry.controller, count: 3 },
    ]
}
