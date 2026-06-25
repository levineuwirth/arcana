//! Chronicler of Worship — `{1}{G}` 1/1 green Human Monk.
//!
//! Oracle:
//! * When Chronicler of Worship enters, put a random Shrine card from among
//!   the top seven cards of your library into your hand. It perpetually gains
//!   "This spell costs {1} less to cast." Then shuffle. — GAP: random reveal
//!   from a fixed depth + a perpetual cost-reduction grant are not
//!   expressible (DigTopN is a may-pick, single-take, no perpetual rider).
//! * {T}: Add one mana of any color. — "any color" is modeled as five {T}
//!   mana abilities, one per WUBRG color; the shared tap cost means only one fires.

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
    let name = reg.interner_mut().intern("Chronicler of Worship");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_shrine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(cw_mana_ability("{T}: Add {W}.", cw_white))
            .with_activated_ability(cw_mana_ability("{T}: Add {U}.", cw_blue))
            .with_activated_ability(cw_mana_ability("{T}: Add {B}.", cw_black))
            .with_activated_ability(cw_mana_ability("{T}: Add {R}.", cw_red))
            .with_activated_ability(cw_mana_ability("{T}: Add {G}.", cw_green)),
    )
}

fn cw_mana_ability(
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

fn etb_shrine(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: random Shrine from top seven + perpetual "costs {1} less" grant.
    Vec::new()
}

fn cw_add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn cw_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    cw_add_one(ctx, ManaColor::White)
}
fn cw_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    cw_add_one(ctx, ManaColor::Blue)
}
fn cw_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    cw_add_one(ctx, ManaColor::Black)
}
fn cw_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    cw_add_one(ctx, ManaColor::Red)
}
fn cw_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    cw_add_one(ctx, ManaColor::Green)
}
