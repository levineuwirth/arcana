//! Test of Faith — `{1}{W}` instant. "Prevent the next 3 damage that
//! would be dealt to target creature this turn. For each 1 damage
//! prevented this way, put a +1/+1 counter on that creature."
//!
//! The prevention shield is faithful (`Effect::PreventDamage`). The
//! "for each 1 damage prevented this way, put a +1/+1 counter" rider
//! must observe how much damage the shield actually prevents and react
//! later in the turn — there is no engine primitive that hooks the
//! amount of damage prevented by a replacement effect and converts it
//! into counters, so that clause is GAPped.

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
    let name = reg.interner_mut().intern("Test of Faith");
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
                text: "Prevent the next 3 damage that would be dealt to target creature this turn. For each 1 damage prevented this way, put a +1/+1 counter on that creature.".into(),
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
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: Some(3),
        duration: ReplacementDuration::EndOfTurn,
    }]
    // GAP: "For each 1 damage prevented this way, put a +1/+1 counter on
    // that creature." Requires observing the amount of damage the
    // prevention shield actually absorbs and later converting it into
    // counters; no engine primitive exposes prevented-damage amount.
}
