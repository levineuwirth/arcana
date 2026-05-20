//! Spite of Mogis — `{R}` sorcery. "Spite of Mogis deals damage to
//! target creature equal to the number of instant and sorcery cards
//! in your graveyard. Scry 1."
//!
//! Dynamic damage = count of instant+sorcery cards in your graveyard.
//! Computed via graveyard_matching with an instant/sorcery filter.

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
    let name = reg.interner_mut().intern("Spite of Mogis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Spite of Mogis deals damage to target creature equal to the number of instant and sorcery cards in your graveyard. Scry 1.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let instants = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::INSTANT.into()),
        entry.controller,
        entry.controller,
    );
    let sorceries = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::SORCERY.into()),
        entry.controller,
        entry.controller,
    );
    let amount = instants + sorceries;
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount,
        },
        Effect::Scry {
            player: entry.controller,
            count: 1,
        },
    ]
}
