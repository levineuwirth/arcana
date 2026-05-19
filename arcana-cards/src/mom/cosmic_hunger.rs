//! Cosmic Hunger — `{1}{G}` instant.
//! "Target creature you control deals damage equal to its power to another
//! target creature, planeswalker, or battle."
//!
//! GAP: TargetFilter has no "creature, planeswalker, or battle" composite.
//! Battle type not modeled. Best-effort: target creature fights target creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cosmic Hunger");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to another target creature, planeswalker, or battle.".into(),
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
    let (TargetChoice::Object(a), TargetChoice::Object(b)) = (t0, t1) else { return Vec::new(); };
    let power = script::power_of(state, *a).max(0) as u32;
    use arcana_core::events::DamageTarget;
    // GAP: "planeswalker or battle" target type not available; using creature fight as best-effort
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*b),
        amount: power,
    }]
}
