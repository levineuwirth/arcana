//! Spectral Steel — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +2/+2. {1}{W}, Exile this card
//!  from your graveyard: Return another target Aura or Equipment card from
//!  your graveyard to your hand."
//!
//! The +2/+2 is an ETB-installed `attached_pt`. The graveyard-activated ability
//! (exile-this-from-graveyard cost, returning an Aura/Equipment from the
//! graveyard) is a self-graveyard ability, not an attached host grant, and the
//! exile-from-graveyard cost is not expressible — GAP.

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
    let name = reg.interner_mut().intern("Spectral Steel");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard-activated "exile this from graveyard: return an Aura or
    // Equipment from graveyard to hand" — exile-from-graveyard cost not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(trig.source, 2, 2, Duration::WhileSourceOnBattlefield),
    }]
}
