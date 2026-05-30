//! Dreamshaper Shaman — `{5}{R}` 5/4 red Enchantment Creature — Minotaur Shaman.
//! "At the beginning of your end step, you may pay {2}{R} and sacrifice a nonland
//! permanent. If you do, reveal cards from the top of your library until you reveal
//! a nonland permanent card. Put that card onto the battlefield and the rest on the
//! bottom of your library in a random order."
//! GAP: OptionalPayment only supports Mana or Life costs; the additional
//! "sacrifice a nonland permanent" cost cannot be expressed in OptionalPaymentKind.
//! Emitting the RevealUntil effect without the optional-payment/sacrifice gate.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreamshaper Shaman");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
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
                effect: end_step_reveal_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_reveal_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {2}{R} and sacrifice a nonland permanent" optional cost not
    // expressible — OptionalPaymentKind only supports Mana or Life, not sacrifice.
    // Emitting the RevealUntil effect unconditionally as best-effort.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}
