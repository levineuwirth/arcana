//! Phabine, Boss's Confidant — `{3}{R}{G}{W}` 3/6 Legendary Cat Advisor.
//! "Creature tokens you control have haste."; Parley — "At the
//! beginning of combat on your turn, each player reveals the top card
//! of their library. For each land card revealed this way, you create a
//! 1/1 green and white Citizen token. Then creatures you control get
//! +1/+1 until end of turn for each nonland card revealed this way.
//! Then each player draws a card."
//!
//! The static "creature tokens you control have haste" is GAP'd (pure
//! continuous static keyword grant). The Parley combat trigger is wired
//! for the faithful tail step ("then each player draws a card"); the
//! reveal-and-count steps (tokens per land revealed, pump per nonland
//! revealed) are GAP'd — there is no effect that reveals the top of
//! each library and branches on the revealed cards' types.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phabine, Boss's Confidant");
    let cat = reg.interner_mut().intern("Cat");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static "Creature tokens you control have haste."
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: parley_each_player_draws,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn parley_each_player_draws(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top of each library, create a Citizen token per land
    //      revealed, pump creatures you control +1/+1 per nonland revealed
    //      — no reveal-and-count-by-type effect surface. Wires only the
    //      faithful tail: "then each player draws a card".
    let draws = script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    vec![Effect::Sequence(draws)]
}
