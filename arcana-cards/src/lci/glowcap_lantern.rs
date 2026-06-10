//! Glowcap Lantern — `{G}` green artifact — Equipment (The Lost Caverns
//! of Ixalan, 2023). "Equipped creature has 'You may look at the top
//! card of your library any time' and 'Whenever this creature attacks,
//! it explores.' Equip {2}."
//! The Equip activation is wired via the builder; both granted statics
//! are GAPs — there is no attached-keyword/ability grant (attached_pt
//! covers P/T only), no look-at-top-card static, and no trigger
//! condition for "the creature this Equipment is attached to attacks".

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
    let name = reg.interner_mut().intern("Glowcap Lantern");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_statics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_statics(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature has 'You may look at the top card of your
    // library any time'" — no attached-ability grant and no
    // look-at-top-of-library static.
    // GAP: "Whenever this creature attacks, it explores" granted to the
    // equipped creature — no trigger condition tracks the dynamically
    // attached creature.
    Vec::new()
}
