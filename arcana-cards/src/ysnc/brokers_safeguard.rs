//! Brokers' Safeguard — `{W}{U}` instant. "Exile target nonartifact creature
//! you control. It perpetually gains 'This creature enters the battlefield with
//! an additional shield counter on it.' Then return that card to the battlefield
//! under its owner's control."
//!
//! # GAP: perpetually modify a card's abilities
//! # GAP: exile then immediately return to battlefield as part of one effect
//! Best-effort: exile the target (the return and perpetual grant are not
//! expressible).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brokers' Safeguard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target nonartifact creature you control. It perpetually gains \"This creature enters the battlefield with an additional shield counter on it.\" Then return that card to the battlefield under its owner's control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .without_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: perpetually grant ability to a card
    // GAP: exile-then-immediately-return-to-battlefield as atomic pair
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: *id }]
}
