//! Sacred Boon — `{1}{W}` instant. "Prevent the next 3 damage that
//! would be dealt to target creature this turn. At the beginning of
//! the next end step, put a +0/+1 counter on that creature for each 1
//! damage prevented this way."
//!
//! The prevention shield is expressed with `Effect::PreventDamage`.
//! The end-step rider that adds +0/+1 counters equal to the amount of
//! damage actually prevented cannot be modeled: there is no primitive
//! to read "damage prevented this way" and feed it into a delayed
//! counter placement.

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
    let name = reg.interner_mut().intern("Sacred Boon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Prevent the next 3 damage that would be dealt to target creature this turn. At the beginning of the next end step, put a +0/+1 counter on that creature for each 1 damage prevented this way.".into(),
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
    // GAP: cannot place +0/+1 counters at the next end step equal to
    // the amount of damage actually prevented this way — there is no
    // primitive to read the prevented amount and feed it to a delayed
    // counter placement. Only the prevention shield is emitted.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: Some(3),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
