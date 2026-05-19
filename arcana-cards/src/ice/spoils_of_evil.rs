//! Spoils of Evil — `{2}{B}` instant. "For each artifact or creature card in
//! target opponent's graveyard, add {C} and you gain 1 life."
//!
//! GAP: adding mana ({C} per card) — no Effect variant for mana production (AddMana).
//! The life gain is computable via graveyard_matching.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spoils of Evil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each artifact or creature card in target opponent's graveyard, add {C} and you gain 1 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(opponent) = target else { return Vec::new(); };
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let creature_filter = ObjectFilter::creature();
    let artifacts = script::graveyard_matching(state, &artifact_filter, *opponent, entry.controller);
    let creatures = script::graveyard_matching(state, &creature_filter, *opponent, entry.controller);
    let count = artifacts + creatures;
    // GAP: add {C} for each card — no AddMana Effect variant
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: entry.controller, amount: count }]
}
