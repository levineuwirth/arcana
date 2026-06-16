//! Visitor from Planet Q — `{1}{U}` 0/4 Instant Creature — Alien.
//!
//! Type line is Instant Creature: TypeLine(INSTANT | CREATURE).
//!
//! Line 1 (static): "All creature cards you own with flash are
//!   instants in addition to their other types." — a pure static
//!   continuous type-adding effect, not a trigger/cost. GAP'd.
//! Line 2: "Whenever you cast another spell with two or more card
//!   types, you may draw a card, then discard a card." — modeled as a
//!   SpellCast trigger (caster: You) with a draw-then-discard effect.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP (static): "All creature cards you own with flash are instants in
// addition to their other types." — a pure static continuous
// type-adding effect; not expressible as a triggered/activated ability
// in this card class. Omitted.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visitor from Planet Q");
    let alien = reg.interner_mut().intern("Alien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::INSTANT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // FIDELITY GAP: the "another spell with two or more card
            // types" restriction (and the "another" self-exclusion) is
            // not expressible as an ObjectFilter, so this fires on every
            // spell you cast. The "may" is a resolution-time choice we
            // approximate by always drawing-then-discarding.
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_then_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Draw a card, then discard a card.
fn draw_then_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
