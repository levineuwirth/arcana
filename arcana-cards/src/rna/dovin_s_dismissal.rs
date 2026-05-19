//! Dovin's Dismissal — `{2}{W}{U}` instant.
//! "Put up to one target tapped creature on top of its owner's library. You
//! may search your library and/or graveyard for a card named Dovin, Architect
//! of Law, reveal it, and put it into your hand. If you search your library
//! this way, shuffle."
//! GAP: 'tapped creature' target restriction; 'up to one' optional target;
//! search graveyard for specific named card; combined library+graveyard search.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dovin's Dismissal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put up to one target tapped creature on top of its owner's library. You may search your library and/or graveyard for a card named Dovin, Architect of Law, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
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
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            // GAP: 'tapped creature' restriction on target
            effects.push(Effect::PutOnTopOfLibrary { target: *id });
        }
    }
    // GAP: search library and/or graveyard for specific named card 'Dovin, Architect of Law'
    effects
}
