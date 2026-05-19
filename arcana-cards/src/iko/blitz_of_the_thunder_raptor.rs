//! Blitz of the Thunder-Raptor — `{1}{R}` instant. "Blitz of the
//! Thunder-Raptor deals damage to target creature or planeswalker equal to the
//! number of instant and sorcery cards in your graveyard. If that creature or
//! planeswalker would die this turn, exile it instead."
//!
//! # GAP: replacement effect "if would die this turn, exile instead"
//! # GAP: filter graveyard for instant OR sorcery type — graveyard_matching
//!   only takes a single ObjectFilter; TypeLine OR filtering uses with_types_any

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
    let name = reg.interner_mut().intern("Blitz of the Thunder-Raptor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Blitz of the Thunder-Raptor deals damage to target creature or planeswalker equal to the number of instant and sorcery cards in your graveyard. If that creature or planeswalker would die this turn, exile it instead.".into(),
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
    // GAP: replacement effect "if would die this turn, exile instead"
    // Counting instant+sorcery in graveyard via two graveyard_matching calls (summed)
    let instant_filter = ObjectFilter::new().with_types(TypeLine::INSTANT.into());
    let sorcery_filter = ObjectFilter::new().with_types(TypeLine::SORCERY.into());
    let instants = script::graveyard_matching(state, &instant_filter, entry.controller, entry.controller);
    let sorceries = script::graveyard_matching(state, &sorcery_filter, entry.controller, entry.controller);
    let amount = instants + sorceries;
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
