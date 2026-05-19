//! Rabid Attack — `{1}{B}` instant. "Until end of turn, any number of target
//! creatures you control each get +1/+0 and gain 'When this creature dies,
//! draw a card.'"
//!
//! # GAP: granting a triggered ability ("when this creature dies, draw a
//! card") until end of turn has no Effect variant (GrantKeyword only grants
//! KeywordAbility values). Best-effort: pump +1/+0.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rabid Attack");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, any number of target creatures you control each get \
                       +1/+0 and gain \"When this creature dies, draw a card.\""
                    .into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Any,
                    controller: None,
                }],
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
    // GAP: grant "when dies, draw a card" triggered ability until end of turn not supported
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::Pump {
                    target: *id,
                    power: 1,
                    toughness: 0,
                    duration: Duration::EndOfTurn,
                    keywords: vec![],
                })
            } else {
                None
            }
        })
        .collect()
}
