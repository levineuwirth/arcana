//! Revival Experiment — `{4}{B}{G}` sorcery. "For each permanent type, return
//! up to one card of that type from your graveyard to the battlefield. You lose
//! 3 life for each card returned this way. Exile Revival Experiment."
//!
//! # GAP: "for each permanent type" iteration, per-returned-card life loss, and
//! self-exile are not expressible via the catalog. We emit a single
//! ReturnFromGraveyardToBattlefield as a placeholder and note the gaps.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revival Experiment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each permanent type, return up to one card of that type from your graveyard to the battlefield. You lose 3 life for each card returned this way. Exile Revival Experiment.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::new() },
                    count: TargetCount::UpTo(5),
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
    // GAP: per-permanent-type iteration; per-card life loss; self-exile
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    effects
}
