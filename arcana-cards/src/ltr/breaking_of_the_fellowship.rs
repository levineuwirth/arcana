//! Breaking of the Fellowship — `{1}{R}` sorcery. "Target creature an
//! opponent controls deals damage equal to its power to another
//! target creature that player controls. The Ring tempts you."
//!
//! "The Ring tempts you" has no catalog Effect — GAP'd; the
//! power-equal damage between the two targeted creatures is emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breaking of the Fellowship");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature an opponent controls deals damage equal to its power to another target creature that player controls. The Ring tempts you.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(src)) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(victim)) = entry.targets.targets.get(1) else { return Vec::new(); };
    let amount = script::power_of(state, *src).max(0) as u32;
    // GAP: "The Ring tempts you" has no catalog Effect.
    vec![Effect::DealDamage {
        source: *src,
        target: DamageTarget::Object(*victim),
        amount,
    }]
}
