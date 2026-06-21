//! Bronze Walrus — `{3}` 2/2 Artifact Creature — Walrus.
//! When this creature enters, scry 2.
//! {T}: Add one mana of any color. (Modeled as five mana abilities, one
//! per WUBRG color — the activator picks the color by which ability is
//! activated.)

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bronze Walrus");
    let walrus = reg.interner_mut().intern("Walrus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(walrus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green)),
    )
}

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
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
    }
}

fn etb_scry2(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 2 }]
}

fn add_color(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(ManaColor::White, ctx)
}
fn add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(ManaColor::Blue, ctx)
}
fn add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(ManaColor::Black, ctx)
}
fn add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(ManaColor::Red, ctx)
}
fn add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_color(ManaColor::Green, ctx)
}
