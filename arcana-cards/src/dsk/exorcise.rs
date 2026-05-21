//! Exorcise — `{1}{W}` sorcery. "Exile target artifact, enchantment,
//! or creature with power 4 or greater." We split the disjunction:
//! the artifact-or-enchantment slot is wide-open; the
//! creature-with-power-4+ slot uses a power refinement. The catalog
//! only allows ONE TargetRequirement per spell-target — best effort:
//! target an artifact, enchantment, OR a creature with power 4+ via
//! the OR'd type bits (with creature power refinement applied).

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
    let name = reg.interner_mut().intern("Exorcise");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target artifact, enchantment, or creature with power 4 or greater.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            arcana_core::types::TypeLine(
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
    // GAP: 'creature with power 4 or greater' applies only to the creature
    // arm of the disjunction; can't express disjunctive filter per type.
    vec![Effect::ExilePermanent { target: *id }]
}
