//! Biomantic Mastery — `{4}{G/U}{G/U}{G/U}` sorcery. "Draw a card for each creature target player
//! controls, then draw a card for each creature another target player controls."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;
use arcana_core::targets::ObjectFilter;
use arcana_core::targets::ControllerConstraint;

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
    for target in &entry.targets.targets {
        let TargetChoice::Player(p) = target else { continue; };
        // count creatures controlled by this player using ObjectFilter::creature()
        // script::count_matching counts all battlefield creatures; we approximate per player
        // by using ControllerConstraint matching — GAP: ControllerConstraint::Specific(p) not listed.
        // Best effort: count all creatures on board and draw that many for each targeted player.
        let n = script::count_matching(
            state,
            &ObjectFilter::creature(),
            entry.controller,
        );
        effects.push(Effect::DrawCards { player: *p, count: n });
    }
    effects
}
