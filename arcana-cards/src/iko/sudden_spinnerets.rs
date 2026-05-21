//! Sudden Spinnerets — `{G}` instant. "Target creature gets +1/+3
//! until end of turn. Put a reach counter on it. Untap it."
//!
//! The +1/+3 and Untap are expressed. A reach counter is not a
//! supported counter kind (only +1/+1) — GAP.

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
    let name = reg.interner_mut().intern("Sudden Spinnerets");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +1/+3 until end of turn. Put a reach counter on it. Untap it.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: a reach counter is not a supported counter kind.
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Untap { target: *id },
    ]
}
