//! Ral, Crackling Wit — `{2}{U}{R}` Legendary Planeswalker — Ral, starting loyalty 3.
//! Static triggered: whenever you cast a noncreature spell, put a loyalty counter on Ral.
//! +1: Create a 1/1 blue and red Otter token with prowess (prowess omitted — not a KeywordAbility variant).
//! -3: Draw three cards, then discard two cards.
//! -10: Draw three cards; emblem "Instant and sorcery spells you cast have storm" — storm not buildable,
//!      so the emblem is emitted as a shell (empty statics/abilities); the draw is implemented.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Crackling Wit");
    let sub = reg.interner_mut().intern("Ral");
    let otter = reg.interner_mut().intern("Otter");
    let _emblem = reg.interner_mut().intern("Ral, Crackling Wit emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let mut otter_subtypes = SubtypeSet::default();
    otter_subtypes.0.insert(otter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let _ = otter_subtypes; // used inside the +1 resolver via re-derivation

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_noncreature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 blue and red Otter creature token with \
                       prowess."
                    .into(),
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
                effect: minus_three_draw_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Draw three cards. You get an emblem with \"Instant \
                       and sorcery spells you cast have storm.\""
                    .into(),
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
                effect: minus_ten_emblem,
            }),
    )
}

/// Static trigger — on noncreature cast, add a loyalty counter to Ral.
fn on_noncreature_cast(
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

/// `+1` — create a 1/1 blue and red Otter token.
fn plus_one_otter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let otter = reg.interner().lookup("Otter").expect("Otter interned");
    let mut otter_subtypes = SubtypeSet::default();
    otter_subtypes.0.insert(otter);
    // GAP: prowess is not a KeywordAbility variant — token created without it.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: otter,
            colors: ColorSet::blue() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: otter_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `-3` — draw three, then discard two.
fn minus_three_draw_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 3,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

/// `-10` — draw three, get the storm emblem (storm GAP'd).
fn minus_ten_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Ral, Crackling Wit emblem")
        .expect("emblem name interned");
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 3,
        },
        Effect::CreateEmblem {
            controller: ctx.controller,
            emblem: EmblemDefinition {
                name: emblem_name,
                // GAP: "instant and sorcery spells you cast have storm" is not
                // buildable from anthem/keyword/filtered or a buildable trigger;
                // emit the emblem shell so the catalog records it.
                statics: vec![],
                abilities: vec![],
            },
        },
    ]
}
