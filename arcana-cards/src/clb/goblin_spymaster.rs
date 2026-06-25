//! Goblin Spymaster — `{2}{R}` 2/1 Goblin Rogue with First strike.
//! "At the beginning of each opponent's end step, that player creates a 1/1 red
//!  Goblin creature token with 'Creatures you control attack each combat if
//!  able.'"
//!
//! First strike is expressible. The end-step trigger fires on each opponent's
//! end step; "that player" is the active player whose end step it is, so the
//! token is created under that player's control. The token's embedded static
//! ("Creatures you control attack each combat if able") is wired as a
//! token-borne SelfEntersBattlefield trigger installing a board-wide
//! `filtered_must_attack` over creatures that player controls.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Spymaster");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: end_step_make_goblin,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_make_goblin(
    state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" = the active player whose end step it is (an opponent).
    let them = state.active_player();
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // Token-borne static "Creatures you control attack each combat if able":
        // a SelfEntersBattlefield trigger installing a board-wide must-attack.
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: token_install_must_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }],
    };
    vec![Effect::CreateToken {
        controller: them,
        token,
    }]
}

fn token_install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Creatures you control attack each combat if able." The token is the
    // source; FilteredMustAttack reads the source's controller, so
    // controlled_by(You) = creatures that player (the token's controller)
    // controls.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
