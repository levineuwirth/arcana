//! Necroskitter — `{1}{B}{B}` 1/4 Elemental with Wither.
//!
//! * Wither (keyword).
//! * "Whenever a creature an opponent controls with a -1/-1 counter on
//!   it dies, you may return that card to the battlefield under your
//!   control." — a `ZoneChange` (battlefield → graveyard) filtered to
//!   an opponent-controlled creature that has a -1/-1 counter; on
//!   resolution it returns the dying card to the battlefield. GAP: the
//!   "under your control" clause (no documented graveyard-return effect
//!   places the card under the controller rather than its owner) and
//!   the "you may" optionality (resolution-time choice not modeled);
//!   the card is returned under its owner's control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necroskitter");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    let dying_filter = ObjectFilter {
        has_counter: Some(CounterKind::MinusOneMinusOne),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: dying_filter,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: reanimate_dying,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reanimate_dying(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    // GAP: "under your control" + "you may" — returned under owner's control.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
