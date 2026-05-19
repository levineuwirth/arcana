//! Giant's Ire — `{3}{R}` Kindred Sorcery — Giant. "Giant's Ire deals 4 damage to target player or
//! planeswalker. If you control a Giant, draw a card."
//! GAP: Kindred Sorcery subtype — TypeLine::SORCERY used; no Kindred supertype/subtype support.
//! GAP: "If you control a Giant" conditional draw — Effect::Conditional requires a condition type
//! for checking creature subtype presence; not available.

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
    let name = reg.interner_mut().intern("Giant's Ire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Giant's Ire deals 4 damage to target player or planeswalker. If you control a Giant, draw a card.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    // GAP: "If you control a Giant, draw a card" — no condition variant for checking creature subtype
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 4,
    }]
}
