//! Ore Gorger — `{3}{R}{R}` 3/1 red creature. "Whenever you cast a Spirit or
//! Arcane spell, you may destroy target nonbasic land."
//!
//! NOTE: Arcane is a tribal subtype used as a spell type in Kamigawa; treated
//! as a subtype filter for the SpellCast condition. The "nonbasic" land target
//! constraint can't be expressed in TargetFilter — emitting basic target_creature
//! for land and noting the gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ore Gorger");
    let spirit = reg.interner_mut().intern("Spirit");
    let _arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — SpellCast filter for "Spirit or Arcane subtype"; using creature subtype Spirit only
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: destroy_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn destroy_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: target filter — "nonbasic land" not expressible; destroying any targeted land
    vec![Effect::DestroyPermanent { target: *id }]
}
