//! Kheru Lich Lord — `{3}{B}{G}{U}` 4/4 black/green/blue Creature — Zombie Wizard.
//! "At the beginning of your upkeep, you may pay {2}{B}. If you do, return a creature card at
//! random from your graveyard to the battlefield. It gains flying, trample, and haste. Exile that
//! card at the beginning of your next end step. If it would leave the battlefield, exile it instead
//! of putting it anywhere else."
//! GAP: random graveyard reanimate — Reanimate picks by filter, not randomly; best-effort used.
//! GAP: 'if it would leave the battlefield, exile it instead' — replacement effect not modeled.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kheru Lich Lord");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: random graveyard return not available; Reanimate uses filter-based selection.
    // GAP: grant flying/trample/haste to the reanimated creature — object id unknown at resolution.
    // GAP: delayed exile at next end step of returned creature — object id unknown at resolution.
    // GAP: replacement effect (leave battlefield → exile) not modeled.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{B}").expect("valid cost")),
        then: Box::new(Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            from_zone: Zone::Graveyard(trig.controller),
        }),
        else_effect: None,
    }]
}
