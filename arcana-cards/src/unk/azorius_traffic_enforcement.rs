//! Azorius Traffic Enforcement — `{2}{W}` 2/3 Creature — Human Soldier.
//!
//! * Creatures opponents control entering the battlefield don't cause
//!   backup abilities to trigger. (Static replacement — not expressible;
//!   GAP'd.)
//! * At the beginning of your combat step, detain target creature with
//!   backup or vehicle an opponent controls.
//!
//! GAP: Detain is not a supported keyword and there is no Detain effect
//! primitive; the combat trigger's effect is GAP'd (the "with backup or
//! vehicle" target filter is also not expressible).
//! GAP: the "don't cause backup abilities to trigger" static replacement is
//! not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Azorius Traffic Enforcement");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: detain_gap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn detain_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Detain effect primitive; "with backup or vehicle" filter also
    // not expressible.
    Vec::new()
}
