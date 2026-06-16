//! Daretti, Scrap Savant — `{3}{R}` Legendary Planeswalker — Daretti, starting
//! loyalty 3.
//!
//! +2: Discard up to two cards, then draw that many cards. GAP — "up to two,
//!   then draw THAT MANY" is a chosen variable linkage; `Effect::Discard` takes a
//!   fixed count and there is no demonstrated draw-equals-discarded primitive.
//!   Correct +2 cost retained, effect GAP'd.
//! −2: Sacrifice an artifact. If you do, return target artifact card from your
//!   graveyard to the battlefield. Modeled via Sacrifice + ReturnFromGraveyard.
//! −10: You get an emblem with "Whenever an artifact is put into your graveyard
//!   from the battlefield, return that card to the battlefield at the beginning
//!   of the next end step." Emblem shell retained; the trigger's "return THAT
//!   card" (the specific object from the firing event) at a delayed end step is
//!   not expressible from the demonstrated trigger-effect surface — GAP.
//! "Daretti, Scrap Savant can be your commander." — Commander-zone rule, not a
//!   loyalty ability; not modeled.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daretti, Scrap Savant");
    let daretti = reg.interner_mut().intern("Daretti");
    let _emblem = reg.interner_mut().intern("Daretti, Scrap Savant emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(daretti);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Discard up to two cards, then draw that many \
                       cards.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Sacrifice an artifact. If you do, return target \
                       artifact card from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_reanimate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: You get an emblem with \"Whenever an artifact is put \
                       into your graveyard from the battlefield, return that card \
                       to the battlefield at the beginning of the next end \
                       step.\"".into(),
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

fn plus_two_loot(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Discard up to two cards, then draw that many cards" — the discarded
    //      count is a player choice (0–2) and the draw count must equal it; this
    //      dynamic linkage is not expressible from the demonstrated Discard/Draw
    //      primitives (fixed counts only).
    let _ = DiscardChoice::ControllerChooses;
    Vec::new()
}

fn minus_two_reanimate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::Sacrifice {
            player: ctx.controller,
            filter: ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
            count: 1,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
    ]
}

fn minus_ten_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Daretti, Scrap Savant emblem").expect("name interned");
    // GAP: the emblem's triggered ability returns "THAT card" (the specific
    //      object from the firing artifact-dies event) at the next end step — a
    //      delayed, event-object-referencing return not expressible from the
    //      demonstrated trigger-effect surface. Emblem shell retained.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
