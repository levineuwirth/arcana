//! The Aetherspark — `{4}` legendary artifact planeswalker — Equipment,
//! colorless, starting loyalty 0 (gains loyalty via its attached static).
//!
//! # Static ability (not a loyalty ability)
//!
//! "As long as The Aetherspark is attached to a creature, it can't be
//! attacked and has 'Whenever equipped creature deals combat damage
//! during your turn, put that many loyalty counters on The
//! Aetherspark.'" This is a continuous/static ability with a granted
//! triggered ability — not a loyalty ability and not expressible from
//! the demonstrated surface; it is documented here but not wired.
//!
//! # Loyalty abilities
//!
//! * `+1`: Attach The Aetherspark to up to one target creature you
//!   control. Put a +1/+1 counter on that creature. (Effect::Attach +
//!   Effect::AddCounters; "up to one" → `TargetCount::UpTo(1)`.)
//! * `−5`: Draw two cards. (Effect::DrawCards.)
//! * `−10`: Add ten mana of any one color. Modeled as five loyalty
//!   abilities, one per WUBRG color, each at the −10 cost and each adding
//!   ten mana of that one color; the player picks the color by choosing
//!   which ability to activate (the shared −10 cost means only one fires).
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 / 606.3 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Aetherspark");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::PLANESWALKER),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(0),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Attach The Aetherspark to up to one target \
                       creature you control. Put a +1/+1 counter on that \
                       creature.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_attach,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: Draw two cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_draw,
            })
            .with_activated_ability(minus_ten_mana_ability("−10: Add ten {W}.", minus_ten_white))
            .with_activated_ability(minus_ten_mana_ability("−10: Add ten {U}.", minus_ten_blue))
            .with_activated_ability(minus_ten_mana_ability("−10: Add ten {B}.", minus_ten_black))
            .with_activated_ability(minus_ten_mana_ability("−10: Add ten {R}.", minus_ten_red))
            .with_activated_ability(minus_ten_mana_ability("−10: Add ten {G}.", minus_ten_green)),
    )
}

/// `−10: Add ten mana of any one color` — one loyalty ability per color,
/// each at the −10 cost, each adding ten mana of that one color.
fn minus_ten_mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            remove_self_counter: Some((CounterKind::Loyalty, 10)),
            ..ActivationCost::default()
        },
        target_requirements: vec![],
        is_mana_ability: false,
        is_loyalty_ability: true,
        activation_zone: arcana_core::registry::ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

/// `+1: Attach The Aetherspark to up to one target creature you control.
/// Put a +1/+1 counter on that creature.`
fn plus_one_attach(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Up to one target" — picking zero is legal, in which case nothing
    // attaches and no counter is placed.
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::Attach {
            equipment_or_aura: ctx.source,
            target: *id,
        },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}

/// `−5: Draw two cards.`
fn minus_five_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−10: Add ten mana of any one color.` — ten mana of one chosen color.
fn minus_ten_of(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source); 10],
    }]
}

fn minus_ten_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    minus_ten_of(ctx, ManaColor::White)
}
fn minus_ten_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    minus_ten_of(ctx, ManaColor::Blue)
}
fn minus_ten_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    minus_ten_of(ctx, ManaColor::Black)
}
fn minus_ten_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    minus_ten_of(ctx, ManaColor::Red)
}
fn minus_ten_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    minus_ten_of(ctx, ManaColor::Green)
}
