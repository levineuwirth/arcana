//! Flesh Reaver — `{1}{B}` 4/4 Phyrexian Horror. "Whenever this
//! creature deals damage to a creature or opponent, this creature
//! deals that much damage to you." Self-damage rider triggered by
//! the engine's `DamageDealt` event; the amount is read from the
//! triggering event via `trig.damage_amount()`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flesh Reaver");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "creature or opponent" is narrower
                // than `TargetFilter::AnyTarget` (excludes
                // planeswalkers and "you"); the catalog has no
                // creature-or-player union, so we use AnyTarget as
                // the closest match. The engine binds the trigger's
                // self-as-source via the ability's home object.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: deal_damage_to_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Resolution: Flesh Reaver deals damage equal to the triggering
/// damage amount to its own controller ("you").
fn deal_damage_to_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: n,
        source: trig.source,
    }]
}
