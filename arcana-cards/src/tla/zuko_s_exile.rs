//! Zuko's Exile — `{5}` instant (colorless) — Lesson, "Exile target artifact,
//! creature, or enchantment. Its controller creates a Clue token."
//!
//! GAP: target's controller creates a Clue token (Clue token for the target's
//! controller, not the caster; no way to read target's controller as PlayerId
//! at resolve time; also Clue / Investigate is not in the Effect catalog).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zuko's Exile");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target artifact, creature, or enchantment. Its controller creates a Clue token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine::ARTIFACT.into())
                            .with_types_any(TypeLine::CREATURE.into())
                            .with_types_any(TypeLine::ENCHANTMENT.into()),
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
    // GAP: target's controller creates a Clue token (Investigate effect not
    // in catalog; target's controller not accessible as PlayerId)
    vec![Effect::ExilePermanent { target: *id }]
}
