//! Wrathful Raptors — `{4}{R}` 5/5 red Dinosaur.
//!
//! Trample.
//! Whenever a Dinosaur you control is dealt damage, it deals that much
//! damage to any target that isn't a Dinosaur.
//!
//! The trigger fires on damage to any Dinosaur you control and deals
//! `damage_amount` to the chosen any target. Two partials: (1) "it
//! deals" — the redirected source should be the damaged Dinosaur, but
//! there is no damaged-object accessor, so the damage is sourced from
//! Wrathful Raptors itself; (2) the "that isn't a Dinosaur" target
//! restriction can't be expressed on `any_target`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrathful Raptors");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let dino_you_control = arcana_core::script::subtype_filter(reg, "Dinosaur")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::permanent(),
                target_filter: TargetFilter::Permanent(dino_you_control),
                combat_only: false,
            },
            intervening_if: None,
            effect: reflect_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "any target that isn't a Dinosaur" — the Dinosaur
            // exclusion can't be expressed on any_target.
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn reflect_damage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    if amount == 0 {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    // Partial: "it deals" should source from the damaged Dinosaur; no
    // damaged-object accessor exists, so source from this creature.
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount,
    }]
}
