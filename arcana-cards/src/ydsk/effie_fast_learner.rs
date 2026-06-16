//! Effie, Fast Learner — `{1}{G}{W}` 3/3 Legendary Human Survivor with
//! Enlist.
//! "Survival — At the beginning of your second main phase, if Effie is
//!  tapped, put a +1/+1 counter on each tapped creature you control.
//!  Then seek a Survivor card with mana value less than or equal to the
//!  number of tapped creatures you control."
//!
//! Keywords: Enlist is wired; Seek and Survival are not in the usable
//! keyword surface (Survival is just the trigger-naming word).
//! GAP: the "if Effie is tapped" intervening-if has no expressible
//! source-tapped condition predicate — the trigger fires unconditionally.
//! GAP: "seek a Survivor card …" — Seek is not an Effect variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Effie, Fast Learner");
    let human = reg.interner_mut().intern("Human");
    let survivor = reg.interner_mut().intern("Survivor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(survivor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Enlist],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PostCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: counter_each_tapped,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_each_tapped(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tapped_only(),
        trig.controller,
    );
    // GAP: "Then seek a Survivor card …" — Seek has no Effect variant.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
