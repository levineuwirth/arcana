//! Disallow — `{1}{U}{U}` instant. "Counter target spell, activated
//! ability, or triggered ability." The spell-or-ability target union
//! is modeled as a choose-one modal (CR 700.2): clause 0 targets a
//! spell (TargetFilter::Spell), clause 1 targets an activated or
//! triggered ability (TargetFilter::AbilityOnStack). Either way the
//! resolver emits Effect::Counter, which handles both spell objects
//! and ability stack entries.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModalSpec, ModeClause, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disallow");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    // "Counter target spell, activated ability, or triggered ability"
    // — one target drawn from the union of spells and abilities on the
    // stack. No single TargetFilter expresses the union, so the pick
    // is modeled as a choose-one modal; each clause carries its own
    // target requirement and both resolve to the same Counter.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell, activated ability, or triggered ability.".into(),
                target_requirements: vec![],
                modal: Some(ModalSpec {
                    min_modes: 1,
                    max_modes: 1,
                    clauses: vec![
                        ModeClause {
                            text: "Counter target spell.".into(),
                            target_requirements: vec![TargetRequirement {
                                filter: TargetFilter::Spell(ObjectFilter::default()),
                                count: TargetCount::Exactly(1),
                                controller: None,
                            }],
                        },
                        ModeClause {
                            text: "Counter target activated or triggered ability.".into(),
                            target_requirements: vec![TargetRequirement {
                                filter: TargetFilter::AbilityOnStack {
                                    activated: true,
                                    triggered: true,
                                    source_filter: None,
                                },
                                count: TargetCount::Exactly(1),
                                controller: None,
                            }],
                        },
                    ],
                }),
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Counter { target: *id }]
}
