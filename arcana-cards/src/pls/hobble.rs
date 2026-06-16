//! Hobble — `{2}{W}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw a card. Enchanted creature
//!  can't attack. Enchanted creature can't block if it's black."
//!
//! The ETB "draw a card" rides the fixed trigger; the unconditional "can't
//! attack" installs `attached_cant_attack`. The "can't block if it's black"
//! is a color-conditional restriction the unconditional attached_cant_block
//! can't express — GAP that clause.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Hobble");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't block if it's black" — color-conditional restriction not
    // expressible with the unconditional attached_cant_block.
    let mut effects = Vec::new();
    if let Some(you) = _state.objects.get(trig.source).map(|o| o.controller) {
        effects.push(Effect::DrawCards {
            player: you,
            count: 1,
        });
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_cant_attack(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
