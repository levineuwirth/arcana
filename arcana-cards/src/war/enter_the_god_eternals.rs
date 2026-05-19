//! Enter the God-Eternals — `{2}{U}{U}{B}` sorcery. "Enter the God-Eternals
//! deals 4 damage to target creature. You gain life equal to the damage dealt
//! this way. Mill 4. Amass Zombies 4."
//!
//! GAP: Amass Zombies 4 mechanic not in the Effect catalog.
//! DealDamage 4, GainLife 4, and Mill 4 are expressed.

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
    let name = reg.interner_mut().intern("Enter the God-Eternals");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Enter the God-Eternals deals 4 damage to target creature. You gain life equal to the damage dealt this way. Mill 4. Amass Zombies 4.".into(),
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
    // GAP: Amass Zombies 4 not in the Effect catalog
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 4,
        },
        Effect::GainLife { player: entry.controller, amount: 4 },
        Effect::Mill { player: entry.controller, count: 4 },
    ]
}
