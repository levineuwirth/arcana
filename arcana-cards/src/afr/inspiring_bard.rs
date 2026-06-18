//! Inspiring Bard — `{3}{G}` 3/3 Elf Bard.
//! When this creature enters, choose one —
//!   • Bardic Inspiration — Target creature gets +2/+2 until end of turn.
//!   • Song of Rest — You gain 3 life.
//!
//! Modal choice is not expressible for a triggered ability (modal dispatch
//! exists only for spell abilities). We fire the ETB trigger and emit the
//! non-targeted "Song of Rest" mode faithfully; the choose-one is GAP'd.

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
    let name = reg.interner_mut().intern("Inspiring Bard");
    let elf = reg.interner_mut().intern("Elf");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_choose_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_choose_one(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" on a triggered ability is not expressible (the
    // "Bardic Inspiration" +2/+2-target mode is dropped). Emit "Song of Rest".
    vec![Effect::GainLife { player: trig.controller, amount: 3 }]
}
