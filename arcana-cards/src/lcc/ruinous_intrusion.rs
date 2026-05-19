//! Ruinous Intrusion — `{3}{G}` Instant. "Exile target artifact or
//! enchantment. Put X +1/+1 counters on target creature you control,
//! where X is the exiled permanent's mana value."
//!
//! # Implementation note
//! ExilePermanent is expressible. The counter count equal to the exiled
//! permanent's mana value is a dynamic quantity not expressible with
//! AddCounters (which requires a fixed count).
//!
//! # GAP
//! AddCounters with count = exiled permanent's mana value not
//! expressible (dynamic count).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruinous Intrusion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target artifact or enchantment. Put X +1/+1 counters on target creature you control, where X is the exiled permanent's mana value.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types_any(
                                TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT),
                            ),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
                ],
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
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(exile_id) = first else { return Vec::new(); };
    vec![
        Effect::ExilePermanent { target: *exile_id },
        // GAP: AddCounters count = exiled permanent's mana value (dynamic) not expressible
    ]
}
