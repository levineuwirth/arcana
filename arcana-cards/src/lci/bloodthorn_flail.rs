//! Bloodthorn Flail — `{B}` artifact — Equipment (Duskmourn).
//! "Equipped creature gets +2/+1. Equip—Pay {3} or discard a card."
//!
//! The +2/+1 static installs normally. The equip cost is a CHOICE
//! ("Pay {3} or discard a card") — only the mana option is modeled via
//! `with_equip({3})`; the discard alternative is a GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Bloodthorn Flail");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "Equip—Pay {3} or discard a card" — the discard-a-card
    // alternative equip cost is not expressible; only the {3} mana option
    // is wired.
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_attached_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
