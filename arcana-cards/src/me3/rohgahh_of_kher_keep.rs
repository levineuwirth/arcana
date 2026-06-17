//! Rohgahh of Kher Keep — `{2}{B}{B}{R}{R}` 5/5 legendary Kobold.
//! "At the beginning of your upkeep, you may pay {R}{R}{R}. If you
//! don't, tap Rohgahh and all creatures named Kobolds of Kher Keep, then
//! an opponent gains control of them."
//! "Creatures you control named Kobolds of Kher Keep get +2/+2." — GAP
//! (static anthem, not expressible).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Rohgahh of Kher Keep");
    let kobold = reg.interner_mut().intern("Kobold");
    let _kobolds_name = reg.interner_mut().intern("Kobolds of Kher Keep");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);

    // GAP: static "Creatures you control named Kobolds of Kher Keep get
    // +2/+2" — no expressible anthem static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_pay_or_lose,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_pay_or_lose(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "If you don't [pay {R}{R}{R}]", tap Rohgahh and all creatures named
    // Kobolds of Kher Keep, then an opponent gains control of them.
    let kobold_name = reg.interner().lookup("Kobolds of Kher Keep");
    let filter = ObjectFilter {
        name: kobold_name,
        ..ObjectFilter::creature()
    };
    let mut ids = script::ids_matching(state, &filter, trig.controller);
    ids.push(trig.source);

    let opponents = script::opponents(state, trig.controller);
    let mut punishment: Vec<Effect> = Vec::new();
    for id in &ids {
        punishment.push(Effect::Tap { target: *id });
    }
    if let Some(opp) = opponents.first() {
        for id in &ids {
            punishment.push(Effect::ChangeControl {
                target: *id,
                new_controller: *opp,
            });
        }
    }

    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sequence(punishment))),
    }]
}
