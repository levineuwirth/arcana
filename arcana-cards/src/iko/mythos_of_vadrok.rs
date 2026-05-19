//! Mythos of Vadrok — `{2}{R}{R}` sorcery. "Mythos of Vadrok deals 5
//! damage divided as you choose among any number of target creatures
//! and/or planeswalkers. If {W}{U} was spent to cast this spell, until
//! your next turn, those permanents can't attack or block and their
//! activated abilities can't be activated."
//!
//! GAP: divided damage among multiple targets, mana-spent conditional,
//! and "can't attack or block / can't activate" effect are not
//! expressible. Best-effort: deal 5 damage to a single target creature.

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
    let name = reg.interner_mut().intern("Mythos of Vadrok");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mythos of Vadrok deals 5 damage divided as you choose among any number of target creatures and/or planeswalkers. If {W}{U} was spent to cast this spell, until your next turn, those permanents can't attack or block and their activated abilities can't be activated.".into(),
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
    // GAP: divided damage, mana-spent conditional, attack/block/activate restriction
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 5,
    }]
}
