//! Goblin Rabblemaster — `{2}{R}` 2/2 Goblin Warrior.
//!
//! 1. "Other Goblin creatures you control attack each combat if able."
//!    — board-wide must-attack, but GAP: the "OTHER" self-exclusion has
//!    no surface. `FilteredMustAttack`/`ObjectFilter` cannot exclude the
//!    source (the `custom` predicate receives no source id), so a
//!    Goblin-you-control filter would wrongly force Rabblemaster itself
//!    to attack. Left honest pending an exclude-source predicate.
//! 2. "At the beginning of combat on your turn, create a 1/1 red Goblin
//!    creature token with haste."  — triggered, expressible.
//! 3. "Whenever this creature attacks, it gets +1/+0 until end of turn
//!    for each other attacking Goblin."  — triggered, dynamic pump.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Rabblemaster");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Other Goblin creatures you control attack each combat if
    // able" is a static combat restriction with no expressible primitive.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_goblin_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_per_attacking_goblin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_goblin_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: goblin,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}

fn pump_per_attacking_goblin(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Count attacking Goblins, then subtract this creature itself for
    // the "other attacking Goblin" wording.
    let filter = script::subtype_filter(reg, "Goblin")
        .controlled_by(ControllerConstraint::You)
        .attacking_only();
    let total = script::count_matching(state, &filter, trig.controller);
    let n = total.saturating_sub(1);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
