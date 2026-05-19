//! Mutiny — `{R}` sorcery, "Target creature an opponent controls deals damage equal
//! to its power to another target creature that player controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mutiny");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature an opponent controls deals damage equal to its power to another target creature that player controls.".into(),
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
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(dealer_id) = first else { return Vec::new(); };
    let TargetChoice::Object(target_id) = second else { return Vec::new(); };
    let power = script::power_of(state, *dealer_id).max(0) as u32;
    vec![Effect::DealDamage {
        source: *dealer_id,
        target: DamageTarget::Object(*target_id),
        amount: power,
    }]
}
