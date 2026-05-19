//! Legion's End — `{1}{B}` sorcery.
//! "Exile target creature an opponent controls with mana value 2 or less
//! and all other creatures that player controls with the same name as that
//! creature. Then that player reveals their hand and exiles all cards with
//! that name from their hand and graveyard."
//!
//! GAP: "same name as target creature" filter not expressible. GAP: exile
//! cards by name from hand. The single-target exile with CMC<=2 is expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Legion's End");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature an opponent controls with mana value 2 or less and all other creatures that player controls with the same name as that creature. Then that player reveals their hand and exiles all cards with that name from their hand and graveyard.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_max_cmc(2),
                    ),
                    count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "same name" filter for additional exiles and hand/graveyard exile by name
    vec![Effect::ExilePermanent { target: *id }]
}
