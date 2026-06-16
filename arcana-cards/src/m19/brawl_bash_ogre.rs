//! Brawl-Bash Ogre — `{2}{B}{R}` 3/3 Ogre Warrior with Menace.
//! "Whenever this creature attacks, you may sacrifice another creature. If you do,
//! this creature gets +2/+2 until end of turn."
//!
//! Menace is expressible. The attack trigger is GAP'd: it requires an OPTIONAL
//! sacrifice cost paid during resolution ("you may sacrifice another creature; if
//! you do, ..."). OptionalPayment only supports Mana / Life costs, not a chosen
//! sacrifice, so the conditional pump cannot be expressed faithfully.

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
    let name = reg.interner_mut().intern("Brawl-Bash Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_sac_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_sac_pump(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature. If you do, +2/+2" — optional
    // sacrifice cost during resolution is not expressible (OptionalPayment only
    // supports Mana / Life). Emitting nothing rather than an unconditional pump.
    Vec::new()
}
