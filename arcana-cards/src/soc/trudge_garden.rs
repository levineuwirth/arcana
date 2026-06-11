//! Trudge Garden — `{2}{G}` enchantment.
//! "Whenever you gain life, you may pay {2}. If you do, create a 4/4
//! green Fungus Beast creature token with trample."

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
    let name = reg.interner_mut().intern("Trudge Garden");
    let _token_name = reg.interner_mut().intern("Fungus Beast");
    let _fungus = reg.interner_mut().intern("Fungus");
    let _beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_for_fungus_beast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {2}. If you do, create a 4/4 green Fungus Beast
/// creature token with trample."
fn pay_for_fungus_beast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Fungus Beast").unwrap_or_default();
    let fungus = reg.interner().lookup("Fungus").unwrap_or_default();
    let beast = reg.interner().lookup("Beast").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(beast);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{2}").expect("valid cost"),
        ),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: token_name,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![KeywordAbility::Trample],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
