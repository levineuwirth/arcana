//! Burn the Accursed — `{4}{R}` instant, "Burn the Accursed deals 4 damage to
//! target creature. If that creature dies this turn, Burn the Accursed deals 2
//! damage to that creature's controller."
//!
//! GAP: (1) exile-instead-of-die replacement clause; (2) damage to the targeted
//! creature's controller (requires reading controller field from the object, not
//! from a TargetChoice::Player) — best effort: DealDamage 4 to target creature.

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
    let name = reg.interner_mut().intern("Burn the Accursed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Burn the Accursed deals 4 damage to target creature. If that creature dies this turn, Burn the Accursed deals 2 damage to that creature's controller.".into(),
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
    // GAP: triggered "if dies this turn, deal 2 to controller" and controller lookup
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
