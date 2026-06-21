//! Arbalest Engineers — `{1}{R}{G}` 2/2 red-green Human Artificer.
//! When this creature enters, choose one —
//! • This creature deals 1 damage to any target.
//! • Put a +1/+1 counter on target creature. It gains trample and haste.
//! • Create a tapped Powerstone token.
//!
//! GAP: modal ("choose one —") is only expressible on a SPELL ability
//! (ModalSpec + dispatch_modal_effect via with_spell_ability); a
//! TRIGGERED ability has a single EffectFn and no mode-dispatch surface,
//! and the three modes carry incompatible target requirements that
//! cannot be declared on one trigger. The ETB trigger is recorded but
//! the modal body cannot be wired faithfully.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Arbalest Engineers");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: triggered-ability modal "choose one —" not expressible (no
    // mode dispatch on triggered abilities; modes have differing targets).
    Vec::new()
}
