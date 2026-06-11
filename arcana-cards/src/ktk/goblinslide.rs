//! Goblinslide — `{2}{R}` enchantment.
//! "Whenever you cast a noncreature spell, you may pay {1}. If you do,
//! create a 1/1 red Goblin creature token with haste."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblinslide");
    let _goblin = reg.interner_mut().intern("Goblin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_for_goblin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}. If you do, create a 1/1 red Goblin creature token
/// with haste."
fn pay_for_goblin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: goblin,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Haste],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
