//! Sorin the Mirthless — `{2}{B}{B}` Legendary Planeswalker — Sorin,
//! starting loyalty 4.
//!
//! * `+1`: Look at the top card; you may reveal it and put it into your
//!   hand, losing life equal to its mana value. GAP'd (top-of-library
//!   conditional reveal + dynamic mana-value life loss not expressible).
//! * `−2`: Create a 2/3 black Vampire creature token with flying and
//!   lifelink. Expressed via `Effect::CreateToken`.
//! * `−7`: Sorin deals 13 damage to any target. You gain 13 life.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin the Mirthless");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top card of your library. You may reveal \
                       that card and put it into your hand. If you do, you lose \
                       life equal to its mana value.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_look,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 2/3 black Vampire creature token with flying \
                       and lifelink.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_vampire,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Sorin deals 13 damage to any target. You gain 13 \
                       life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_drain,
            }),
    )
}

fn plus_one_look(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: look at top, optional reveal-to-hand with dynamic mana-value
    // life loss — not expressible in the demonstrated surface.
    Vec::new()
}

fn minus_two_vampire(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire")
        .expect("Vampire interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_seven_drain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(_) => match target.object_id() {
            Some(id) => DamageTarget::Object(id),
            None => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage { source: ctx.source, target: dt, amount: 13 },
        Effect::GainLife { player: ctx.controller, amount: 13 },
    ]
}
