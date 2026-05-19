//! Containment Breach — `{2}{G}` sorcery (Lesson).
//! "Destroy target artifact or enchantment. If its mana value is 2 or
//! less, create a 1/1 black and green Pest creature token with 'When
//! this token dies, you gain 1 life.'"
//
// GAP: conditional token creation based on target's mana value (runtime
//      comparison) not expressible.
// GAP: Pest token has a triggered ability ("when this dies, gain 1 life")
//      — TokenDefinition.abilities is `vec![]` with no mechanism to attach
//      triggers to tokens.
// Best effort: destroy the permanent; token and conditional omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Containment Breach");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target artifact or enchantment. If its mana value is 2 or less, create a 1/1 black and green Pest creature token with \"When this token dies, you gain 1 life.\"".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
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
    // GAP: conditional token if MV ≤2; GAP: Pest death trigger
    vec![Effect::DestroyPermanent { target: *id }]
}
