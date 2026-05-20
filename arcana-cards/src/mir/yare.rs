//! Yare — `{2}{W}` instant. "Target creature defending player
//! controls gets +3/+0 until end of turn. That creature can block up
//! to two additional creatures this turn."
//!
//! No "defending player controls" target refinement; targets any
//! creature for the pump. The extra-blockers rider has no catalog
//! Effect (GAP'd).

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
    let name = reg.interner_mut().intern("Yare");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature defending player controls gets +3/+0 until end of turn. That creature can block up to two additional creatures this turn.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "can block up to two additional creatures" has no catalog Effect.
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
