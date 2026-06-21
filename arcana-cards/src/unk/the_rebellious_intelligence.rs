//! The Rebellious Intelligence — `{5}` 5/5 Legendary Artifact Creature — Rebel.
//!
//! Oracle:
//! * At the beginning of your upkeep, put a random card from outside the game
//!   into your hand. (GAP: outside-the-game / wishboard access is not modeled.)
//! * {T}: Add {W}{U}{B}{R}{G}. This mana can't be spent to pay generic mana
//!   costs. (The five fixed-color mana is modeled via AddMana; the
//!   spend-restriction rider is a fidelity GAP — no spend-restriction API.)
//!
//! Decomposition: upkeep trigger → one `TriggeredAbilityDef` (GAP body);
//! the five-color tap-for-mana → one `ActivatedAbilityDef`.

use arcana_core::effects::Effect;
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
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Rebellious Intelligence");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rebel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
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
                effect: upkeep_outside_game,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {W}{U}{B}{R}{G}. This mana can't be spent to pay \
                       generic mana costs."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                // The spend-restriction rider can't be modeled, so this is not a
                // pure mana ability (it would skip the stack incorrectly).
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_wubrg,
            }),
    )
}

fn upkeep_outside_game(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a random card from outside the game into your hand" — no
    // wishboard / outside-the-game access in the demonstrated API.
    Vec::new()
}

fn add_wubrg(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP fidelity: the "can't be spent to pay generic mana costs" restriction
    // is not modeled; the five colored mana itself is added faithfully.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
