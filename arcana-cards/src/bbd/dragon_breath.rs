//! Dragon Breath — `{1}{R}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has haste. {R}: Enchanted creature
//!  gets +1/+0 until end of turn. When a creature with mana value 6 or
//!  greater enters, you may return this card from your graveyard to the
//!  battlefield attached to that creature."
//!
//! Haste is an ETB-installed `attached_keyword`; the {R} pump is an
//! ETB-installed `attached_activated` ability that pumps the host. The
//! graveyard self-reanimation trigger (fires from the graveyard, on
//! another creature ETB) is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Dragon Breath");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "When a creature with mana value 6+ enters, you may return this
    // card from your graveyard to the battlefield attached to that creature"
    // — graveyard-zone self-reattach trigger, not expressible here.
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
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{R}: Enchanted creature gets +1/+0 until end of turn."
                        .into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: pump_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn pump_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
