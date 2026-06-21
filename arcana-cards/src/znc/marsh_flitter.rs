//! Marsh Flitter — `{3}{B}` 1/1 black Faerie Rogue.
//!
//! Oracle:
//! * Flying.
//! * "When this creature enters, create two 1/1 black Goblin Rogue
//!   creature tokens." — an ETB trigger that mints two 1/1 black Goblin
//!   Rogue tokens (two CreateToken effects).
//! * "Sacrifice a Goblin: This creature has base power and toughness 3/3
//!   until end of turn." — an activated ability whose cost sacrifices a
//!   chosen Goblin you control (`sacrifice_other`), setting this creature's
//!   base P/T to 3/3 until end of turn.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marsh Flitter");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let goblin_filter = script::subtype_filter(reg, "Goblin");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_goblin_rogues,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Goblin: This creature has base power and toughness 3/3 until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(goblin_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_three_three,
            }),
    )
}

fn make_goblin_rogues(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn become_three_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
    }]
}
