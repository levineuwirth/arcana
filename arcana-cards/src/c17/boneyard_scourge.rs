//! Boneyard Scourge — `{2}{B}{B}` 4/3 Zombie Dragon with Flying.
//! "Whenever a Dragon you control dies while this card is in your graveyard,
//!  you may pay {1}{B}. If you do, return this card from your graveyard to the
//!  battlefield."
//!
//! A graveyard-zone triggered ability: when one of your Dragons dies, you may
//! pay {1}{B} to return this card to the battlefield (Effect::OptionalPayment
//! wrapping a self-return).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boneyard Scourge");
    let zombie = reg.interner_mut().intern("Zombie");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // "a Dragon you control dies" → a Dragon-subtype creature you control
    // moving from the battlefield to a graveyard.
    let dragon_filter =
        script::subtype_filter(reg, "Dragon").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: dragon_filter,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: maybe_return_self,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn maybe_return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{B}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: trig.source,
        }),
        else_effect: None,
    }]
}
