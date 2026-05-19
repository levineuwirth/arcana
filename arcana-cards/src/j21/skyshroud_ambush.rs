//! Skyshroud Ambush — `{1}{G}` instant. "Target creature you control fights
//! target creature you don't control. When the creature you control wins the
//! fight, draw a card."
//!
//! GAP: conditional draw trigger on winning a fight (outcome-checking after
//! Effect::Fight) is not expressible with the catalog's Effect variants.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshroud Ambush");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control fights target creature you don't control. When the creature you control wins the fight, draw a card.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
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
    let mut iter = entry.targets.targets.iter();
    let Some(t0) = iter.next() else { return Vec::new(); };
    let Some(t1) = iter.next() else { return Vec::new(); };
    let (TargetChoice::Object(id0), TargetChoice::Object(id1)) = (t0, t1) else { return Vec::new(); };
    vec![
        Effect::Fight { a: *id0, b: *id1 },
        // GAP: conditional draw if id0 wins the fight — fight outcome check not expressible
    ]
}
