//! Drake Haven — `{2}{U}` enchantment.
//! "Whenever you cycle or discard a card, you may pay {1}. If you do,
//! create a 2/2 blue Drake creature token with flying."
//!
//! Wired on `CardDiscarded(You)` — cycling pays a discard, so both
//! halves of "cycle or discard" route through the discard event.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drake Haven");
    let _drake = reg.interner_mut().intern("Drake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_for_drake,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}. If you do, create a 2/2 blue Drake creature
/// token with flying."
fn pay_for_drake(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let drake = reg.interner().lookup("Drake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: drake,
                colors: ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
