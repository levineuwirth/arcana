//! Brinelin, the Moon Kraken — `{6}{U}{U}` 6/8 Legendary Kraken.
//! "When Brinelin enters and whenever you cast a spell with mana value 6 or
//!  greater, you may return target nonland permanent to its owner's hand.
//!  Partner."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brinelin, the Moon Kraken");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    // Partner is a commander-format keyword not on the usable surface — GAP'd.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    let bounce_req = || TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: bounce_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![bounce_req()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_min_cmc(6)),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: bounce_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![bounce_req()],
            }),
    )
}

fn bounce_target(
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
    vec![Effect::ReturnToHand { target: *id }]
}
