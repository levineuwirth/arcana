//! Artificer's Hex — `{B}` enchantment — Aura.
//! "Enchant Equipment. At the beginning of your upkeep, if enchanted
//! Equipment is attached to a creature, destroy that creature."
//!
//! Enchant-Equipment Aura; the enchant target is approximated by an
//! artifact filter (Equipment is an artifact subtype). The upkeep payoff
//! is a `StepBegins(Upkeep, You)` trigger that double-hops `attached_to`:
//! the Aura's host is the Equipment (`source.attached_to`), and the
//! creature to destroy is the Equipment's OWN `attached_to`. The "if
//! attached to a creature" intervening-if is honored at resolution — the
//! effect returns nothing when the Equipment isn't equipping a creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artificer's Hex");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: Enchant Equipment approximated by an artifact filter.
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_destroy_equipped_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_destroy_equipped_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // Host is the enchanted Equipment; the creature it's destroying is the
    // Equipment's own attachment. "if enchanted Equipment is attached to a
    // creature" is satisfied exactly when this second hop resolves.
    let Some(equipment) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(creature) = state.object_or_lki(equipment).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: creature }]
}
