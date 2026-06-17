//! Asmoranomardicadaistinaculdacar — Legendary 3/3 Human Wizard (B/R),
//! no printed mana cost. ETB tutors for The Underworld Cookbook by name;
//! "Sacrifice two Foods: Target creature deals 6 damage to itself."
//! The alternative-cost static ("you may pay {B/R} to cast this spell")
//! is not expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Asmoranomardicadaistinaculdacar");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let food = reg.interner_mut().intern("Food");
    let food_filter = ObjectFilter {
        subtypes: Some(vec![food]),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "As long as you've discarded a card this turn, you may pay {B/R} to
    //      cast this spell" — alternative cast-cost static, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_cookbook,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice two Foods: Target creature deals 6 damage to itself.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(food_filter),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: target_self_damage,
            }),
    )
}

fn etb_tutor_cookbook(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("The Underworld Cookbook");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}

fn target_self_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "deals 6 damage to itself" — the target creature is both source and target.
    vec![Effect::DealDamage {
        source: *id,
        target: DamageTarget::Object(*id),
        amount: 6,
    }]
}
