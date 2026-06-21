//! Frodo, Sauron's Bane — `{W}` 1/2 Legendary Halfling Citizen.
//! {W/B}{W/B}: If Frodo is a Citizen, it becomes a Halfling Scout with base
//! power and toughness 2/3 and lifelink.
//! {B}{B}{B}: If Frodo is a Scout, it becomes a Halfling Rogue with
//! "Whenever this creature deals combat damage to a player, that player
//! loses the game if the Ring has tempted you four or more times this game.
//! Otherwise, the Ring tempts you."
//!
//! Ability 1 is partially expressed: the {W/B}{W/B} activation sets Frodo's
//! base power/toughness to 2/3 and grants lifelink (Duration::
//! WhileSourceOnBattlefield). The "if Frodo is a Citizen" subtype
//! precondition and the subtype change to Halfling Scout are NOT expressible
//! and are GAP'd.
//! Ability 2's effect is GAP'd entirely: the subtype change, the granted
//! Ring-tempting combat-damage trigger, and the "loses the game / Ring
//! tempts you" payoff have no engine primitives.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frodo, Sauron's Bane");
    let halfling = reg.interner_mut().intern("Halfling");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W/B}{W/B}: If Frodo is a Citizen, it becomes a Halfling Scout with base power and toughness 2/3 and lifelink.".into(),
                // GAP: "If Frodo is a Citizen" precondition and the subtype
                // change to Halfling Scout are not expressible; only the base
                // P/T set and lifelink grant are emitted.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W/B}{W/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_scout,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}{B}: If Frodo is a Scout, it becomes a Halfling Rogue with a Ring-tempting combat-damage ability.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_rogue,
            }),
    )
}

fn become_scout(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 2,
            toughness: 3,
            duration: Duration::WhileSourceOnBattlefield,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::WhileSourceOnBattlefield,
        },
    ]
}

fn become_rogue(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: subtype change to Halfling Rogue, the granted Ring-tempting
    // combat-damage trigger, the "loses the game if the Ring has tempted you
    // four or more times" payoff, and the Ring-tempts-you mechanic have no
    // engine primitives.
    Vec::new()
}
