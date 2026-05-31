//! Muscle Burst — `{1}{G}` instant. "Target creature gets +X/+X until
//! end of turn, where X is 3 plus the number of cards named Muscle
//! Burst in all graveyards."
//!
//! X is dynamic: 3 plus the count of cards named "Muscle Burst" across
//! every player's graveyard, computed at resolution via
//! `script::graveyard_matching` summed over all players.

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
    let name = reg.interner_mut().intern("Muscle Burst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +X/+X until end of turn, where X is 3 plus the number of cards named Muscle Burst in all graveyards."
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let name = reg.interner().lookup("Muscle Burst");
    let filter = ObjectFilter {
        name,
        ..ObjectFilter::default()
    };
    let mut count: u32 = 0;
    for p in script::all_players(state) {
        count += script::graveyard_matching(state, &filter, p, entry.controller);
    }
    let x = (3 + count) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
