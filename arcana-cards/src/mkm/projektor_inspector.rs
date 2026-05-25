//! Projektor Inspector — `{2}{U}` 3/2 blue Human Detective. "Whenever this creature or
//! another Detective you control enters and whenever a Detective you control is turned
//! face up, you may draw a card. If you do, discard a card."
//!
//! GAP: trigger — Two trigger clauses are combined: (1) a Detective ETB (ZoneChange
//! watching Detectives entering under your control) and (2) "turned face up" — there
//! is no TriggerCondition variant for a permanent being turned face up. Using only the
//! ZoneChange ETB clause as the closest expressible match; the face-up clause is
//! omitted and the verify pipeline will flag it.
//!
//! GAP: trigger — The ETB clause says "this creature OR another Detective you control".
//! ZoneChange fires on ANY qualifying entering permanent, including self. The "another"
//! exclusion (so this creature's own ETB triggers separately via SelfEntersBattlefield)
//! is not enforceable here; the trigger will fire on all Detectives entering including
//! self, which is a slight over-trigger relative to the printed text — flagged for
//! verify.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Projektor Inspector");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — fires on Detective ETB only; "turned face up" clause
                // omitted (no TriggerCondition variant). Also over-triggers on self ETB
                // versus the "another" restriction in the oracle.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtypes_any(vec![detective]),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: may_draw_then_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn may_draw_then_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may draw a card. If you do, discard a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(0),
        then: Box::new(Effect::Sequence(vec![
            Effect::DrawCards { player: trig.controller, count: 1 },
            Effect::Discard {
                player: trig.controller,
                count: 1,
                choice: DiscardChoice::ControllerChooses,
            },
        ])),
        else_effect: None,
    }]
}
