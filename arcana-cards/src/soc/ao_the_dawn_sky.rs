//! Ao, the Dawn Sky — `{3}{W}{W}` Legendary 5/4 Creature — Dragon Spirit.
//! Flying, vigilance.
//! "When Ao dies, choose one —
//!  • Look at the top seven cards of your library. Put any number of nonland
//!    permanent cards with total mana value 4 or less from among them onto the
//!    battlefield. Put the rest on the bottom of your library in a random order.
//!  • Put two +1/+1 counters on each permanent you control that's a creature or
//!    Vehicle."
//!
//! Decomposition:
//! - Keyword line: Flying, Vigilance.
//! - "When Ao dies, choose one — …" → a SelfDies triggered ability.
//!   GAP: a *triggered* "choose one" modal selection is not expressible with
//!   the documented API (modal dispatch is spell-only). Mode 1 (look at top 7,
//!   put any-number nonland permanents with total mv ≤ 4) is also not
//!   expressible. We emit Mode 2 (put two +1/+1 counters on each creature /
//!   Vehicle you control) as a best-effort, faithful death payoff.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ao, the Dawn Sky");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: ao_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// GAP: triggered "choose one" modal selection and Mode 1 (look at top 7, put
/// any-number nonland permanents with total mv ≤ 4) are not expressible. Emits
/// Mode 2 (two +1/+1 counters on each creature / Vehicle you control).
fn ao_dies(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if let Some(vehicle) = reg.interner().lookup("Vehicle") {
        let veh = ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .with_subtype_sym(vehicle);
        for id in script::ids_matching(state, &veh, trig.controller) {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    if ids.is_empty() {
        return Vec::new();
    }
    // Two +1/+1 counters on each = the AddCounters effect applied twice per id.
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::AddCounters {
                target: arcana_core::objects::NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::AddCounters {
                target: arcana_core::objects::NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
    ]
}
