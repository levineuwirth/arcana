//! Teferi's Contingency — `{W}{U}{U}` instant. "Counter target spell.
//! Each card in its controller's graveyard, hand, and library with
//! the same name as that spell perpetually gains 'This spell costs
//! {2} more to cast.'"
//!
//! "Perpetually gains" cost modification on same-named cards across
//! zones has no catalog Effect. Only the counter is modeled.

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: perpetual cost-up rider on same-named cards across zones not in catalog.
    vec![Effect::Counter { target: *id }]
}
