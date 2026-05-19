//! Rewind — `{2}{U}{U}` instant. "Counter target spell. Untap up to four lands."
//!
//! # GAP: "untap up to four lands" requires targeting up to 4 permanents and
//! untapping each. We counter the spell and note the multi-untap as a gap
//! (TargetCount::UpTo(4) lands + untap loop not in catalog patterns).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rewind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Untap up to four lands.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())),
                        count: TargetCount::UpTo(4),
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
    let mut effects = Vec::new();
    let mut iter = entry.targets.targets.iter();
    // First target: spell to counter
    if let Some(TargetChoice::Object(spell_id)) = iter.next() {
        effects.push(Effect::Counter { target: *spell_id });
    }
    // Remaining targets: lands to untap
    for t in iter {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::Untap { target: *id });
        }
    }
    effects
}
