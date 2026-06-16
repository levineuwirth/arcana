//! Kirtar's Desire — `{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't attack. Threshold — Enchanted
//!  creature can't block as long as there are seven or more cards in your
//!  graveyard."
//!
//! The unconditional "can't attack" is an ETB-installed `attached_cant_attack`.
//! The Threshold "can't block as long as 7+ cards in graveyard" is a
//! conditional ("as long as") restriction with no expressible graveyard-gated
//! continuous-effect primitive, so it is GAP'd.

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
    let name = reg.interner_mut().intern("Kirtar's Desire");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
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
    // GAP: Threshold "can't block as long as 7+ cards in graveyard" —
    // conditional graveyard-gated restriction not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_cant_attack(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
