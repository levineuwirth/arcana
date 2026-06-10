//! Swashbuckler's Whip — `{1}` artifact — Equipment (The Lost Caverns
//! of Ixalan, 2023). "Equipped creature has reach, \"{2}, {T}: Tap
//! target artifact or creature,\" and \"{8}, {T}: Discover 10.\"
//! Equip {1}"
//!
//! The Equip activation is wired via `with_equip`; every granted line
//! is GAP'd — the Equipment static surface covers attached P/T only,
//! with no attached keyword grant and no attached activated-ability
//! grant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swashbuckler's Whip");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_grants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_grants(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Equipped creature has reach" — attached_pt covers P/T only
    // (no attached keyword grant yet).
    // GAP: "Equipped creature has \"{2}, {T}: Tap target artifact or
    // creature\"" and "\"{8}, {T}: Discover 10\"" — granting activated
    // abilities to the dynamically-attached creature is not expressible
    // in the Equipment static surface.
    Vec::new()
}
