//! Ureni, the Song Unending — `{5}{G}{U}{R}` 10/10 Legendary Creature —
//! Spirit Dragon.
//!
//! Oracle:
//! * Flying. (protection from white and from black — Protection is NOT
//!   in the usable KeywordAbility surface; GAP that half of the line.)
//! * "When Ureni enters, it deals X damage divided as you choose among
//!   any number of target creatures and/or planeswalkers your opponents
//!   control, where X is the number of lands you control." →
//!   `DealDamageDivided`; `TargetCount::Any` over opponent creatures /
//!   planeswalkers, with the total computed from the land count at
//!   resolution. (Even-split division is the documented fidelity gap.)

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP keyword: "protection from white and from black" — Protection is
//             not in the usable KeywordAbility surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ureni, the Song Unending");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_divided_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(
                                TypeLine::CREATURE | TypeLine::PLANESWALKER,
                            ))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
    )
}

fn etb_divided_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let total = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if total == 0 {
        return Vec::new();
    }
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
        total,
    }]
}
