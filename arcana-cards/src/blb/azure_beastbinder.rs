//! Azure Beastbinder — `{1}{U}` 1/3 blue Rat Rogue with Vigilance.
//! "This creature can't be blocked by creatures with power 2 or greater.
//!  Whenever this creature attacks, up to one target artifact, creature, or
//!  planeswalker an opponent controls loses all abilities until your next
//!  turn. If it's a creature, it also has base power and toughness 2/2 until
//!  your next turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Azure Beastbinder");
    let rat = reg.interner_mut().intern("Rat");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "can't be blocked by creatures with power 2 or greater" —
    // no power-conditioned evasion static is expressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_strip_abilities,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        ))
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn attack_strip_abilities(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Loses all abilities + becomes base 2/2 (only relevant if a creature);
    // SetBasePT on a noncreature is harmless. Use "until your next turn".
    vec![
        Effect::LoseAllAbilities {
            target: *id,
            duration: Duration::UntilYourNextTurn(trig.controller),
        },
        Effect::SetBasePT {
            target: *id,
            power: 2,
            toughness: 2,
            duration: Duration::UntilYourNextTurn(trig.controller),
        },
    ]
}
