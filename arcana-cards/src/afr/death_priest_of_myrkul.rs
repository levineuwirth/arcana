//! Death-Priest of Myrkul — `{2}{B}{B}` 2/2 Tiefling Cleric.
//! Skeletons, Vampires, and Zombies you control get +1/+1 (static,
//! GAP'd — not a triggered/activated ability). At the beginning of your
//! end step, if a creature died this turn, you may pay {1}; if you do,
//! create a 1/1 black Skeleton creature token.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::effects::TokenDefinition;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death-Priest of Myrkul");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(cleric);
    // Pre-intern the Skeleton token subtype so the resolver can recover it.
    let _skeleton = reg.interner_mut().intern("Skeleton");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static anthem "Skeletons, Vampires, and Zombies you control
    // get +1/+1" is a continuous static, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if a creature died this turn" — no
                // generic "a creature died this turn" predicate is exposed
                // (only subtype-scoped died-this-turn counts). Left None.
                intervening_if: None,
                effect: end_step_pay_make_skeleton,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_pay_make_skeleton(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let skeleton = reg.interner().lookup("Skeleton").unwrap_or_default();
    let mut sub = SubtypeSet::default();
    sub.0.insert(skeleton);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: skeleton,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: sub,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
