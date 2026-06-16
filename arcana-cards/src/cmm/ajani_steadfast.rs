//! Ajani Steadfast — `{3}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 4.
//!
//! +1: Until end of turn, up to one target creature gets +1/+1 and gains
//!   first strike, vigilance, and lifelink.
//! −2: Put a +1/+1 counter on each creature you control and a loyalty counter
//!   on each other planeswalker you control.
//! −7: You get an emblem with "If a source would deal damage to you or a
//!   planeswalker you control, prevent all but 1 of that damage."
//!
//! GAP: −2 affects "each creature you control" and "each other planeswalker
//!   you control" untargeted; the demonstrated Effect surface has no
//!   add-counters-to-each-matching-permanent variant. Ability shell declared
//!   with correct cost; effect GAP'd.
//! GAP: −7 grants an emblem; emblem creation is not in the demonstrated
//!   Effect surface. Ability shell declared with correct cost; effect GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani Steadfast");
    let sub = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +1: up to one target creature gets +1/+1 and FS, vigilance, lifelink EOT.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, up to one target creature gets +1/+1 and gains first strike, vigilance, and lifelink.".into(),
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
                effect: plus_one_pump,
            })
            // −2: counters on each creature/PW you control (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Put a +1/+1 counter on each creature you control and a loyalty counter on each other planeswalker you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_counters,
            })
            // −7: emblem (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"If a source would deal damage to you or a planeswalker you control, prevent all but 1 of that damage.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Lifelink,
        ],
    }]
}

fn minus_two_counters(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no add-counters-to-each-matching-permanent Effect variant.
    Vec::new()
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not in the demonstrated Effect surface.
    Vec::new()
}
