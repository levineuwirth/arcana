//! Gate of the Black Dragon — Land — Swamp Gate. "({T}: Add {B}.)
//! Gate of the Black Dragon enters the battlefield tapped. {3}{B}, {T}:
//! Seek a nonland card. Activate only once."
//!
//! Seek is an Arena-only mechanic with no engine support, and the
//! "Activate only once" (per game) restriction has no cost field —
//! both are documented gaps.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gate of the Black Dragon");
    let swamp = reg.interner_mut().intern("Swamp");
    let gate = reg.interner_mut().intern("Gate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(swamp);
    subtypes.0.insert(gate);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {B}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}, {T}: Seek a nonland card. Activate only once."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: seek_nonland,
            }),
    )
}

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn seek_nonland(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Seek a nonland card" — Seek is an Arena-only mechanic with
    // no Effect variant. "Activate only once" (per game) also has no
    // ActivationCost field.
    Vec::new()
}
