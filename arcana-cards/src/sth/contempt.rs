//! Contempt — `{1}{U}` enchantment — Aura.
//! "Enchant creature. When enchanted creature attacks, return it and this
//! Aura to their owners' hands at end of combat."
//!
//! Host-attacks trigger (`AttachedCreatureDoes { SelfAttacks }`) returns
//! both the host (`trig.attacking_creature()`) and this Aura
//! (`trig.source`) to their owners' hands. The "at end of combat" delay
//! has no delayed-bounce form here, so the return resolves when the
//! attacks trigger resolves rather than at end of combat.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Contempt");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
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
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfAttacks),
                },
                intervening_if: None,
                effect: bounce_both,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bounce_both(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at end of combat" delay — returns immediately on the attacks
    // trigger rather than being scheduled for end of combat.
    let mut effects = Vec::new();
    if let Some(host) = trig.attacking_creature() {
        effects.push(Effect::ReturnToHand { target: host });
    }
    effects.push(Effect::ReturnToHand { target: trig.source });
    effects
}
