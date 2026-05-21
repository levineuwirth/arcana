//! Practiced Tactics — `{W}` instant. "Choose target attacking or
//! blocking creature. Practiced Tactics deals damage to that creature
//! equal to twice the number of creatures in your party." 'Party'
//! (one each of Cleric/Rogue/Warrior/Wizard) requires per-subtype
//! counts; we approximate by checking each of the four party subtypes
//! and summing 'has at least one' to get N (capped at 4). 'Attacking
//! or blocking' is a GAP on the target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Practiced Tactics");
    let _ = reg.interner_mut().intern("Cleric");
    let _ = reg.interner_mut().intern("Rogue");
    let _ = reg.interner_mut().intern("Warrior");
    let _ = reg.interner_mut().intern("Wizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    // GAP: 'attacking or blocking' combat-state filter on target.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target attacking or blocking creature. Practiced Tactics deals damage to that creature equal to twice the number of creatures in your party.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut party: u32 = 0;
    for ty in ["Cleric", "Rogue", "Warrior", "Wizard"] {
        let f = script::subtype_filter(reg, ty).controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &f, entry.controller) > 0 {
            party += 1;
        }
    }
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: party * 2,
    }]
}
