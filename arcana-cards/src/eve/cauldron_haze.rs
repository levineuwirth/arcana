//! Cauldron Haze — `{1}{W/B}` instant. "Choose any number of target
//! creatures. Each of those creatures gains persist until end of turn."
//!
//! # GAP: granting a keyword (Persist) that triggers a delayed-return
//! ability until end of turn via Effect::GrantKeyword is supported for
//! evergreen keywords in the catalog, but Persist is listed as a fully
//! implemented keyword (KeywordAbility::Persist) — however the prompt
//! specifies Persist as a keyword grant until end of turn, which
//! Effect::GrantKeyword with Duration::EndOfTurn supports.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cauldron Haze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose any number of target creatures. Each of those creatures gains persist until end of turn.".into(),
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
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Persist,
                duration: Duration::EndOfTurn,
            })
        } else {
            None
        }
    }).collect()
}
