//! Ral, Crackling Wit — `{2}{U}{R}` Legendary Planeswalker — Ral, starting
//! loyalty 4. Colors R, U.
//!
//! Whenever you cast a noncreature spell, put a loyalty counter on Ral.
//! (Modeled as a triggered ability.)
//! +1: Create a 1/1 blue and red Otter creature token with prowess. (Prowess
//!     isn't an available keyword — the token is created without it; GAP.)
//! −3: Draw three cards, then discard two cards.
//! −10: Draw three cards. You get an emblem with "Instant and sorcery spells
//!      you cast have storm." (Partial — the draw is expressed; the emblem
//!      is a GAP.)

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Crackling Wit");
    let ral = reg.interner_mut().intern("Ral");
    let _otter = reg.interner_mut().intern("Otter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::permanent().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: noncreature_cast_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 blue and red Otter creature token with prowess.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_otter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Draw three cards, then discard two cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Draw three cards. You get an emblem with \"Instant and sorcery spells you cast have storm.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_draw_emblem,
            }),
    )
}

fn noncreature_cast_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn plus_one_otter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: prowess isn't an available keyword — the Otter is created without
    //      it.
    let otter = reg.interner().lookup("Otter").expect("Otter interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(otter);
    let token = TokenDefinition {
        name: otter,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_three_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 3 },
        Effect::Discard {
            player: ctx.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn minus_ten_draw_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: the emblem ("Instant and sorcery spells you cast have storm")
    //          is a GAP — emblem-borne keyword grant not expressible. The
    //          draw is expressed faithfully.
    vec![Effect::DrawCards { player: ctx.controller, count: 3 }]
}
