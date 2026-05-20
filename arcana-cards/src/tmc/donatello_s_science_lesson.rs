//! Donatello's Science Lesson — `{2}{U}` instant. "Tap up to two
//! target creatures. Up to two target players each draw a card."

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
    let name = reg.interner_mut().intern("Donatello's Science Lesson");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Tap up to two target creatures. Up to two target players each draw a card.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        match t {
            TargetChoice::Object(id) => effects.push(Effect::Tap { target: *id }),
            TargetChoice::Player(p) => effects.push(Effect::DrawCards {
                player: *p,
                count: 1,
            }),
            _ => {}
        }
    }
    effects
}
