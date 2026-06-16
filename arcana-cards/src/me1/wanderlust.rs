//! Wanderlust — `{2}{G}` enchantment — Aura.
//! "Enchant creature. At the beginning of the upkeep of enchanted
//!  creature's controller, this Aura deals 1 damage to that player."
//!
//! Partial: the trigger fires at the beginning of the upkeep of the
//! enchanted creature's CONTROLLER. There is no `ControllerConstraint`
//! that targets "the controller of the enchanted creature", so the timing
//! is approximated to the Aura controller's upkeep (`You`). The damage —
//! 1 to the enchanted creature's controller (read via
//! `source.attached_to`) — is faithful.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Wanderlust");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "upkeep of enchanted creature's controller" approximated to
        // the Aura controller's upkeep (no ControllerConstraint for host's controller).
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(controller),
        amount: 1,
    }]
}
