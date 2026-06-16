//! Daybreak Ranger // Nightfall Predator
//!
//! Front face: `{2}{G}` Human Archer Ranger Werewolf 2/2.
//! {T}: This creature deals 2 damage to target creature with flying.
//! At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face: Werewolf 4/4.
//! {R}, {T}: This creature fights target creature.
//! At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! GAP: "if no spells were cast last turn" condition not expressible; front upkeep transform
//!      fires unconditionally.
//! GAP: back-face activated ability ({R},{T}: fight target creature) not modeled
//!      (back-face-only activated ability engine debt).
//! GAP: back-face upkeep transform ("if a player cast two or more spells last turn") not modeled
//!      (back-face-only triggered ability engine debt).
//! GAP: day/night cycle not modeled.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daybreak Ranger");
    let human_sub = reg.interner_mut().intern("Human");
    let archer_sub = reg.interner_mut().intern("Archer");
    let ranger_sub = reg.interner_mut().intern("Ranger");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(archer_sub);
    subtypes.0.insert(ranger_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Nightfall Predator");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: deals 2 damage to target creature with flying
            // (restriction enforced via keyword filter).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 2 damage to target creature with flying.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: tap_deal_damage_flyer,
            })
            // Front face: at beginning of each upkeep, if no spells were cast last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face {R},{T}: fight target creature — back-face-only activated ability.
            // GAP: back-face upkeep transform (≥2 spells last turn) — back-face-only triggered ability.
    )
}

fn tap_deal_damage_flyer(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: 2,
        source: ctx.source,
    }]
}

fn iif_no_spells(state: &GameState, _s: ObjectId, _y: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
