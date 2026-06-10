//! Trenzalore Clocktower — legendary land (Doctor Who, 2023).
//! "{T}: Add {U}. Put a time counter on Trenzalore Clocktower." and
//! "{1}{U}, {T}, Remove twelve time counters from Trenzalore Clocktower
//! and exile it: Shuffle your graveyard and hand into your library, then
//! draw seven cards. Activate only if you control a Time Lord."
//! The first ability adds a counter rider, so it cannot be a CR 605 mana
//! ability here (the engine's mana abilities must be AddMana-only) — a
//! documented fidelity note.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trenzalore Clocktower");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}. Put a time counter on Trenzalore Clocktower."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                // Not a pure mana ability: the counter rider forces the
                // stack path (engine mana abilities must be AddMana-only).
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_and_count,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, {T}, Remove twelve time counters from Trenzalore Clocktower and exile it: Shuffle your graveyard and hand into your library, then draw seven cards. Activate only if you control a Time Lord."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    tap: true,
                    remove_self_counter: Some((CounterKind::Time, 12)),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                // GAP: 'Activate only if you control a Time Lord' — activation
                // preconditions are not expressible on ActivationCost here.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: timetwist_draw_seven,
            }),
    )
}

fn add_blue_and_count(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Time,
            count: 1,
        },
    ]
}

fn timetwist_draw_seven(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Shuffle your graveyard and hand into your library' is not
    // expressible with any catalog Effect; only the draw is wired.
    vec![Effect::DrawCards { player: ctx.controller, count: 7 }]
}
