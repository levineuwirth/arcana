//! Toxin Sliver — `{3}{B}` 3/3 black Creature — Sliver.
//! "Whenever a Sliver deals combat damage to a creature, destroy that creature.
//! It can't be regenerated."
//! GAP: trigger — "a Sliver deals combat damage to a creature" has no exact
//! TriggerCondition; DamageDealt targets a player, not a creature.
//! Using DamageDealt with creature target_filter as closest approximation.
//! GAP: effect — "can't be regenerated" modifier on DestroyPermanent is not
//! in the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toxin Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — DamageDealt with TargetFilter::Creature is not
                // in the catalog (only TargetFilter::Player); using Creature
                // as best guess but this may not compile.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Creature,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_sliver_damages_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_sliver_damages_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "destroy that creature" — the damaged creature id from dying_object.
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::DestroyPermanent { target: id }]
}
