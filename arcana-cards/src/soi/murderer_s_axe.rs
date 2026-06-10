//! Murderer's Axe — `{4}` artifact — Equipment (Shadows over
//! Innistrad). "Equipped creature gets +2/+2. Equip—Discard a card."
//!
//! The +2/+2 static is installed as an attached-P/T continuous effect
//! via the ETB trigger.
//!
//! GAP: the equip cost is non-mana ("Equip—Discard a card") — non-mana
//! equip costs are not expressible, so `with_equip` is NOT called and
//! the Equip ability is omitted entirely.

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
    let name = reg.interner_mut().intern("Murderer's Axe");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "Equip—Discard a card" — non-mana equip cost not
    // expressible; the Equip activation is omitted (no with_equip).
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_attached_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// ETB trigger: install the layer-7c "attached creature gets +2/+2"
/// continuous effect anchored to this Equipment.
fn etb_install_attached_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
