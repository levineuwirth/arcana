//! Volatile Stormdrake — `{1}{U}` 3/2 Drake with Flying and Hexproof
//! from activated and triggered abilities.
//! "When this creature enters, exchange control of this creature and
//! target creature an opponent controls. If you do, you get {E}{E}{E}{E},
//! then sacrifice that creature unless you pay an amount of {E} equal to
//! its mana value."
//!
//! Fidelity: "Hexproof from activated and triggered abilities" is modeled
//! as the base Hexproof keyword (the source-restricted refinement isn't
//! expressible).
//!
//! The ETB is a partial: control exchange has no demonstrated primitive
//! (`ChangeControl` is one-directional), so the trigger gains control of
//! the target opponent's creature and grants you the four energy. GAP:
//! handing this creature to that opponent (the other half of the
//! exchange) and "sacrifice that creature unless you pay {E} equal to
//! its mana value" (energy isn't an OptionalPayment cost kind).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Volatile Stormdrake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: steal_and_energy,
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

fn steal_and_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Sequence(vec![
        Effect::ChangeControl {
            target: *id,
            new_controller: trig.controller,
        },
        Effect::GainEnergy {
            player: trig.controller,
            amount: 4,
        },
    ])]
}
