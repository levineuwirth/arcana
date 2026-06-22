//! Flameblade Angel — `{4}{R}{R}` 4/4 Angel with Flying.
//!
//! Oracle:
//! * Flying — keyword, base characteristic.
//! * "Whenever a source an opponent controls deals damage to you or a permanent
//!    you control, you may have this creature deal 1 damage to that source's
//!    controller." — wired as a damage-dealt trigger (a source an opponent
//!    controls deals damage to you). GAP: "that source's controller" cannot be
//!    read (no damage-source-controller accessor), and the "or a permanent you
//!    control" target side and the "you may" gate aren't expressible — the
//!    effect body returns `Vec::new()`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flameblade Angel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::Opponent),
                target_filter: TargetFilter::Player,
                combat_only: false,
            },
            intervening_if: None,
            effect: retaliate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn retaliate(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "deal 1 damage to that source's controller" — no accessor for the
    //      damage source's controller; the "you may" gate is also not modeled.
    Vec::new()
}
