//! Trickster's Talisman — `{U}` artifact — Equipment (blue).
//! "Invoke Duplicity — Equipped creature gets +1/+1 and has 'Whenever
//! this creature deals combat damage to a player, you may sacrifice
//! Trickster's Talisman. If you do, create a token that's a copy of
//! this creature.' Equip {2}"
//!
//! The Equip activation is wired via `with_equip`; the +1/+1 static
//! installs `ContinuousEffect::attached_pt`. The granted triggered
//! ability is a documented gap (no way to grant an ability to the
//! dynamically-attached creature).

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
    let name = reg.interner_mut().intern("Trickster's Talisman");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
    // GAP: "equipped creature ... has 'Whenever this creature deals combat
    // damage to a player, you may sacrifice Trickster's Talisman. If you
    // do, create a token that's a copy of this creature.'" — there is no
    // attached-ability grant (GrantTriggeredAbility takes a fixed id, not
    // the dynamic attached creature); only the +1/+1 half is installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
