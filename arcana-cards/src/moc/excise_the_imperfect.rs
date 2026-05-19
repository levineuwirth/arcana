//! Excise the Imperfect — `{1}{W}{W}` instant. "Exile target nonland permanent. Its controller
//! incubates X, where X is its mana value."
//! GAP: incubate X mechanic (create an Incubator token with X +1/+1 counters that can transform
//! into a 0/0 Phyrexian artifact creature) — no Effect::Incubate variant; X is also dynamically
//! computed from the exiled permanent's mana value.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Excise the Imperfect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target nonland permanent. Its controller incubates X, where X is its mana value.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new().without_types(TypeLine::LAND.into())),
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
    vec![
        Effect::ExilePermanent { target: *id },
        // GAP: incubate X (X = exiled permanent's mana value) — no Effect::Incubate variant;
        // dynamic X based on exiled permanent's mana value not expressible
    ]
}
