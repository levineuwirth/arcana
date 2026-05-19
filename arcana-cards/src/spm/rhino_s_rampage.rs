//! Rhino's Rampage — `{R/G}` sorcery. "Target creature you control gets +1/+0 until end of
//! turn. It fights target creature an opponent controls. When excess damage is dealt to the
//! creature an opponent controls this way, destroy up to one target noncreature artifact with
//! mana value 3 or less."
//!
//! GAP: excess-damage triggered destroy and mana-value filter on artifact target are not
//! in the catalog. Pump + Fight are expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhino's Rampage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn. It fights target creature an opponent controls. When excess damage is dealt to the creature an opponent controls this way, destroy up to one target noncreature artifact with mana value 3 or less.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
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
    // GAP: excess-damage trigger and mana-value artifact destroy not in catalog
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(a)) = targets.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(b)) = targets.next() else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *a,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Fight { a: *a, b: *b },
    ]
}
