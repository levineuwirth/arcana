//! Wormwood Dryad — `{2}{G}` 3/1 Creature — Dryad.
//! {G}: This creature gains forestwalk until end of turn and deals 1 damage
//! to you.
//! {B}: This creature gains swampwalk until end of turn and deals 1 damage to
//! you.

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
    let name = reg.interner_mut().intern("Wormwood Dryad");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    // Pre-intern the basic land subtypes for the landwalk grants.
    let _forest = reg.interner_mut().intern("Forest");
    let _swamp = reg.interner_mut().intern("Swamp");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: This creature gains forestwalk until end of turn and deals 1 damage to you.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_forestwalk,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: This creature gains swampwalk until end of turn and deals 1 damage to you.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_swampwalk,
            }),
    )
}

fn gain_forestwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg
        .interner()
        .lookup("Forest")
        .expect("Forest interned during register()");
    vec![Effect::Sequence(vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Landwalk(forest),
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
        },
    ])]
}

fn gain_swampwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg
        .interner()
        .lookup("Swamp")
        .expect("Swamp interned during register()");
    vec![Effect::Sequence(vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Landwalk(swamp),
            duration: Duration::EndOfTurn,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
        },
    ])]
}
