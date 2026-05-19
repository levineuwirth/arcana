//! Diplomatic Relations — `{2}{G}` instant. "Target creature you control gets
//! +1/+0 and gains vigilance until end of turn. It deals damage equal to its
//! power to target creature an opponent controls."
//!
//! # GAP: DamageEqualToPower — no Effect variant for 'deals damage equal to
//!   its own power' (Fight is symmetric; this is one-sided power-based damage)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diplomatic Relations");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 and gains vigilance until end of turn. It deals damage equal to its power to target creature an opponent controls.".into(),
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
    // GAP: DamageEqualToPower — no Effect variant for one-sided 'deals damage equal to its power'
    // Partial: pump the first target and fight as best approximation
    let mut targets = entry.targets.targets.iter();
    let Some(t1) = targets.next() else { return Vec::new(); };
    let Some(t2) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    let TargetChoice::Object(id2) = t2 else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id1,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Vigilance],
        },
        Effect::Fight { a: *id1, b: *id2 },
    ]
}
