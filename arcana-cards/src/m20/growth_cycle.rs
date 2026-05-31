//! Growth Cycle — `{1}{G}` instant. "Target creature gets +3/+3 until
//! end of turn. It gets an additional +2/+2 until end of turn for each
//! card named Growth Cycle in your graveyard."
//!
//! The base buff is +3/+3, plus +2/+2 per copy of this card in your
//! graveyard. The per-copy amount is dynamic and is computed at
//! resolution with `script::graveyard_matching` over a name filter.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Growth Cycle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +3/+3 until end of turn. It gets an additional +2/+2 until end of turn for each card named Growth Cycle in your graveyard.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let nm = reg.interner().lookup("Growth Cycle");
    let copies = script::graveyard_matching(
        state,
        &ObjectFilter { name: nm, ..ObjectFilter::default() },
        entry.controller,
        entry.controller,
    );
    let bonus = 3 + 2 * copies as i32;
    vec![Effect::Pump {
        target: *id,
        power: bonus,
        toughness: bonus,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
