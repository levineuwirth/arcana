//! Thieving Amalgam — `{5}{B}{B}` 6/7 black Ape Snake.
//!
//! Oracle:
//! * "At the beginning of each opponent's upkeep, you manifest the top
//!   card of that player's library." — modeled with StepBegins(Upkeep,
//!   Opponent) firing `Effect::Manifest { player: active_player }`.
//!   GAP: the engine's Manifest manifests the player's OWN top card
//!   under their own control; the real card manifests the active
//!   opponent's library card under YOUR control. Fidelity gap on both
//!   the source library and the controller.
//! * "Whenever a creature you control but don't own dies, its owner
//!   loses 2 life and you gain 2 life." — modeled via a ZoneChange
//!   (creature you control, battlefield → graveyard) granting you 2
//!   life. GAP: the "control but don't own" restriction and the "its
//!   owner loses 2 life" half are not cleanly expressible (no owner
//!   accessor on the dying object); only the GainLife half fires, with
//!   no control-not-own gate.
//!
//! GAP: Manifest is not a `KeywordAbility` variant → `keywords: vec![]`.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Thieving Amalgam");
    let ape = reg.interner_mut().intern("Ape");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![], // GAP: Manifest is not a KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: manifest_opponent_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: stolen_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn manifest_opponent_top(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Manifest puts the named player's OWN top card under their own
    // control; the oracle manifests the active opponent's library card
    // under YOUR control. Both the source library and the controller are
    // fidelity gaps.
    vec![Effect::Manifest { player: state.active_player() }]
}

fn stolen_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "control but don't own" restriction and "its owner loses
    // 2 life" half aren't cleanly expressible (no owner accessor on the
    // dying object). Only the GainLife half fires, with no own-vs-control
    // gate.
    vec![Effect::GainLife { player: trig.controller, amount: 2 }]
}
