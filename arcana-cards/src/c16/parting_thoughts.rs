//! Parting Thoughts — `{2}{B}` sorcery. "Destroy target creature. You
//! draw X cards and you lose X life, where X is the number of counters
//! on that creature."
//!
//! The destroy half is expressed; X is the number of counters on the
//! targeted creature, but the script helper surface exposes no
//! "number of counters on a permanent" accessor, so that DYNAMIC amount
//! cannot be computed and the draw/lose-life rider is GAP-ed rather than
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
    let name = reg.interner_mut().intern("Parting Thoughts");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature. You draw X cards and you lose X life, where X is the number of counters on that creature.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "draw X cards and lose X life, where X is the number of counters
    // on that creature" — no script helper exposes the count of counters on
    // a permanent, so the dynamic X cannot be computed. Only the destroy is
    // emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
