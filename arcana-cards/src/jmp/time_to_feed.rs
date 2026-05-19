//! Time to Feed — `{2}{G}` sorcery. "Choose target creature an opponent controls.
//! When that creature dies this turn, you gain 3 life. Target creature you
//! control fights that creature."
//!
//! GAP: "when target creature dies this turn, gain 3 life" delayed triggered
//! ability not in catalog. Partial: Fight between two targeted creatures expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time to Feed");
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
                text: "Choose target creature an opponent controls. When that creature dies this turn, you gain 3 life. Target creature you control fights that creature.".into(),
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
    // GAP: delayed "when creature dies this turn, gain 3 life" trigger not in catalog
    let mut targets = entry.targets.targets.iter();
    let first = targets.next();
    let second = targets.next();
    match (first, second) {
        (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) => {
            vec![Effect::Fight { a: *a, b: *b }]
        }
        _ => Vec::new(),
    }
}
