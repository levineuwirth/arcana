//! Time to Feed — `{2}{G}` sorcery. "Choose target creature an opponent
//! controls. When that creature dies this turn, you gain 3 life. Target
//! creature you control fights that creature."
//!
//! The fight is expressible (`Effect::Fight` over two targets). The
//! "when that creature dies this turn, you gain 3 life" rider is a
//! one-shot delayed death-trigger watching a specific object, which is
//! not expressible with the available `DelayedAction` (no GainLife
//! delayed action) — that portion is GAP-ed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time to Feed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature an opponent controls. When that creature dies this turn, you gain 3 life. Target creature you control fights that creature.".into(),
            target_requirements: vec![
                // The opponent's creature (target listed first).
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                // Your creature that fights it.
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    let Some(TargetChoice::Object(opp_creature)) = targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(my_creature)) = targets.get(1) else { return Vec::new(); };
    // GAP: "When that creature dies this turn, you gain 3 life" — a
    // one-shot delayed death-watch granting GainLife is not expressible
    // (DelayedAction has no GainLife action / dies-on-other-object hook).
    vec![Effect::Fight { a: *my_creature, b: *opp_creature }]
}
