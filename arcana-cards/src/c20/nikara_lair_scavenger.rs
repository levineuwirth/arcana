//! Nikara, Lair Scavenger — `{2}{B}` 2/2 Legendary Human Cleric with Menace.
//!
//! Oracle:
//! * Partner with Yannik, Scavenging Sentinel. (GAP — Partner is not an
//!   expressible keyword/effect; the tutor-on-ETB rider is also not modeled.)
//! * Menace (keyword).
//! * Whenever another creature you control leaves the battlefield, if it had
//!   one or more counters on it, you draw a card and you lose 1 life.
//!
//! The leaves-the-battlefield trigger is approximated with a `ZoneChange`
//! from Battlefield → Graveyard over creatures you control (a fidelity gap:
//! exile/bounce leaves aren't covered, and the "another" exclusion relies on
//! the engine's self-source distinction). The "if it had one or more counters
//! on it" intervening clause has no matching condition helper, so it is GAP'd
//! (the trigger fires unconditionally on the death). The effect (draw 1, lose
//! 1 life) is expressible.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Nikara, Lair Scavenger");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // GAP: "Partner with Yannik, Scavenging Sentinel" — Partner is not an
    // expressible keyword; the search-for-Yannik ETB rider is not modeled.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Approximation of "leaves the battlefield" via dies (battlefield →
            // graveyard); covers the common case.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            // GAP: "if it had one or more counters on it" — no condition helper
            // inspects the leaving object's counters; fires unconditionally.
            intervening_if: None,
            effect: draw_and_lose,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_and_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: 1,
        },
    ])]
}
