//! Jegantha, the Wellspring — `{4}{R/G}` 5/5 Legendary Creature — Elemental Elk.
//!
//! Companion — No card in your starting deck has more than one of the same
//! mana symbol in its mana cost. (deck-construction restriction; not modeled.)
//! `{T}: Add {W}{U}{B}{R}{G}. This mana can't be spent to pay generic mana costs.`
//!
//! Companion is not in the usable keyword surface (a deck-building/sideboard
//! mechanic), so `keywords: vec![]`. The tap-for-five-colors mana ability is
//! wired as an `ActivatedAbilityDef`; the "can't pay generic costs" rider is a
//! fidelity GAP (mana-spend restrictions aren't an effect field).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jegantha, the Wellspring");
    let elemental = reg.interner_mut().intern("Elemental");
    let elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(elk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R/G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP keyword: Companion — deck-construction restriction, not modeled.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {W}{U}{B}{R}{G}. This mana can't be spent to pay generic mana costs."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_wubrg,
            }),
    )
}

/// `{T}: Add {W}{U}{B}{R}{G}`. The "can't pay generic costs" rider is a
/// fidelity GAP (mana-spend restrictions are not an engine field).
fn add_wubrg(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
