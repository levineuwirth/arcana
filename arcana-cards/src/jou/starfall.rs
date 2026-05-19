//! Starfall — `{4}{R}` instant. "Starfall deals 3 damage to target
//! creature. If that creature is an enchantment, Starfall deals 3 damage
//! to that creature's controller."
//!
//! # GAP: conditional damage based on the target's type ("if it is an
//! enchantment") requires Effect::Conditional with a type-check condition
//! that is not in the catalog; damage to "that creature's controller"
//! requires looking up the permanent's controller at resolve time, which
//! is not expressible without state inspection. The guaranteed 3 damage
//! to the creature is emitted; the conditional rider is omitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Starfall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Starfall deals 3 damage to target creature. If that creature is an enchantment, Starfall deals 3 damage to that creature's controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: conditional "if enchantment creature, deal 3 to its controller" not expressible
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}
