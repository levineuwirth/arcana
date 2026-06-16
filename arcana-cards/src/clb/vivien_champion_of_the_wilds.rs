//! Vivien, Champion of the Wilds — `{2}{G}` Legendary Planeswalker — Vivien, starting loyalty 4.
//!
//! Static: "You may cast creature spells as though they had flash." — a casting-
//!   permission static; not a loyalty ability and not expressible from the
//!   demonstrated surface. GAP (static, not modeled).
//! +1: Until your next turn, up to one target creature gains vigilance and reach.
//!   Modeled with two `Effect::GrantKeyword` (Duration::UntilYourNextTurn).
//! −2: Look at the top three cards of your library. Exile one face down and put
//!   the rest on the bottom; you may cast it if it's a creature spell. GAP: the
//!   look-3 / exile-face-down / cast-if-creature chain is approximated with
//!   `Effect::ImpulseExile` (count 3); the face-down + creature-only-cast nuance
//!   is not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien, Champion of the Wilds");
    let vivien = reg.interner_mut().intern("Vivien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature gains \
                       vigilance and reach.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_grant,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Look at the top three cards of your library. Exile one \
                       face down and put the rest on the bottom of your library \
                       in any order. For as long as it remains exiled, you may \
                       cast it if it's a creature spell.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_impulse,
            }),
    )
}

fn plus_one_grant(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    let pid = ctx.controller;
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::UntilYourNextTurn(pid),
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Reach,
            duration: Duration::UntilYourNextTurn(pid),
        },
    ]
}

fn minus_two_impulse(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: face-down exile + creature-only cast permission approximated by
    //      ImpulseExile over the top 3 cards.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 3 }]
}
