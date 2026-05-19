//! Tail Swipe — `{G}` instant. "Choose target creature you control and
//! target creature you don't control. If you cast this spell during your
//! main phase, the creature you control gets +1/+1 until end of turn.
//! Then those creatures fight each other."
//!
//! GAP: MainPhaseConditional (no Effect variant to apply effects
//! conditionally based on the phase in which a spell was cast).
//! The +1/+1 pump is omitted; the fight is expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tail Swipe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature you don't control. If you cast this spell during your main phase, the creature you control gets +1/+1 until end of turn. Then those creatures fight each other.".into(),
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
    // GAP: MainPhaseConditional (no variant to conditionally apply +1/+1 based on cast phase)
    let mut targets = entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(*id) } else { None }
    });
    let Some(a) = targets.next() else { return Vec::new(); };
    let Some(b) = targets.next() else { return Vec::new(); };
    vec![Effect::Fight { a, b }]
}
