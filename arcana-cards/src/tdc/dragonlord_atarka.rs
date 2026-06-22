//! Dragonlord Atarka — `{5}{R}{G}` 8/8 Legendary Elder Dragon.
//! Flying, trample.
//! When Dragonlord Atarka enters, it deals 5 damage divided as you choose among
//! any number of target creatures and/or planeswalkers your opponents control.
//!
//! Flying + Trample are base characteristics. The ETB is the divided-damage
//! recipe: a `TargetRequirement` over opponent-controlled creatures/planeswalkers
//! with `TargetCount::Any`, resolved into `Effect::DealDamageDivided` with
//! `total: 5` (the engine spreads the total across the chosen targets).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonlord Atarka");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_divided_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
                        .controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
        }),
    )
}

fn etb_divided_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets: Vec<DamageTarget> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
            TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
            _ => None,
        })
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::DealDamageDivided {
        source: trig.source,
        targets,
        total: 5,
    }]
}
