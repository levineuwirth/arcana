//! Bellowing Mauler — `{4}{B}` 4/6 black Ogre Warrior.
//! "At the beginning of your end step, each player loses 4 life unless they sacrifice
//! a nontoken creature of their choice."
//!
//! Wired via one Effect::OptionalPayment per player: the chooser may
//! Sacrifice(Creature) to avoid the 4-life penalty (else_effect = LoseLife 4).
//! Caveat: SacrificeFilter::Creature can't encode "nontoken" — a player could
//! satisfy the cost with a token creature (minor over-inclusion).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bellowing Mauler");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: each_player_sac_or_lose_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_player_sac_or_lose_life(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each player loses 4 life unless they sacrifice a [nontoken] creature."
    // One penalty-avoidance gate per player: pay = sacrifice a creature, decline
    // = lose 4 life.
    let players = script::all_players(state);
    let effects: Vec<Effect> = players
        .into_iter()
        .map(|p| Effect::OptionalPayment {
            chooser: p,
            cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::LoseLife { player: p, amount: 4 })),
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
