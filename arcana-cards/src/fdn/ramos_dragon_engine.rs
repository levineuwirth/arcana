//! Ramos, Dragon Engine — `{6}` 4/4 Legendary Artifact Creature Dragon
//! with Flying.
//! "Whenever you cast a spell, put a +1/+1 counter on Ramos for each of
//!  that spell's colors."
//! "Remove five +1/+1 counters from Ramos: Add {W}{W}{U}{U}{B}{B}{R}{R}
//!  {G}{G}. Activate only once each turn."
//!
//! Flying expressible. The cast trigger fires on every spell you cast but
//! the "+1/+1 counter for each of that spell's colors" amount has no
//! accessor for the cast spell's color count — GAP'd effect, trigger
//! structure kept. The remove-five-counters mana ability is wired in full.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ramos, Dragon Engine");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counters_per_color,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove five +1/+1 counters from Ramos: Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}. Activate only once each turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 5)),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_ten_mana,
            }),
    )
}

fn counters_per_color(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+1 counter for each of that spell's colors" — no accessor for
    // the cast spell's color count.
    Vec::new()
}

fn add_ten_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mana = vec![
        ManaUnit::plain(ManaColor::White, ctx.source),
        ManaUnit::plain(ManaColor::White, ctx.source),
        ManaUnit::plain(ManaColor::Blue, ctx.source),
        ManaUnit::plain(ManaColor::Blue, ctx.source),
        ManaUnit::plain(ManaColor::Black, ctx.source),
        ManaUnit::plain(ManaColor::Black, ctx.source),
        ManaUnit::plain(ManaColor::Red, ctx.source),
        ManaUnit::plain(ManaColor::Red, ctx.source),
        ManaUnit::plain(ManaColor::Green, ctx.source),
        ManaUnit::plain(ManaColor::Green, ctx.source),
    ];
    vec![Effect::AddMana { player: ctx.controller, mana }]
}
