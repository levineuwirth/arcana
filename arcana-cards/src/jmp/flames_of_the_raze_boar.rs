//! Flames of the Raze-Boar — `{5}{R}` instant, "Flames of the Raze-Boar deals 4
//! damage to target creature. If you control a creature with power 4 or greater,
//! Flames of the Raze-Boar also deals 2 damage to each other creature that player
//! controls."
//!
//! GAP: conditional 2 damage to each other creature controlled by target's
//! controller (requires checking whether you control a power-4+ creature — no
//! predicate for that in the Effect catalog).

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
    let name = reg.interner_mut().intern("Flames of the Raze-Boar");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flames of the Raze-Boar deals 4 damage to target creature. If you control a creature with power 4 or greater, Flames of the Raze-Boar also deals 2 damage to each other creature that player controls.".into(),
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
    // GAP: conditional 2 damage to each other creature that player controls if
    // you control a creature with power 4 or greater
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
