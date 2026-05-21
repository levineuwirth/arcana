//! Unwind — `{2}{U}` instant. "Counter target noncreature spell.
//! Untap up to three lands." Untap-up-to-three on a target-filter we
//! don't get to pick (no second target slot for the lands here in this
//! card class without a target_requirements entry) — expose three
//! optional land targets so the Untap fires per chosen land.

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
    let name = reg.interner_mut().intern("Unwind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target noncreature spell. Untap up to three lands.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(
                            ObjectFilter::new()
                                .without_types(TypeLine::CREATURE.into()),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(3),
                        controller: None,
                    },
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
    let mut iter = entry.targets.targets.iter();
    let Some(first) = iter.next() else { return Vec::new(); };
    let TargetChoice::Object(spell_id) = first else { return Vec::new(); };
    let mut effects = vec![Effect::Counter { target: *spell_id }];
    for choice in iter {
        if let TargetChoice::Object(land_id) = choice {
            effects.push(Effect::Untap { target: *land_id });
        }
    }
    effects
}
