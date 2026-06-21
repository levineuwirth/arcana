//! Zacama, Primal Calamity — `{6}{R}{G}{W}` 9/9 Legendary Elder Dinosaur.
//! Vigilance, reach, trample.
//! When Zacama enters, if you cast it, untap all lands you control.
//! {2}{R}: Zacama deals 3 damage to target creature.
//! {2}{G}: Destroy target artifact or enchantment.
//! {2}{W}: You gain 3 life.
//!
//! Keywords are base characteristics. The ETB untaps all lands you control via
//! ForEach over the matching land ids; the "if you cast it" intervening-if has
//! no condition helper, so it is GAPped (fires unconditionally). The three
//! activated abilities are wired directly.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zacama, Primal Calamity");
    let elder = reg.interner_mut().intern("Elder");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dinosaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if you cast it" — no cast-source condition;
            // fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_untap_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Zacama deals 3 damage to target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Destroy target artifact or enchantment.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                        )),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: You gain 3 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_three,
            }),
    )
}

fn etb_untap_lands(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let lands = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &lands, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}

fn deal_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}

fn destroy_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn gain_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: 3,
    }]
}
