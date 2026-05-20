//! Spoils of Evil — `{2}{B}` instant. "For each artifact or creature card in
//! target opponent's graveyard, add {C} and you gain 1 life."
//!
//! GAP: no Effect variant to add mana (floating mana production). The life
//! gain is computable dynamically but the mana production is inexpressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement, TargetChoice};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;
use arcana_core::targets::ObjectFilter;

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
    let TargetChoice::Player(opp) = target else { return Vec::new(); };
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let creature_filter = ObjectFilter::creature();
    let n_artifacts = script::graveyard_matching(state, &artifact_filter, *opp, entry.controller);
    let n_creatures = script::graveyard_matching(state, &creature_filter, *opp, entry.controller);
    let n = n_artifacts + n_creatures;
    // GAP: no Effect variant to add floating mana (add {C} per card)
    vec![Effect::GainLife { player: entry.controller, amount: n }]
}
