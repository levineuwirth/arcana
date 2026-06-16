//! Abian, Luvion Usurper — `{5}{R}{G}` Legendary Planeswalker — Abian,
//! starting loyalty 20.
//!
//! As Abian, Luvion Usurper enters, you become Abian. (Your life total becomes
//! equal to their loyalty. You can activate the loyalty abilities by spending
//! or gaining life.)
//! +3: Discard your hand, then draw cards equal to the greatest power among
//!     creatures you control.
//! +1: Create a 3/2 red and green Spirit creature token.
//! −X: You deal X damage to any target.
//!
//! # Scope
//! - The enters-replacement ("you become Abian; your life total becomes equal
//!   to their loyalty; activate loyalty abilities by spending/gaining life") is
//!   a bespoke life-as-loyalty rule from the Planechase/un-set space, not a
//!   loyalty ability and not expressible by the demonstrated surface — GAP'd
//!   (noted here; standard CR 606 loyalty is modeled instead).
//! - The `+3` discards your hand (count = current hand size) then draws cards
//!   equal to the greatest power among your creatures (dynamic, computed at
//!   resolution via script::power_of).
//! - The `+1` creates a 3/2 red-and-green Spirit creature token.
//! - The `−X` is the canonical dynamic-X loyalty ability (now supported): X is
//!   the loyalty paid; deal X damage to any target.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abian, Luvion Usurper");
    let abian = reg.interner_mut().intern("Abian");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(abian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(20),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+3: Discard your hand, then draw cards equal to the \
                       greatest power among creatures you control."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_three_wheel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/2 red and green Spirit creature token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_spirit,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-X: You deal X damage to any target.".into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_damage,
            }),
    )
}

/// `+3: Discard your hand, then draw cards = greatest power among your creatures.`
fn plus_three_wheel(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let hand = script::hand_size(state, ctx.controller);
    // Greatest power among creatures you control, at resolution.
    let mut greatest: i32 = 0;
    for o in state.objects.objects_in_zone(arcana_core::zones::Zone::Battlefield) {
        if o.controller == ctx.controller && o.characteristics.types.is_creature() {
            let p = script::power_of(state, o.id);
            if p > greatest {
                greatest = p;
            }
        }
    }
    let draw = greatest.max(0) as u32;
    let mut effects = Vec::new();
    if hand > 0 {
        effects.push(Effect::Discard {
            player: ctx.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    if draw > 0 {
        effects.push(Effect::DrawCards { player: ctx.controller, count: draw });
    }
    effects
}

/// `+1: Create a 3/2 red and green Spirit creature token.`
fn plus_one_spirit(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spirit);
    let token = arcana_core::effects::TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−X: You deal X damage to any target` (X = loyalty paid).
fn minus_x_damage(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let amount = ctx.x_value.unwrap_or(0);
    let dt = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id))) => DamageTarget::Object(*id),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p))) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount,
    }]
}
