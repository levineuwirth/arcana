//! Green Dragon — `{4}{G}{G}` 4/4 Dragon with Flying.
//! "Poison Breath — When this creature enters, until end of turn,
//! whenever a creature an opponent controls is dealt damage, destroy
//! it."
//!
//! Flying is a base keyword. The ETB schedules a floating "until end
//! of turn" trigger watching damage dealt to a creature an opponent
//! controls.

use arcana_core::effects::{Effect, FloatingUntil, KeywordAbility};
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
    let name = reg.interner_mut().intern("Green Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: poison_breath,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn poison_breath(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ScheduleFloatingTrigger {
        source: trig.source,
        controller: trig.controller,
        condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::permanent(),
            target_filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            ),
            combat_only: false,
        },
        effect: destroy_damaged_creature,
        until: FloatingUntil::EndOfTurn,
    }]
}

fn destroy_damaged_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "destroy it" — no PendingTrigger accessor surfaces the
    // damaged OBJECT (only damaged_player()), so the specific creature
    // dealt damage cannot be identified at resolution to destroy it.
    Vec::new()
}
