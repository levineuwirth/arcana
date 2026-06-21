//! Vineshaper Prodigy — `{1}{G}` 2/2 Elf Druid.
//! Kicker {1}{U}. (GAP — Kicker not in usable keyword surface)
//! When this creature enters, if it was kicked, look at the top three cards of
//! your library. Put one into your hand and the rest on the bottom in any order.

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
    let name = reg.interner_mut().intern("Vineshaper Prodigy");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    // GAP: Kicker {1}{U} — Kicker is not part of the usable keyword surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_if_kicked_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_if_kicked_dig(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the effect is gated on "if it was kicked", but there is no
    // kicked-status condition/accessor. Firing the dig unconditionally would be
    // materially wrong, so the whole effect is omitted.
    Vec::new()
}
