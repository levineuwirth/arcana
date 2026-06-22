//! Kemba's Outfitter — `{W}` 2/1 Cat Artificer.
//! When Kemba's Outfitter enters, choose one —
//! • Choose an Equipment card in your hand. It perpetually gains equip {1}.
//! • Target Equipment you control perpetually gains equip {1}.
//! GAP: both modes "perpetually gain equip {1}" — perpetual ability grants are
//! not expressible.

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
    let name = reg.interner_mut().intern("Kemba's Outfitter");
    let cat = reg.interner_mut().intern("Cat");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_perpetual_equip,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_perpetual_equip(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: both modes give an Equipment "perpetually gains equip {1}" — granting
    // a perpetual activated ability to a card (in hand or on battlefield) is not
    // expressible with the available Effect primitives.
    Vec::new()
}
