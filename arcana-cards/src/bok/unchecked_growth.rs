//! Unchecked Growth — `{2}{G}` Instant — Arcane. "Target creature
//! gets +4/+4 until end of turn. If it's a Spirit, it gains trample
//! until end of turn."
//!
//! GAP: conditional 'if it's a Spirit' requires inspecting the
//! target's subtypes via state — no script helper exposes that. We
//! emit only the +4/+4.

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
    let name = reg.interner_mut().intern("Unchecked Growth");
    let _arcane = reg.interner_mut().intern("Arcane");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +4/+4 until end of turn. If it's a Spirit, it gains trample until end of turn.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: per-target subtype check 'is a Spirit' not in script API.
    vec![Effect::Pump {
        target: *id,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
