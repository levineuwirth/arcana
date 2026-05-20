//! Deny the Divine — `{2}{U}` instant. "Counter target creature or
//! enchantment spell. If that spell is countered this way, exile it
//! instead of putting it into its owner's graveyard."
//!
//! The "exile instead of graveyard" replacement on the countered spell
//! is not expressible; the base counter is emitted. "Creature or
//! enchantment spell" is a disjunctive filter the demonstrated
//! ObjectFilter can't express, so the spell filter is unconstrained.

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
    let name = reg.interner_mut().intern("Deny the Divine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target creature or enchantment spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.".into(),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "exile instead of graveyard" replacement on the countered spell not expressible.
    vec![Effect::Counter { target: *id }]
}
