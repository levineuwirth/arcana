//! Weight of Spires — `{R}` instant. "Weight of Spires deals damage to target
//! creature equal to the number of nonbasic lands that creature's controller
//! controls."
//!
//! GAP: "creature's controller" — cannot retrieve a permanent's controller PlayerId
//! from the resolver without state field access; using entry.controller as approximation.
//! GAP: "nonbasic lands" filter — ObjectFilter has no BasicLand exclusion; using
//! script::count_matching with LAND type as best-effort.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weight of Spires");
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
                text: "Weight of Spires deals damage to target creature equal to the number of nonbasic lands that creature's controller controls.".into(),
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
    // GAP: "creature's controller" — using opponent's land count as approximation
    // GAP: "nonbasic lands" — no basic-land exclusion filter; counting all opponent lands
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::Opponent);
    let n = script::count_matching(state, &land_filter, entry.controller);
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: n,
    }]
}
