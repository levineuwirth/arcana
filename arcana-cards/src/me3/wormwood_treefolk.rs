//! Wormwood Treefolk — `{3}{G}{G}` 4/4 Treefolk (G).
//!
//! Oracle:
//! * {G}{G}: This creature gains forestwalk until end of turn and deals
//!   2 damage to you.
//! * {B}{B}: This creature gains swampwalk until end of turn and deals
//!   2 damage to you.
//!
//! Two mana activations, each granting the matching landwalk to itself
//! for the turn and dealing 2 damage to its controller. The basic-land
//! subtype symbols ("Forest"/"Swamp") are interned at registration so
//! the resolver can recover them via the read-only interner handle.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wormwood Treefolk");
    let treefolk = reg.interner_mut().intern("Treefolk");
    // Intern the basic-land subtype names so the resolvers can recover
    // them via reg.interner().lookup(...) at resolution time.
    let _forest = reg.interner_mut().intern("Forest");
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{G}: This creature gains forestwalk until end of turn \
                       and deals 2 damage to you."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: forestwalk_and_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}: This creature gains swampwalk until end of turn \
                       and deals 2 damage to you."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: swampwalk_and_damage,
            }),
    )
}

fn forestwalk_and_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest").unwrap_or_default();
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Landwalk(forest),
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: 2,
        },
    ]
}

fn swampwalk_and_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").unwrap_or_default();
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Landwalk(swamp),
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: 2,
        },
    ]
}
