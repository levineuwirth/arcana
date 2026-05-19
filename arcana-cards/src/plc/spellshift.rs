//! Spellshift — `{3}{U}` instant, "Counter target instant or sorcery spell.
//! Its controller may reveal cards from the top of their library until they
//! reveal an instant or sorcery card. They may cast that card without paying
//! its mana cost. Then they put the rest on the bottom of their library in any
//! order."
//!
//! GAP: counter target instant-or-sorcery (instant/sorcery type filter on
//! TargetFilter::Spell not supported); reveal-until-then-cast-for-free mechanic
//! not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellshift");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target instant or sorcery spell. Its controller may reveal cards from the top of their library until they reveal an instant or sorcery card. They may cast that card without paying its mana cost. Then they put the rest on the bottom of their library in any order.".into(),
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
    // GAP: instant-or-sorcery type filter on spell target; reveal-until-cast-free mechanic
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Counter { target: *id }]
}
