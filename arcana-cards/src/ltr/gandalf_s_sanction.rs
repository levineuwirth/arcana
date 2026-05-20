//! Gandalf's Sanction — `{1}{U}{R}` sorcery. "Gandalf's Sanction deals X
//! damage to target creature, where X is the number of instant and
//! sorcery cards in your graveyard. Excess damage is dealt to that
//! creature's controller instead."

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
    let name = reg.interner_mut().intern("Gandalf's Sanction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Gandalf's Sanction deals X damage to target creature, \
                   where X is the number of instant and sorcery cards in your \
                   graveyard. Excess damage is dealt to that creature's \
                   controller instead."
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let inst = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::INSTANT.into()),
        entry.controller,
        entry.controller,
    );
    let sorc = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::SORCERY.into()),
        entry.controller,
        entry.controller,
    );
    // "Excess damage to controller" rider is not expressible — damage only.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: inst + sorc,
    }]
}
