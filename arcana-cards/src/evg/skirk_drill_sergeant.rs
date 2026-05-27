//! Skirk Drill Sergeant — `{1}{R}` 2/1 red Goblin.
//! "Whenever this creature or another Goblin dies, you may pay {2}{R}. If you do, reveal the top card of your library. If it's a Goblin permanent card, put it onto the battlefield. Otherwise, put it into your graveyard."
//! GAP: OptionalPayment + conditional TutorToBattlefield-or-graveyard-from-top-of-library not fully expressible; using OptionalPayment with TutorToBattlefield as best approximation (graveyard fallback not modeled).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skirk Drill Sergeant");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    // GAP: "this creature or another Goblin" — can't filter for self OR subtype in one filter; using controlled_by(You) creature filter as approximation
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: maybe_cheat_goblin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_cheat_goblin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin_filter = arcana_core::script::subtype_filter(reg, "Goblin");
    // GAP: reveal-top-then-conditional-battlefield-or-graveyard not modeled;
    // using TutorToBattlefield as best effort (always puts a Goblin into play if found).
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{R}").expect("valid cost")),
        then: Box::new(Effect::TutorToBattlefield {
            player: trig.controller,
            filter: goblin_filter,
            tapped: false,
        }),
        else_effect: None,
    }]
}
