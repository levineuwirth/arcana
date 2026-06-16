//! Thornmantle Striker — `{4}{B}` 4/3 Elf Rogue.
//!
//! Oracle (ETB, "choose one"):
//! * Remove X counters from target permanent, where X is the number of
//!   Elves you control.
//! * Target creature an opponent controls gets -X/-X until end of turn,
//!   where X is the number of Elves you control.
//!
//! Modal dispatch is a SPELL-ability mechanism in this engine
//! (`with_mode_effects` / `dispatch_modal_effect`); a TRIGGERED ETB
//! ability has no modal field, and the two modes carry different target
//! filters, so a single triggered ability cannot express the "choose
//! one" branch. We implement the second mode (target creature an
//! opponent controls gets -X/-X, X = Elves you control), which is a
//! clean targeted ETB trigger, and GAP the first mode + the modal
//! choice.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornmantle Striker");
    let elf = reg.interner_mut().intern("Elf");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: modal "choose one" + mode 1 (remove X counters from target
    // permanent) — no modal field on a triggered ability and the modes
    // carry distinct target filters.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: minus_x_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn minus_x_x(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
