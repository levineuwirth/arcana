//! Teferi's Contingency — `{W}{U}{U}` instant, "Counter target spell. Each
//! card in its controller's graveyard, hand, and library with the same name as
//! that spell perpetually gains 'This spell costs {2} more to cast.'"
//!
//! # GAP
//! No "perpetually gains cost increase" effect in the catalog. Partial:
//! Counter is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi's Contingency");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Each card in its controller's graveyard, hand, and library with the same name as that spell perpetually gains \"This spell costs {2} more to cast.\"".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
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
    // GAP: no perpetual cost-increase effect; no same-name enumeration across zones
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Counter { target: stack_id }]
}
