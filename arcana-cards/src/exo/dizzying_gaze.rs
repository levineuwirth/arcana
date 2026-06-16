//! Dizzying Gaze — `{R}` enchantment — Aura.
//! "Enchant creature you control. {R}: Enchanted creature deals 1 damage
//!  to target creature with flying."
//!
//! The Aura's own "{R}: enchanted creature deals 1 damage to target
//! creature with flying" activated ability. The damage source is the
//! host creature (reached via the Aura's `attached_to`); the target is a
//! creature with flying.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dizzying Gaze");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: controller wording ("you control") approximated by caster's choice.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: |_, _, _| Vec::new(),
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: Enchanted creature deals 1 damage to target \
                       creature with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..Default::default()
                },
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
                is_instant_speed: false,
                face_gate: None,
                effect: host_ping_flyer,
            }),
    )
}

fn host_ping_flyer(
    state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(target)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Damage source is the enchanted creature (the Aura's host).
    let source = state
        .objects
        .get(ctx.source)
        .and_then(|o| o.attached_to)
        .unwrap_or(ctx.source);
    vec![Effect::DealDamage {
        source,
        target: DamageTarget::Object(*target),
        amount: 1,
    }]
}
