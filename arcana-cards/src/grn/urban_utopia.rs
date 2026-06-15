//! Urban Utopia — `{1}{G}` enchantment — Aura.
//! "Enchant land.
//!  When this Aura enters, draw a card.
//!  Enchanted land has '{T}: Add one mana of any color.'"
//!
//! On ETB the Aura's controller draws a card, and the host land gains a
//! `{T}`-mana ability for each color (the established "any color" idiom,
//! installed as `attached_activated` mana abilities running against the host).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urban Utopia");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
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

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            tap: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn etb_install(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let controller = state.object_or_lki(trig.source).map(|o| o.controller);
    let mut out = Vec::new();
    if let Some(player) = controller {
        out.push(Effect::DrawCards { player, count: 1 });
    }
    let installs = [
        ("{T}: Add {W}.", add_white as fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>),
        ("{T}: Add {U}.", add_blue),
        ("{T}: Add {B}.", add_black),
        ("{T}: Add {R}.", add_red),
        ("{T}: Add {G}.", add_green),
    ];
    for (text, eff) in installs {
        out.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                mana_ability(text, eff),
                Duration::WhileSourceOnBattlefield,
            ),
        });
    }
    out
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}
fn add_blue(_s: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)] }]
}
fn add_black(_s: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)] }]
}
fn add_red(_s: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)] }]
}
fn add_green(_s: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}
