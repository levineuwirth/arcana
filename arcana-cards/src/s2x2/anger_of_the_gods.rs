//! Anger of the Gods — `{1}{R}{R}` sorcery, "Anger of the Gods deals 3 damage to each creature.
//! If a creature dealt damage this way would die this turn, exile it instead."
//!
//! GAP: Replacement effect — exile instead of die for creatures dealt damage this way
//! (no exile-replacement Effect variant).
//! GAP: No documented API to enumerate all battlefield creatures from resolver.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anger of the Gods");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Anger of the Gods deals 3 damage to each creature. If a creature dealt damage this way would die this turn, exile it instead.".into(),
                target_requirements: vec![],
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
    // GAP: enumerate all battlefield creatures (no documented state accessor in resolver)
    // GAP: exile-instead-of-die replacement for damaged creatures
    vec![Effect::ForEach {
        targets: vec![],
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 3,
        }),
    }]
}
