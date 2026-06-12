//! Goblin Assassin — `{3}{R}{R}` 2/2 Goblin Assassin.
//! Whenever this creature or another Goblin enters, each player flips a coin.
//! Each player whose coin comes up tails sacrifices a creature of their choice.
//! The ZoneChange filter is restricted to Goblin creatures; since this card is
//! itself a Goblin, "this creature or another Goblin" = any Goblin entering.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Assassin");
    let goblin = reg.interner_mut().intern("Goblin");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().with_subtype_sym(goblin),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: each_player_flip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_player_flip(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let players = script::all_players(state);
    let inner: Vec<Effect> = players.into_iter().map(|p| Effect::FlipCoin {
        player: p,
        win: Box::new(Effect::Sequence(vec![])),
        lose: Some(Box::new(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })),
    }).collect();
    vec![Effect::Sequence(inner)]
}
