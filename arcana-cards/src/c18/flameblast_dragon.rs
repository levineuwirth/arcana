//! Flameblast Dragon — `{4}{R}{R}` 5/5 Dragon with Flying.
//! "Whenever this creature attacks, you may pay {X}{R}. If you do, it
//! deals X damage to any target."
//!
//! Flying is a base keyword. The attack trigger is wired with the
//! correct `SelfAttacks` condition, but its body is a per-ability GAP:
//! the optional payment is a VARIABLE-X mana cost ({X}{R}) whose paid X
//! determines the damage dealt. `OptionalPaymentKind` only exposes a
//! FIXED `Mana(ManaCost)` / `Life(u32)` cost (no X form), and there is
//! no way to feed the chosen X into a subsequent DealDamage amount — so
//! the optional-payment-then-deal-X-damage clause is unexpressible with
//! the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Flameblast Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_pay_x_deal_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_pay_x_deal_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}{R}. If you do, it deals X damage to any
    // target." The variable-X optional mana payment and feeding the
    // paid X into a DealDamage amount are not expressible —
    // OptionalPaymentKind has only Mana(ManaCost)/Life(u32) (fixed, no
    // X), and there is no API to read the chosen X into the damage.
    Vec::new()
}
