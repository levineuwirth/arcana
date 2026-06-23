//! Moria Scavenger — `{1}{B}{R}` 1/4 Creature — Orc Rogue.
//!
//! * Deathtouch, haste (keywords).
//! * "{T}, Discard a card: Draw a card. If the discarded card was a
//!   creature card, amass Orcs 1." — a tap + discard-a-card activation.
//!   The "draw a card" half is wired faithfully. The conditional rider
//!   ("if the discarded card was a creature card, amass Orcs 1") cannot
//!   be expressed: the engine routes the discard as a cost and exposes
//!   no accessor for the discarded card's type at resolution, so the
//!   amass half is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moria Scavenger");
    let orc = reg.interner_mut().intern("Orc");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Discard a card: Draw a card. If the discarded card was a \
                   creature card, amass Orcs 1."
                .into(),
            cost: ActivationCost {
                tap: true,
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_a_card,
        }),
    )
}

fn draw_a_card(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional rider — "if the discarded card was a creature card,
    // amass Orcs 1." No accessor for the cost-discarded card's type.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
