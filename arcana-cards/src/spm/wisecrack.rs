//! Wisecrack — `{2}{R}` instant. "Target creature deals damage equal to its power
//! to itself. If that creature is attacking, Wisecrack deals 2 damage to that
//! creature's controller."
//!
//! GAP: detecting whether a creature is currently attacking (combat status check)
//! is not available via the script helpers — full effect cannot be expressed.
//! We emit the self-damage portion via Pump with power_of, and note the gap
//! for the conditional rider.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wisecrack");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature deals damage equal to its power to itself. If that creature is attacking, Wisecrack deals 2 damage to that creature's controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let pwr = script::power_of(state, *id);
    let amount = pwr.max(0) as u32;
    // GAP: conditional "if that creature is attacking" — combat status check not available via script helpers
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
