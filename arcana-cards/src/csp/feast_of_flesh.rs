//! Feast of Flesh — `{B}` sorcery, "Feast of Flesh deals X damage to target creature and you
//! gain X life, where X is 1 plus the number of cards named Feast of Flesh in all graveyards."
//!
//! GAP: No engine effect for damage/life gain with amount computed from count of same-named cards in all graveyards.

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
    let name = reg.interner_mut().intern("Feast of Flesh");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Feast of Flesh deals X damage to target creature and you gain X life, where X is 1 plus the number of cards named Feast of Flesh in all graveyards.".into(),
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
    // GAP: No engine effect for damage/life amount based on count of same-named cards in all graveyards;
    // using base 1 as minimum
    vec![
        Effect::DealDamage { source: entry.source, target: DamageTarget::Object(*id), amount: 1 },
        Effect::GainLife { player: entry.controller, amount: 1 },
    ]
}
