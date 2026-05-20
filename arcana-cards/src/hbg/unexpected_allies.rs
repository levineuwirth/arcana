//! Unexpected Allies — `{1}{R}` sorcery. "Target nontoken creature
//! you control gets +2/+0 and gains double team until end of turn.
//! It also gains first strike until end of turn if it has the same
//! name as another creature you control or a creature card in your
//! graveyard."
//!
//! Double team is not in the demonstrated `KeywordAbility` surface
//! and the "same name as another creature you control / in graveyard"
//! conditional first strike has no catalog/script support. Only the
//! +2/+0 pump is modeled.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unexpected Allies");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target nontoken creature you control gets +2/+0 and gains double team until end of turn. It also gains first strike until end of turn if it has the same name as another creature you control or a creature card in your graveyard.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "double team" keyword not in supported surface; "same name as another creature
    // you control / in graveyard" name-equality conditional first-strike not in catalog.
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
