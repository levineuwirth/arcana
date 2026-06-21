//! Subira, Tulzidi Caravanner — `{2}{R}` 2/3 Legendary Human Shaman with
//! Haste. "{1}: Another target creature with power 2 or less can't be
//! blocked this turn." plus "{1}{R}, {T}, Discard your hand: Until end of
//! turn, whenever a creature you control with power 2 or less deals
//! combat damage to a player, draw a card."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Subira, Tulzidi Caravanner");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}: Another target creature with power 2 or less can't be
            // blocked this turn." (The "another" self-exclusion isn't
            // expressible on the target filter — a documented fidelity
            // gap; the filter restricts to power 2 or less.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Another target creature with power 2 or less can't be blocked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_max_power(2),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_unblockable,
            })
            // "{1}{R}, {T}, Discard your hand: Until end of turn, whenever
            // a creature you control with power 2 or less deals combat
            // damage to a player, draw a card." (Effect GAP'd below.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, {T}, Discard your hand: Until end of turn, whenever a creature you control with power 2 or less deals combat damage to a player, draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    tap: true,
                    discard_hand: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_floating_draw,
            }),
    )
}

/// "Another target creature with power 2 or less can't be blocked this turn."
fn make_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CantBeBlocked {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}

/// GAP: "Until end of turn, whenever a creature you control with power 2
/// or less deals combat damage to a player, draw a card." — a floating,
/// board-wide combat-damage-watching triggered ability not bound to a
/// single creature is not expressible with GrantTriggeredAbility (which
/// grants to one target id). The cost (discard your hand, tap, {1}{R}) is
/// still modeled.
fn grant_floating_draw(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
