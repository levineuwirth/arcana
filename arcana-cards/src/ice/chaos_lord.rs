//! Chaos Lord — `{4}{R}{R}{R}` 7/7 Human with First strike.
//! "At the beginning of your upkeep, target opponent gains control of
//! this creature if the number of permanents is even."
//! "This creature can attack as though it had haste unless it entered
//! this turn."
//!
//! First strike is a base keyword. The upkeep trigger's "if the number
//! of permanents is even" parity gate has no condition helper, so its
//! body is GAP'd (firing unconditionally would always give the creature
//! away — materially wrong). The haste-like static is a continuous
//! ability with no expressible primitive, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Lord");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "This creature can attack as though it had haste unless it entered
    // this turn." — a continuous self-modifying attack-permission static with no
    // expressible primitive in the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: maybe_give_control,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn maybe_give_control(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "... if the number of permanents is even" — there is no board-parity
    // condition helper, so the conditional control swap is inexpressible.
    // Firing unconditionally would always hand the creature to the opponent
    // (materially wrong), so the body is GAP'd.
    Vec::new()
}
