//! Lydia Frye — `{2}{U/B}` 3/2 Legendary Human Assassin.
//!
//! * "Lydia Frye can't be blocked by creatures with power 3 or greater." —
//!   static evasion restriction; no filtered "can't be blocked by" Effect,
//!   so this GAPs.
//! * "At the beginning of your end step, surveil X, where X is the number of
//!   tapped Assassins you control." — end-step trigger; X computed at
//!   resolution as the count of tapped Assassins you control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lydia Frye");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    // GAP: "can't be blocked by creatures with power 3 or greater" — filtered
    // static evasion restriction not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_surveil_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_surveil_x(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of tapped Assassins you control.
    let filter = script::subtype_filter(reg, "Assassin")
        .controlled_by(ControllerConstraint::You)
        .tapped_only();
    let x = script::count_matching(state, &filter, trig.controller);
    vec![Effect::Surveil { player: trig.controller, count: x }]
}
