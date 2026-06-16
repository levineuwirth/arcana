//! Mark of Eviction — `{U}` enchantment — Aura.
//! "Enchant creature. At the beginning of your upkeep, return enchanted
//!  creature and all Auras attached to that creature to their owners' hands."
//!
//! An upkeep trigger (`StepBegins(Upkeep, You)`) returns the enchanted
//! creature (reached via `source.attached_to`) to its owner's hand. The
//! "and all Auras attached to that creature" rider has no expressible
//! attachment-enumeration primitive here — GAP'd (returning the host bounces
//! Mark of Eviction itself as a state-based consequence).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mark of Eviction");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_return_host,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_return_host(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and all Auras attached to that creature" — no attachment-enumeration
    //      primitive; only the enchanted creature is returned.
    let Some(host) = state
        .object_or_lki(trig.source)
        .and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: host }]
}
