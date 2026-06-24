//! Drake Familiar — `{1}{U}` 2/1 Drake.
//!
//! Flying.
//! When this creature enters, sacrifice it unless you return an
//! enchantment to its owner's hand.
//!
//! The "sacrifice unless you return an enchantment" gate cannot be
//! expressed: the payment is a RETURN-TO-HAND, and OptionalPaymentKind only
//! covers Mana / Life / Sacrifice / Discard — there is no return-a-permanent
//! payment variant. Sacrificing unconditionally would be materially wrong (it
//! ignores the "unless"), so the ETB effect is GAP'd. The Flying keyword and
//! the trigger shell are faithful.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Drake Familiar");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_unless,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_unless(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it unless you return an enchantment to its owner's
    // hand" — the payment is a return-to-hand, which OptionalPaymentKind has
    // no variant for (only Mana/Life/Sacrifice/Discard). Unconditional
    // sacrifice would be wrong, so the whole effect is dropped.
    Vec::new()
}
