//! Garruk, Unleashed — `{2}{G}{G}` legendary planeswalker, starting
//! loyalty 3. Subtype Garruk; mono-green.
//!
//! Loyalty abilities:
//! * `+1`: Up to one target creature gets +3/+3 and gains trample until end
//!   of turn. (Functional via `Effect::Pump` with a trample keyword grant.)
//! * `−2`: Create a 3/3 green Beast creature token; then if an opponent
//!   controls more creatures than you, put a loyalty counter on Garruk.
//!   (Token is functional; the "more creatures than you" comparison gating
//!   the self-loyalty add isn't expressible — omitted.)
//! * `−7`: You get an emblem with a creature-tutor end-step trigger. GAP —
//!   emblem creation with a bespoke triggered ability.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Unleashed");
    let garruk = reg.interner_mut().intern("Garruk");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets +3/+3 and gains \
                       trample until end of turn."
                    .into(),
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
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 3/3 green Beast creature token. Then if an \
                       opponent controls more creatures than you, put a \
                       loyalty counter on Garruk."
                    .into(),
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
                effect: minus_two_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"At the beginning of your \
                       end step, you may search your library for a creature \
                       card, put it onto the battlefield, then shuffle.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_gap,
            }),
    )
}

fn plus_one_pump(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}

fn minus_two_beast(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    // Token is functional; the "if an opponent controls more creatures than
    // you" comparison gating a self-loyalty add is not expressible — omitted.
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_seven_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem creation with a bespoke end-step creature-tutor trigger.
    Vec::new()
}
