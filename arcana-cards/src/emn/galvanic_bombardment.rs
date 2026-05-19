//! Galvanic Bombardment — `{R}` instant. "Galvanic Bombardment deals X damage
//! to target creature, where X is 2 plus the number of cards named Galvanic
//! Bombardment in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galvanic Bombardment");
    let _gb = reg.interner_mut().intern("Galvanic Bombardment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Galvanic Bombardment deals X damage to target creature, where X is 2 plus the number of cards named Galvanic Bombardment in your graveyard.".into(),
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
    let gb_filter = script::subtype_filter(reg, "Galvanic Bombardment");
    let copies_in_gy = script::graveyard_matching(state, &gb_filter, entry.controller, entry.controller);
    let amount = 2 + copies_in_gy;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
