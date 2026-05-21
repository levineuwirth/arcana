//! Biomantic Mastery — `{4}{G/U}{G/U}{G/U}` sorcery. "Draw a card for each
//! creature target player controls, then draw a card for each creature
//! another target player controls."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biomantic Mastery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G/U}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card for each creature target player controls, then draw a card for each creature another target player controls.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Player,
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Player(p) = t {
            let n = script::count_matching(
                state,
                &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                *p,
            );
            effects.push(Effect::DrawCards { player: entry.controller, count: n });
        }
    }
    effects
}
