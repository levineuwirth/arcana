//! Serene Master — `{1}{W}` 0/2 white Human Monk.
//! "Whenever this creature blocks, exchange its power and the power of
//! target creature it's blocking until end of combat."
//! Implemented with SetBasePT swapping both creatures' base power.
//! GAP: Duration::EndOfTurn used because EndOfCombat is not in the Duration enum.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serene Master");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBlocks,
            intervening_if: None,
            effect: exchange_powers,
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

fn exchange_powers(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let my_power = script::power_of(state, trig.source);
    let my_toughness = script::toughness_of(state, trig.source);
    let target_power = script::power_of(state, *id);
    let target_toughness = script::toughness_of(state, *id);
    // GAP: duration should be EndOfCombat, not EndOfTurn; no EndOfCombat Duration variant.
    vec![
        Effect::SetBasePT {
            target: *id,
            power: my_power,
            toughness: target_toughness,
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: trig.source,
            power: target_power,
            toughness: my_toughness,
            duration: Duration::EndOfTurn,
        },
    ]
}
