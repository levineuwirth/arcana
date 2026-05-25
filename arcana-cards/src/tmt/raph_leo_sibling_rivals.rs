//! Raph & Leo, Sibling Rivals — `{1}{R/W}{R/W}` 2/4 red-white legendary
//! creature (Mutant Ninja Turtle). "Whenever Raph & Leo attack, if it's
//! the first combat phase of the turn, untap one or two target attacking
//! creatures. After this phase, there is an additional combat phase."
//!
//! GAP: "if it's the first combat phase of the turn" — intervening-if
//! tracking which combat phase this is not expressible.
//! GAP: "After this phase, there is an additional combat phase" — no
//! Effect for adding an extra combat phase.
//! Emitting Untap on up to two target creatures as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raph & Leo, Sibling Rivals");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: "if it's the first combat phase of the turn" not
                // expressible as intervening-if.
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "additional combat phase" — no Effect variant for extra combat.
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::Untap { target: *id })
        } else {
            None
        }
    }).collect()
}
