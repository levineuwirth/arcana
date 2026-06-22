//! Warren Instigator — `{R}{R}` 1/1 Goblin Berserker with Double strike.
//! "Whenever this creature deals damage to an opponent, you may put a
//! Goblin creature card from your hand onto the battlefield."
//!
//! Keyword line: Double strike. One damage trigger: DamageDealt to a
//! player (the established self-damage idiom uses an empty source_filter
//! + TargetFilter::Player; "an opponent" narrows to opponents, a fidelity
//! gap of the player-target filter). The payoff puts a Goblin creature
//! card from hand onto the battlefield (the "may" is the put's own pick;
//! no-op if no Goblin in hand).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warren Instigator");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: put_goblin_from_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn put_goblin_from_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin = reg.interner().lookup("Goblin");
    let mut filter = ObjectFilter::creature();
    if let Some(g) = goblin {
        filter = filter.with_subtypes_any(vec![g]);
    }
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter,
        tapped: false,
    }]
}
