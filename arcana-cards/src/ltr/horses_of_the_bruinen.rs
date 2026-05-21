//! Horses of the Bruinen — `{3}{U}{U}` sorcery. "Return up to two
//! target creatures to their owners' hands. Scry 1. The Ring tempts
//! you." Ring isn't catalog; emit bounces (up to 2 targets) + Scry.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horses of the Bruinen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to two target creatures to their owners' hands. Scry 1. The Ring tempts you.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
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
    // GAP: "The Ring tempts you" is not in catalog.
    let mut effects = Vec::new();
    for choice in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = choice {
            effects.push(Effect::ReturnToHand { target: *id });
        }
    }
    effects.push(Effect::Scry { player: entry.controller, count: 1 });
    effects
}
