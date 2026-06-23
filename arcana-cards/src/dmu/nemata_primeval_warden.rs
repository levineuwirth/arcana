//! Nemata, Primeval Warden — `{2}{B}{G}` 3/4 Legendary Creature — Treefolk.
//!
//! Reach.
//! If a creature an opponent controls would die, exile it instead. When you
//! do, create a 1/1 green Saproling creature token.
//! `{G}, Sacrifice a Saproling: Nemata gets +2/+2 until end of turn.`
//! `{1}{B}, Sacrifice two Saprolings: Draw a card.`
//!
//! Reach is a base keyword. The die-replacement clause ("would die, exile it
//! instead") plus its linked Saproling-token rider is a replacement effect —
//! not expressible with the demonstrated trigger/effect surface, so it's
//! GAP'd. Both activated abilities are wired: the Saproling sacrifices map to
//! `sacrifice_other` with a subtype filter (`discard_other_count`-style count
//! for "two Saprolings").

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nemata, Primeval Warden");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    // Subtype filter for the Saproling sacrifice costs.
    let saproling_filter = script::subtype_filter(reg, "Saproling");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: replacement effect "If a creature an opponent controls would die,
    // exile it instead. When you do, create a 1/1 green Saproling token." —
    // a self-installing replacement effect with a linked token rider, not
    // expressible with the demonstrated trigger/effect API.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, Sacrifice a Saproling: Nemata gets +2/+2 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    sacrifice_other: Some(saproling_filter.clone()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, Sacrifice two Saprolings: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    sacrifice_other: Some(saproling_filter),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            }),
    )
}

/// `{G}, Sacrifice a Saproling: Nemata gets +2/+2 until end of turn.`
fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// `{1}{B}, Sacrifice two Saprolings: Draw a card.`
fn draw_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
