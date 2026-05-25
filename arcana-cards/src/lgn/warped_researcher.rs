//! Warped Researcher — `{4}{U}` 3/4 blue Creature — Human Wizard Mutant.
//! "Whenever a player cycles a card, this creature gains flying and shroud
//! until end of turn."
//! GAP: trigger "whenever a player cycles a card" — no TriggerCondition for
//! cycling; using CardDiscarded/Any as closest approximation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warped Researcher");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "player cycles a card" not available; using
                // CardDiscarded/Any as best effort
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: on_cycle_gain_evasion,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cycle_gain_evasion(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword { target: trig.source, keyword: KeywordAbility::Flying, duration: Duration::EndOfTurn },
        Effect::GrantKeyword { target: trig.source, keyword: KeywordAbility::Shroud, duration: Duration::EndOfTurn },
    ]
}
