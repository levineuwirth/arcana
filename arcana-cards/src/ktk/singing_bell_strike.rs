//! Singing Bell Strike — `{1}{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, tap enchanted creature.
//!  Enchanted creature doesn't untap during its controller's untap step.
//!  Enchanted creature has '{6}: Untap this creature.'"
//!
//! The ETB taps the host, installs an `attached_dont_untap` marker, and
//! grants the host the activated ability "{6}: Untap this creature" via
//! `attached_activated` — its {6} cost runs against the host and its effect
//! untaps the host (`ctx.source`). Mirrors Evanescent Intellect for the
//! granted-ability install.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Singing Bell Strike");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_dont_untap(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{6}: Untap this creature.".into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: untap_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ];
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::Tap { target: host });
    }
    effects
}

fn untap_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
