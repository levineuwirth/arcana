//! Boon of Boseiju — `{1}{G}` instant. "Target creature gets +X/+X
//! until end of turn, where X is the greatest mana value among
//! permanents you control. Untap it."
//!
//! The untap half is expressed; X is the greatest mana value among
//! permanents you control, but the script helper surface exposes no
//! "greatest mana value among permanents" accessor, so that DYNAMIC
//! pump amount cannot be computed and the +X/+X is GAP-ed rather than
//! hardcoded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boon of Boseiju");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +X/+X until end of turn, where X is the greatest mana value among permanents you control. Untap it.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "+X/+X where X is the greatest mana value among permanents you
    // control" — no script helper exposes the greatest mana value among
    // permanents, so the dynamic pump amount cannot be computed. Only the
    // untap is emitted.
    vec![Effect::Untap { target: *id }]
}
