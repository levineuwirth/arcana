//! Bramblewood Paragon — `{1}{G}` 2/2 Elf Warrior.
//!
//! Oracle:
//! * "Each other Warrior creature you control enters with an additional
//!   +1/+1 counter on it." — an enters-with replacement modifier
//!   affecting OTHER creatures; no demonstrated primitive expresses a
//!   board-wide enters-with rider. GAP'd.
//! * "Each creature you control with a +1/+1 counter on it has trample."
//!   — a filtered keyword anthem, wired via the `SelfEntersBattlefield`
//!   install idiom: `ContinuousEffect::filtered_keyword` over creatures
//!   you control that have a +1/+1 counter, lasting while the Paragon is
//!   on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bramblewood Paragon");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Each other Warrior creature you control enters with an
    // additional +1/+1 counter on it" — an enters-with replacement
    // modifier affecting other creatures, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_counter_trample,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Each creature you control with a +1/+1 counter on it has trample." —
/// install a filtered keyword anthem (trample) over creatures you control
/// that have a +1/+1 counter.
fn install_counter_trample(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter {
        has_counter: Some(CounterKind::PlusOnePlusOne),
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            filter,
            KeywordAbility::Trample,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
