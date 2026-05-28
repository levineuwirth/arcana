//! Crucias, Titan of the Waves — `{1}{B}{R}` 3/1 legendary black/red Human
//! Pirate. "At the beginning of your end step, you may discard a card. If you
//! do, create a Treasure token and choose ambitious or expedient. If you chose
//! ambitious, seek a card with greater mana value than the discarded card. If
//! you chose expedient, seek a card with lesser mana value."
//!
//! GAP: "seek a card" (Arena-only mechanic) and "choose ambitious or expedient"
//! (modal choice with CMC comparison to discarded card) not expressible.
//! Treasure token and discard are wired.

use arcana_core::effects::{CommodityToken, DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crucias, Titan of the Waves");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
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
                effect: discard_seek,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn discard_seek(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a card" (Arena-only) and modal choice with CMC comparison not expressible.
    // Wiring optional discard + treasure.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: arcana_core::actions::OptionalPaymentKind::Life(0),
        then: Box::new(Effect::Sequence(vec![
            Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses },
            Effect::CreateCommodityToken { controller: trig.controller, kind: CommodityToken::Treasure, count: 1 },
        ])),
        else_effect: None,
    }]
}
