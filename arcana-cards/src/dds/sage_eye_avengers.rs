//! Sage-Eye Avengers — `{4}{U}{U}` 4/5 Djinn Monk.
//!
//! Oracle:
//! * Prowess (Whenever you cast a noncreature spell, this creature gets
//!   +1/+1 until end of turn.)  — GAP: Prowess is not in the usable
//!   `KeywordAbility` surface, so it is not emitted.
//! * Whenever this creature attacks, you may return target creature to
//!   its owner's hand if its power is less than this creature's power.
//!
//! Implemented: the attack trigger. It targets a creature and, at
//! resolution, returns it to its owner's hand only when its power is
//! strictly less than this creature's power (the "if its power is less"
//! restriction is enforced in the resolver via `script::power_of`). The
//! "you may" is the engine's optional resolution.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage-Eye Avengers");
    let djinn = reg.interner_mut().intern("Djinn");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(monk);

    // GAP: keyword — Prowess is not in the usable KeywordAbility surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: maybe_bounce_weaker_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn maybe_bounce_weaker_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // Only return if the target's power is strictly less than this creature's.
    let target_power = script::power_of(state, *id);
    let self_power = script::power_of(state, trig.source);
    if target_power < self_power {
        vec![Effect::ReturnToHand { target: *id }]
    } else {
        Vec::new()
    }
}
