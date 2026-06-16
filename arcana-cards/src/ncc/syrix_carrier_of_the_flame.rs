//! Syrix, Carrier of the Flame — `{2}{B}{R}` 3/3 Legendary Phoenix with
//! Flying and Haste.
//! "At the beginning of each end step, if a creature card left your
//!  graveyard this turn, target Phoenix you control deals damage equal to
//!  its power to any target."
//! "Whenever another Phoenix you control dies, you may cast this card from
//!  your graveyard."
//!
//! Flying + Haste expressible. The end-step damage trigger is wired with
//! two targets (a Phoenix you control + any target); its intervening-if
//! ("a creature card left your graveyard this turn") has no matching
//! conditions:: helper, so it is a GAP (fires unconditionally). The
//! Phoenix-dies trigger has no cast-from-graveyard Effect — GAP'd effect,
//! structure kept.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syrix, Carrier of the Flame");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let phoenix_filter = script::subtype_filter(reg, "Phoenix")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if a creature card left your graveyard
                // this turn" — no matching conditions:: helper.
                intervening_if: None,
                effect: phoenix_deals_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(phoenix_filter),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::any_target(),
                ],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Phoenix")
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: cast_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn phoenix_deals_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut iter = trig.targets.targets.iter();
    let Some(TargetChoice::Object(phoenix_id)) = iter.next() else { return Vec::new(); };
    let Some(dest) = iter.next() else { return Vec::new(); };
    let dt = match dest {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let amount = script::power_of(state, *phoenix_id).max(0) as u32;
    vec![Effect::DealDamage {
        source: *phoenix_id,
        target: dt,
        amount,
    }]
}

fn cast_from_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast this card from your graveyard" — no
    // cast-from-graveyard permission Effect in the catalog.
    Vec::new()
}
