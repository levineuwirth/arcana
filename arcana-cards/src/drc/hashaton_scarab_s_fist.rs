//! Hashaton, Scarab's Fist — `{W}{B}` 1/3 Legendary Zombie Wizard.
//! "Whenever you discard a creature card, you may pay {2}{U}. If you
//! do, create a tapped token that's a copy of that card, except it's a
//! 4/4 black Zombie."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hashaton, Scarab's Fist");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger can't be filtered to "a creature card" — CardDiscarded
            // has no filter field; fires on any discard you make.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_discard_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_discard_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{U}").expect("valid cost")),
        // GAP: "create a tapped token that's a copy of the discarded creature card,
        // except it's a 4/4 black Zombie" is not expressible — there is no copy-of-a-
        // discarded-card token effect (CopyPermanent copies a battlefield permanent,
        // not a card in the graveyard) and no per-token tapped/PT/color override.
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: None,
    }]
}
