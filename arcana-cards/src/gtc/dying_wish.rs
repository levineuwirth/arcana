//! Dying Wish — `{1}{B}` enchantment — Aura.
//! "Enchant creature you control. When enchanted creature dies, target
//!  player loses X life and you gain X life, where X is its power."
//!
//! Host death trigger (`AttachedCreatureDoes { SelfDies }`) with a
//! target-player requirement: the target player loses X life and the
//! Aura's controller gains X life, where X is the dying creature's power
//! (`script::power_of`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dying Wish");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: controller wording ("you control") approximated by caster's choice.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: |_, _, _| Vec::new(),
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: on_host_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn on_host_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    let Some(TargetChoice::Player(target)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::power_of(state, dead).max(0) as u32;
    if x == 0 {
        return Vec::new();
    }
    let you = state.objects.get(trig.source).map(|o| o.controller);
    let mut effects = vec![Effect::LoseLife {
        player: *target,
        amount: x,
    }];
    if let Some(you) = you {
        effects.push(Effect::GainLife {
            player: you,
            amount: x,
        });
    }
    effects
}
