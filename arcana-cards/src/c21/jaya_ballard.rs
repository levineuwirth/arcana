//! Jaya Ballard — `{2}{R}{R}{R}` Legendary Planeswalker — Jaya, starting loyalty 5.
//!
//! +1: Add {R}{R}{R} (spend only to cast instant or sorcery spells).
//!   Modeled as `AddMana` of three red; the "spend only on I/S"
//!   restriction is not expressible and is GAP'd.
//! +1: Discard up to three cards, then draw that many cards. The
//!   variable count ("up to three" / "draw that many") isn't
//!   expressible; approximated as discard 3, draw 3.
//! −8: You get an emblem with "You may cast instant and sorcery spells
//!   from your graveyard. If a spell cast this way would be put into
//!   your graveyard, exile it instead." The emblem's grant is a
//!   rule-altering static no anthem/keyword/filtered builder can
//!   express, so the emblem is created but its grant is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaya Ballard");
    let jaya = reg.interner_mut().intern("Jaya");
    let _emblem = reg.interner_mut().intern("Jaya Ballard emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{R}{R}. Spend this mana only to cast \
                       instant or sorcery spells.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Discard up to three cards, then draw that many \
                       cards.".into(),
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
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"You may cast instant and \
                       sorcery spells from your graveyard. If a spell cast \
                       this way would be put into your graveyard, exile it \
                       instead.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "spend only to cast instant or sorcery spells" restriction
    // not expressible; add three red mana.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Up to three / draw that many" is a dynamic count; approximate as
    // discard three, draw three.
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: 3,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: ctx.controller, count: 3 },
    ]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Jaya Ballard emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: "cast I/S from graveyard + exile-instead" is a
            // rule-altering static no anthem/keyword/filtered builder can
            // express.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
