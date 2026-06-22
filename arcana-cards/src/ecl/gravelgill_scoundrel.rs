//! Gravelgill Scoundrel — `{1}{U}` 1/3 Merfolk Rogue with Vigilance.
//! "Whenever this creature attacks, you may tap another untapped creature you
//! control. If you do, this creature can't be blocked this turn."
//!
//! GAP (effect): the trigger's payload is a resolution-time optional cost
//! ("you may tap another untapped creature you control") gating the
//! can't-be-blocked rider. OptionalPayment only supports Mana/Life costs —
//! there is no "tap a chosen creature" resolution-time payment, so the
//! conditional effect can't be expressed without dropping the cost (which
//! would make the card unconditionally unblockable — materially wrong). The
//! Vigilance keyword and the attack trigger are emitted; the effect is GAP'd.

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
    let name = reg.interner_mut().intern("Gravelgill Scoundrel");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: tap_then_unblockable,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tap_then_unblockable(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may tap another untapped creature you control. If you do,
    // this creature can't be blocked this turn." No resolution-time
    // tap-a-chosen-creature optional cost exists in the catalog.
    Vec::new()
}
