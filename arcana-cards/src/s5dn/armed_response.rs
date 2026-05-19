//! Armed Response — `{2}{W}` instant, "Armed Response deals damage to target
//! attacking creature equal to the number of Equipment you control."
//! Equipment count uses `script::count_matching` with an Equipment subtype
//! filter. The "attacking creature" constraint on the target is not expressible
//! as a TargetFilter, so best effort targets any creature.
//!
//! # GAP: attacking-creature target constraint (no TargetFilter for attacking creatures)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armed Response");
    let _equipment = reg.interner_mut().intern("Equipment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Armed Response deals damage to target attacking creature equal to the number of Equipment you control.".into(),
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
    // GAP: attacking-creature target constraint (no TargetFilter for attacking creatures)
    let equipment_filter = script::subtype_filter(reg, "Equipment");
    let n = script::count_matching(state, &equipment_filter, entry.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: n,
    }]
}
