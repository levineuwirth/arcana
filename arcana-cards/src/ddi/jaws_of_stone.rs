//! Jaws of Stone — `{5}{R}` sorcery, "Jaws of Stone deals X damage
//! divided as you choose among any number of targets, where X is the
//! number of Mountains you control as you cast this spell."
//!
//! Uses `Effect::DealDamageDivided` over any number of any-targets, with
//! `total` computed as the number of Mountains the controller has on the
//! battlefield. (The player's exact division of the damage is a
//! documented fidelity gap; the engine spreads `total` across the chosen
//! targets.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaws of Stone");
    let _mountain = reg.interner_mut().intern("Mountain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Jaws of Stone deals X damage divided as you choose among any number of targets, where X is the number of Mountains you control as you cast this spell."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::AnyTarget,
                count: TargetCount::Any,
                controller: None,
            }],
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
    let targets: Vec<DamageTarget> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
                Some(DamageTarget::Object(*id))
            }
            TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
                Some(DamageTarget::Player(*p))
            }
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    let mountain = reg
        .interner()
        .lookup("Mountain")
        .expect("Mountain interned during register()");
    let total = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(vec![mountain])
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![Effect::DealDamageDivided {
        source: entry.source,
        targets,
        total,
    }]
}
