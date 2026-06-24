//! Serpent Generator — `{6}` artifact.
//! "{4}, {T}: Create a 1/1 colorless Snake artifact creature token.
//! It has 'Whenever this creature deals damage to a player, that
//! player gets a poison counter.'"
//!
//! The token's triggered ability rides on `TokenDefinition.abilities` (a
//! `Vec<TriggeredAbilityDef>`): a `DamageDealt` trigger (any damage, not
//! combat-only) whose source is restricted to a Snake by name and whose effect
//! gives the damaged player a poison counter via Effect::GivePlayerCounters.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serpent Generator");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Create a 1/1 colorless Snake artifact \
                       creature token. It has \"Whenever this creature \
                       deals damage to a player, that player gets a poison \
                       counter.\""
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_snake,
            },
        ),
    )
}

fn make_snake(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: snake,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            // "Whenever this creature deals damage to a player, that player
            // gets a poison counter." The damage source is restricted to a
            // Snake by name (the token's own name); damage is any kind, not
            // just combat.
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter { name: Some(snake), ..ObjectFilter::default() },
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: snake_poisons_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn snake_poisons_player(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player gets a poison counter."
    let Some(player) = trig.damaged_player() else { return Vec::new(); };
    vec![Effect::GivePlayerCounters { player, kind: CounterKind::Poison, count: 1 }]
}
