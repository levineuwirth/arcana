//! Rebellion of the Flamekin — `{3}{R}` Kindred Enchantment —
//! Elemental (Morningtide, 2008). "Whenever you clash, you may pay
//! {1}. If you do, create a 3/1 red Elemental Shaman creature token.
//! If you won, that token gains haste until end of turn."
//!
//! The Kindred card type and the clash mechanic are not modeled: types
//! carry the Enchantment bit plus the Elemental subtype, and the
//! trigger is wired on the closest (inert for an enchantment)
//! `SelfBecomesTapped` placeholder with an honest GAP. The
//! pay-{1}-then-token payoff is wired; the clash-won haste rider is a
//! GAP.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rebellion of the Flamekin");
    let elemental = reg.interner_mut().intern("Elemental");
    let _shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        // GAP: the Kindred card type is not a TypeLine const; only the
        // Enchantment bit (plus the Elemental subtype) is recorded.
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you clash" (the Lorwyn clash
                // mechanic) has no TriggerCondition; SelfBecomesTapped is the
                // closest placeholder (inert for this enchantment).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: pay_for_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {1}. If you do, create a 3/1 red Elemental Shaman
/// creature token."
fn pay_for_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you won, that token gains haste until end of turn" — clash
    // results are not modeled.
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let shaman = reg.interner().lookup("Shaman").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: elemental,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
