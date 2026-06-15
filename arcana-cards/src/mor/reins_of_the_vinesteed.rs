//! Reins of the Vinesteed — `{3}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +2/+2. When enchanted creature
//!  dies, you may return this card from your graveyard to the battlefield
//!  attached to a creature that shares a creature type with that creature."
//!
//! Buff Aura: ETB-installed `attached_pt(+2/+2)` following `source.attached_to`.
//! The host-dies clause returns the Aura from the graveyard to the battlefield
//! attached to a type-sharing creature — a graveyard-to-battlefield reattach
//! with a shared-type targeting constraint, not expressible with the
//! demonstrated API; that clause is GAP'd.

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
    let name = reg.interner_mut().intern("Reins of the Vinesteed");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
    // GAP: "When enchanted creature dies, you may return this card from your
    // graveyard to the battlefield attached to a creature that shares a creature
    // type with that creature" — graveyard-to-battlefield reattach with a
    // shared-type targeting constraint; not expressible with the shown API.
}
