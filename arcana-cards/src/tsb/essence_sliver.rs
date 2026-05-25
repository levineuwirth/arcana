//! Essence Sliver — `{3}{W}` 3/3 white Sliver.
//! "Whenever a Sliver deals damage, its controller gains that much life."
//!
//! GAP: "its controller gains life" (the controller of the damaging Sliver, not necessarily
//! this card's controller) requires reading `trig.damaged_player()` / source controller,
//! which pairs with DamageDealt. The "any Sliver" source is best-effort via subtype_filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let sliver_filter = script::subtype_filter(reg, "Sliver");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: sliver_filter,
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_sliver_deals_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_sliver_deals_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    // GAP: "its controller" (source Sliver's controller) — using trig.controller as best-effort.
    vec![Effect::GainLife { player: trig.controller, amount }]
}
