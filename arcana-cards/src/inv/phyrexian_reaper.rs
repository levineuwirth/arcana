//! Phyrexian Reaper — `{4}{B}` 3/3 black Creature — Phyrexian Zombie.
//! "Whenever this creature becomes blocked by a green creature, destroy that creature.
//! It can't be regenerated."
//! Trigger wired with `SelfBecomesBlockedBy` + a green-creature filter; the effect
//! destroys each green blocker (via `script::blockers_of`, re-filtered for green).
//! GAP: "can't be regenerated" modifier not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Reaper");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "becomes blocked by a green creature" — filtered form; fires
                // only when at least one declared blocker is green.
                trigger_condition: TriggerCondition::SelfBecomesBlockedBy {
                    filter: ObjectFilter::creature().with_colors(ColorSet::green()),
                },
                intervening_if: None,
                effect: blocked_destroy_blocker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blocked_destroy_blocker(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that creature" = the green blocker; destroy each green blocker
    // (covers multi-block where only some blockers are green).
    // GAP: "can't be regenerated" modifier not modeled.
    let green = ObjectFilter::creature().with_colors(ColorSet::green());
    script::blockers_of(state, trig.source)
        .into_iter()
        .filter(|&id| {
            state
                .objects
                .get(id)
                .is_some_and(|o| green.matches(o, state, trig.controller))
        })
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
