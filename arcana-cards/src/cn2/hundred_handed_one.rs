//! Hundred-Handed One — `{2}{W}{W}` 3/5 white Giant with Vigilance.
//!
//! Oracle text:
//! * Vigilance.
//! * {3}{W}{W}{W}: Monstrosity 3.
//! * As long as this creature is monstrous, it has reach and can block
//!   an additional ninety-nine creatures each combat.
//!
//! Implemented: the Vigilance keyword, and the {3}{W}{W}{W} activated
//! ability is wired with its correct mana cost.
//!
//! GAP: Monstrosity (CR 701.30) has no `Effect` variant — the activated
//! ability's effect (put three +1/+1 counters on this creature and make
//! it monstrous) is not expressible, so its resolver returns no effects.
//! GAP: the "as long as this creature is monstrous, …" static is a
//! continuous ability gated on the (unmodeled) monstrous state —
//! omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hundred-Handed One");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}{W}{W}: Monstrosity 3.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}{W}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_3,
        }),
    )
}

fn monstrosity_3(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Monstrosity 3 — no `Effect` variant expresses "if not
    // monstrous, put three +1/+1 counters on it and it becomes
    // monstrous".
    Vec::new()
}
