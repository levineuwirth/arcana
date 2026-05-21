//! Torment of Venom — `{2}{B}{B}` instant. "Put three -1/-1 counters
//! on target creature. Its controller loses 3 life unless they
//! sacrifice another nonland permanent of their choice or discard a
//! card." The unless-choice rider is not expressible; emit the
//! counters and a flat 3 life loss as a best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torment of Venom");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put three -1/-1 counters on target creature. Its controller loses 3 life unless they sacrifice another nonland permanent of their choice or discard a card.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let ctrl = script::target_controller(state, *id, entry.controller);
    // GAP: -1/-1 counters not in CounterKind catalog (only PlusOnePlusOne); using PlusOnePlusOne is wrong-signed, so omit the counter effect.
    vec![
        Effect::LoseLife { player: ctrl, amount: 3 },
    ]
}
