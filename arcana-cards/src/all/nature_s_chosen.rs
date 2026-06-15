//! Nature's Chosen — `{G}` enchantment — Aura.
//! "Enchant creature you control. {0}: Untap enchanted creature. Activate
//!  only during your turn and only once each turn. Tap enchanted creature:
//!  Untap target artifact, creature, or land. Activate only if enchanted
//!  creature is white and untapped and only once each turn."
//!
//! Both abilities are host-activated grants installed on ETB. The first
//! ({0}: untap the host) and the second (tap host: untap a target
//! permanent) are wired as `attached_activated` continuous effects whose
//! costs/effects run against the host. The "only during your turn / only
//! once each turn / only if white and untapped" activation restrictions
//! have no expressible primitive here — GAP (the abilities are otherwise
//! freely activatable).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nature's Chosen");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "creature you control" approximated by caster's Creature choice.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "only during your turn / only once each turn / only if white and
    // untapped" activation restrictions have no expressible primitive.
    let untap_self = ActivatedAbilityDef {
        text: "{0}: Untap enchanted creature.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{0}").expect("valid cost"),
            tap: false,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: untap_host,
    };
    let untap_target = ActivatedAbilityDef {
        text: "Tap enchanted creature: Untap target artifact, creature, or land.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{0}").expect("valid cost"),
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(ObjectFilter::permanent()),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: untap_chosen,
    };
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                untap_self,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                untap_target,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn untap_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}

fn untap_chosen(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => vec![Effect::Untap { target: *id }],
        _ => Vec::new(),
    }
}
