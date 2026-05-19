//! Savage Swipe — `{G}` sorcery, "Target creature you control gets +2/+2
//! until end of turn if its power is 2. Then it fights target creature you
//! don't control."
//!
//! The conditional +2/+2 pump is checked at resolution via `script::power_of`.
//! The fight always occurs (pump just may not apply).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Savage Swipe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +2/+2 until end of turn if its power is 2. Then it fights target creature you don't control.".into(),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(id_a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(id_b) = t1 else { return Vec::new(); };
    let mut effects = Vec::new();
    if script::power_of(state, *id_a) == 2 {
        effects.push(Effect::Pump {
            target: *id_a,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects.push(Effect::Fight { a: *id_a, b: *id_b });
    effects
}
