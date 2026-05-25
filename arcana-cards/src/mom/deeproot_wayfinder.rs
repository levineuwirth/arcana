//! Deeproot Wayfinder — `{1}{G}` 2/3 Merfolk Scout.
//! Keywords: Surveil
//! "Whenever this creature deals combat damage to a player or battle,
//! surveil 1, then you may return a land card from your graveyard to
//! the battlefield tapped."
//!
//! GAP: "or battle" — DamageDealt TargetFilter::Player does not cover
//! Battle permanents.
//! GAP: "return a land card from your graveyard to the battlefield
//! tapped" — Reanimate with land filter. ObjectFilter for lands works.
//! Reanimate used with land filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deeproot Wayfinder");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "or battle" — TargetFilter::Player does not cover battles.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Surveil { player: trig.controller, count: 1 },
        Effect::Reanimate {
            player: trig.controller,
            filter: ObjectFilter {
                types_any: Some(TypeLine::LAND.into()),
                ..Default::default()
            },
            from_zone: Zone::Graveyard(trig.controller),
        },
    ]
}
