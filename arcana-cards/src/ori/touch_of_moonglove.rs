//! Touch of Moonglove — `{B}` instant.
//! "Target creature you control gets +1/+0 and gains deathtouch until end of
//! turn. Whenever a creature dealt damage by that creature dies this turn, its
//! controller loses 2 life."
//!
//! # GAP: triggered ability on the target creature until end of turn
//! The pump (+1/+0) and deathtouch keyword grant are expressible. The
//! "whenever a creature dealt damage by that creature dies this turn, its
//! controller loses 2 life" rider requires a temporary triggered ability to be
//! placed on a permanent, which is not available in the catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Touch of Moonglove");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 and gains deathtouch until end of turn. Whenever a creature dealt damage by that creature dies this turn, its controller loses 2 life.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
        // GAP: "whenever a creature dealt damage by this creature dies this turn, its controller loses 2 life"
        // — temporary triggered ability on target permanent not expressible
    ]
}
