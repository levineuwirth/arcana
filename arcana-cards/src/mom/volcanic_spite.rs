//! Volcanic Spite — `{1}{R}` instant. "Volcanic Spite deals 3 damage to
//! target creature, planeswalker, or battle. You may put a card from your
//! hand on the bottom of your library. If you do, draw a card."
//!
//! GAP: no TargetFilter for planeswalker or battle type, and no Effect variant
//! for optional hand-to-bottom-of-library looting. Using creature target for
//! the damage; optional draw rider omitted.

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
    let name = reg.interner_mut().intern("Volcanic Spite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Volcanic Spite deals 3 damage to target creature, planeswalker, or battle. You may put a card from your hand on the bottom of your library. If you do, draw a card.".into(),
                // GAP: no TargetFilter for planeswalker or battle; using creature only
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
    // GAP: no Effect variant for optional hand-to-bottom loot rider
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}
