//! Archfiend of Depravity — `{3}{B}{B}` 5/4 Demon.
//!
//! Oracle:
//! * Flying — `KeywordAbility::Flying`.
//! * "At the beginning of each opponent's end step, that player chooses up
//!   to two creatures they control, then sacrifices the rest." — an
//!   opponent-end-step trigger. "Choose up to two to keep, sacrifice the
//!   rest" is modeled as `Effect::Sacrifice` of (their creature count − 2)
//!   creatures they control: the player picks which to sacrifice, which is
//!   equivalent to picking which two to keep.

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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archfiend of Depravity");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: sacrifice_all_but_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn sacrifice_all_but_two(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The end step belongs to the active player — the opponent here.
    let them = state.active_player();
    let their_creatures =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &their_creatures, them);
    if n <= 2 {
        return Vec::new();
    }
    vec![Effect::Sacrifice {
        player: them,
        filter: their_creatures,
        count: n - 2,
    }]
}
