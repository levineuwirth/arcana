//! Flubs, the Fool — `{G}{U}{R}` 0/5 Legendary Frog Scout.
//! You may play an additional land on each of your turns.
//! Whenever you play a land or cast a spell, draw a card if you have no
//! cards in hand. Otherwise, discard a card.
//!
//! The "additional land each turn" static is GAP'd (no extra-land-play
//! primitive). The play-or-cast trigger is split into two defs — a land
//! ETB under your control ("play a land") and a spell cast you control — and
//! both branch at resolution on `script::hand_size`: empty → draw, else
//! discard.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flubs, the Fool");
    let frog = reg.interner_mut().intern("Frog");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(scout);

    // GAP: "You may play an additional land on each of your turns." — no
    // extra-land-play static primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever you play a land …" — a land entering under your control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_or_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "… or cast a spell".
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draw_or_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_or_discard(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    if script::hand_size(state, trig.controller) == 0 {
        vec![Effect::DrawCards { player: trig.controller, count: 1 }]
    } else {
        vec![Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        }]
    }
}
