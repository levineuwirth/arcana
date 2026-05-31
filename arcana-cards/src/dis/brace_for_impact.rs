//! Brace for Impact — `{4}{W}` instant. "Prevent all damage that
//! would be dealt to target multicolored creature this turn. For each
//! 1 damage prevented this way, put a +1/+1 counter on that creature."
//!
//! We express the damage-prevention half with `Effect::PreventDamage`
//! (amount: None = prevent all). The counter rider depends on the
//! amount of damage actually prevented, which is not available to the
//! resolver — that coupling is GAP-ed below.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brace for Impact");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                // GAP: target restriction "multicolored creature" is not an
                // expressible ObjectFilter refinement; using a plain creature
                // target requirement.
                text: "Prevent all damage that would be dealt to target multicolored creature this turn. For each 1 damage prevented this way, put a +1/+1 counter on that creature.".into(),
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
    // GAP: "for each 1 damage prevented, put a +1/+1 counter" — the amount of
    // damage actually prevented is not observable by the resolver, so the
    // counter rider cannot be wired.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
