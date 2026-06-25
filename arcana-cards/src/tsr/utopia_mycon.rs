//! Utopia Mycon — `{G}` 0/2 Fungus.
//! "At the beginning of your upkeep, put a spore counter on this creature."
//! "Remove three spore counters from this creature: Create a 1/1 green
//! Saproling creature token."
//! "Sacrifice a Saproling: Add one mana of any color." "any color" is modeled
//! as five Sacrifice-a-Saproling mana abilities, one per WUBRG color (the
//! shared sacrifice cost means only one fires).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaUnit;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Utopia Mycon");
    let fungus = reg.interner_mut().intern("Fungus");
    reg.interner_mut().intern("Saproling");
    reg.interner_mut().intern("spore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let spore = reg.interner().lookup("spore").expect("interned");
    let saproling_filter = {
        let f = script::subtype_filter(reg, "Saproling");
        f.controlled_by(ControllerConstraint::You)
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_spore,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(spore), 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_saproling,
            })
            .with_activated_ability(sac_mana_ability(
                "Sacrifice a Saproling: Add {W}.",
                saproling_filter.clone(),
                um_add_white,
            ))
            .with_activated_ability(sac_mana_ability(
                "Sacrifice a Saproling: Add {U}.",
                saproling_filter.clone(),
                um_add_blue,
            ))
            .with_activated_ability(sac_mana_ability(
                "Sacrifice a Saproling: Add {B}.",
                saproling_filter.clone(),
                um_add_black,
            ))
            .with_activated_ability(sac_mana_ability(
                "Sacrifice a Saproling: Add {R}.",
                saproling_filter.clone(),
                um_add_red,
            ))
            .with_activated_ability(sac_mana_ability(
                "Sacrifice a Saproling: Add {G}.",
                saproling_filter,
                um_add_green,
            )),
    )
}

fn sac_mana_ability(
    text: &str,
    saproling_filter: arcana_core::targets::ObjectFilter,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            sacrifice_other: Some(saproling_filter),
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

fn add_spore(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spore = match reg.interner().lookup("spore") {
        Some(s) => s,
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(spore),
        count: 1,
    }]
}

fn make_saproling(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let saproling = reg.interner().lookup("Saproling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saproling);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: saproling,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn um_add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn um_add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    um_add_one(ctx, ManaColor::White)
}
fn um_add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    um_add_one(ctx, ManaColor::Blue)
}
fn um_add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    um_add_one(ctx, ManaColor::Black)
}
fn um_add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    um_add_one(ctx, ManaColor::Red)
}
fn um_add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    um_add_one(ctx, ManaColor::Green)
}
