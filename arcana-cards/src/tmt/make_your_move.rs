//! Make Your Move — `{2}{W}` instant. "Destroy target artifact,
//! enchantment, or creature with power 4 or greater." The disjunction
//! (artifact OR enchantment OR creature-with-min-power-4) isn't a
//! straight ObjectFilter; with_types_any covers the union, but the
//! per-type filter conjunction (power gate ONLY on creature) isn't
//! catalog-clean.

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
    let name = reg.interner_mut().intern("Make Your Move");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                // GAP: the disjunctive filter (artifact OR enchantment OR creature-with-power>=4) cannot be expressed as one ObjectFilter; using the union of all three types and dropping the power gate.
                text: "Destroy target artifact, enchantment, or creature with power 4 or greater.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            TypeLine(
                                TypeLine::ARTIFACT
                                    | TypeLine::ENCHANTMENT
                                    | TypeLine::CREATURE,
                            ),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
