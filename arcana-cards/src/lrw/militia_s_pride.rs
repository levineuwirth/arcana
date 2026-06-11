//! Militia's Pride — `{1}{W}` Kindred Enchantment — Kithkin (Lorwyn, 2007).
//! "Whenever a nontoken creature you control attacks, you may pay {W}. If
//! you do, create a 1/1 white Kithkin Soldier creature token that's tapped
//! and attacking."
//!
//! `CreatureAttacks` over nontoken creatures you control; the may-pay gate
//! is `Effect::OptionalPayment` minting the token. GAPs: the Kindred card
//! type (not modeled — registered as a plain Enchantment with the Kithkin
//! subtype) and the token entering "tapped and attacking" (CreateToken has
//! no tapped/attacking rider in this catalog).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Militia's Pride");
    // GAP: "Kindred" card type is not modeled; the Kithkin subtype is kept.
    let kithkin = reg.interner_mut().intern("Kithkin");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _token_name = reg.interner_mut().intern("Kithkin Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                },
                intervening_if: None,
                effect: muster_kithkin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {W}. If you do, create a 1/1 white Kithkin Soldier
/// creature token that's tapped and attacking."
fn muster_kithkin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Kithkin Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Kithkin") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Soldier") {
        subtypes.0.insert(s);
    }
    // GAP: the token should enter TAPPED AND ATTACKING; CreateToken has no
    // tapped/attacking rider in this catalog.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{W}").expect("valid cost"),
        ),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: token_name,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
