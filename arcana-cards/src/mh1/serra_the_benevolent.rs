//! Serra the Benevolent — `{2}{W}{W}` Legendary Planeswalker — Serra,
//! starting loyalty 4. White.
//!
//! Oracle text:
//! * `+2`: Creatures you control with flying get +1/+1 until end of turn.
//! * `−3`: Create a 4/4 white Angel creature token with flying and
//!   vigilance.
//! * `−6`: You get an emblem with "If you control a creature, damage
//!   that would reduce your life total to less than 1 reduces it to 1
//!   instead."
//!
//! # Scope
//!
//! * `+2` is a flying-FILTERED anthem ("creatures you control WITH
//!   flying get +1/+1"); the demonstrated `Effect::Anthem` applies to
//!   all creatures you control with no subset filter, so the
//!   flying-only restriction is not expressible — GAP'd.
//! * `−3` mints the Angel token (fully expressible).
//! * `−6` grants an emblem with a damage-replacement static; emblem
//!   abilities carrying a replacement effect are bespoke — GAP'd, shell
//!   declared with the correct `−6` cost.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra the Benevolent");
    let serra = reg.interner_mut().intern("Serra");
    let _angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Creatures you control with flying get +1/+1 \
                       until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Create a 4/4 white Angel creature token with \
                       flying and vigilance.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"If you control a \
                       creature, damage that would reduce your life total \
                       to less than 1 reduces it to 1 instead.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

/// `+2`: flying-filtered anthem.
fn plus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: filtered anthem (only creatures you control WITH flying);
    // Effect::Anthem has no subset filter.
    Vec::new()
}

/// `−3`: create a 4/4 white flying-vigilance Angel token.
fn minus_three(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").expect("Angel interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let token = TokenDefinition {
        name: angel,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−6`: emblem with a life-loss damage-replacement static.
fn minus_six(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem carrying a damage-replacement static ("reduces it to 1
    // instead") is a bespoke replacement effect not expressible from the
    // demonstrated emblem surface.
    Vec::new()
}
