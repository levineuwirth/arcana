//! Gift of Paradise — `{2}{G}` enchantment — Aura.
//! "Enchant land. When this Aura enters, you gain 3 life. Enchanted land has
//!  '{T}: Add two mana of any one color.'"
//!
//! The ETB "gain 3 life" is expressed faithfully. The granted "{T}: Add two
//! mana of any one color" mana ability is installed on the host as five
//! attached_activated mana abilities, one per WUBRG color, each adding two mana
//! of that color; the shared {T} (on the host) means only one fires
//! (command_tower idiom). The host's color choice = which ability is activated.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gift of Paradise");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
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
        Effect::GainLife {
            player: trig.controller,
            amount: 3,
        },
        grant(trig.source, "{T}: Add {W}{W}.", add_white),
        grant(trig.source, "{T}: Add {U}{U}.", add_blue),
        grant(trig.source, "{T}: Add {B}{B}.", add_black),
        grant(trig.source, "{T}: Add {R}{R}.", add_red),
        grant(trig.source, "{T}: Add {G}{G}.", add_green),
    ]
}

/// Install one granted host mana ability — `source` is this Aura; the ability
/// runs against the enchanted land (its host).
fn grant(
    source: arcana_core::objects::ObjectId,
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> Effect {
    Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            source,
            ActivatedAbilityDef {
                text: text.into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect,
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }
}

/// Granted host mana abilities — `ctx.source` is the enchanted land. Each adds
/// two mana of one color ("two mana of any one color").
fn add_two(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(color, ctx.source),
            ManaUnit::plain(color, ctx.source),
        ],
    }]
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_two(ctx, ManaColor::White)
}

fn add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_two(ctx, ManaColor::Blue)
}

fn add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_two(ctx, ManaColor::Black)
}

fn add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_two(ctx, ManaColor::Red)
}

fn add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_two(ctx, ManaColor::Green)
}
