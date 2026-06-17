//! Skoa, Embermage — `{4}{R}{R}` 4/4 Legendary Goblin Wizard.
//! "When Skoa enters, it deals 4 damage to any target."
//! "Grandeur — Discard another card named Skoa, Embermage, Sacrifice two
//!  Mountains: Skoa deals 4 damage to any target."
//!
//! Grandeur is not in the usable keyword surface; the Grandeur ability is
//! modeled as a plain activated ability whose cost is "discard a card
//! named Skoa, Embermage" + "sacrifice two Mountains".

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skoa, Embermage");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);

    let skoa_name = reg.interner_mut().intern("Skoa, Embermage");
    let mountain = reg.interner_mut().intern("Mountain");
    let discard_named = ObjectFilter { name: Some(skoa_name), ..ObjectFilter::default() };
    let sac_mountains = ObjectFilter::permanent().with_subtype_sym(mountain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: deal_4,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Grandeur — Discard another card named Skoa, Embermage, Sacrifice two Mountains: Skoa deals 4 damage to any target.".into(),
                cost: ActivationCost {
                    discard_other: Some(discard_named),
                    sacrifice_other: Some(sac_mountains),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_4_activated,
            }),
    )
}

fn target_to_dt(target: &TargetChoice) -> Option<DamageTarget> {
    match target {
        TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
        TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => Some(DamageTarget::Object(*id)),
            ObjectOrPlayer::Player(p) => Some(DamageTarget::Player(*p)),
        },
    }
}

fn deal_4(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let Some(dt) = target_to_dt(target) else { return Vec::new(); };
    vec![Effect::DealDamage { source: trig.source, target: dt, amount: 4 }]
}

fn deal_4_activated(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let Some(dt) = target_to_dt(target) else { return Vec::new(); };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 4 }]
}
