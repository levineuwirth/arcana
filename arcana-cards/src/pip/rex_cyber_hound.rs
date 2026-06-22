//! Rex, Cyber-Hound — `{1}{W}{U}` 2/2 Legendary Artifact Creature — Robot Dog.
//!
//! "Whenever Rex deals combat damage to a player, they mill two cards and you
//!  get {E}{E} (two energy counters).
//!  Pay {E}{E}: Choose target creature card in a graveyard. Exile it with a
//!  brain counter on it. Activate only as a sorcery.
//!  Rex has all activated abilities of all cards in exile with brain counters
//!  on them."
//!
//! The Scryfall "Mill" tag is a reminder, not a `KeywordAbility` variant →
//! keywords empty.
//!
//! Ability 1 (combat-damage trigger) IS expressible: the damaged player mills
//! two, and the controller gets two energy.
//!
//! Ability 2's cost is "Pay {E}{E}" — spending energy is not an `ActivationCost`
//! field, and "exile a graveyard card WITH A BRAIN COUNTER ON IT" cannot be
//! expressed (no exile-with-counter Effect) — GAP'd.
//!
//! Ability 3 is a static that grants Rex the activated abilities of exiled
//! brain-countered cards — no hook for borrowing abilities from exile — GAP'd.

// GAP (ability): "Pay {E}{E}: … Exile [a graveyard creature card] with a brain
// counter on it. Activate only as a sorcery." — energy is not a cost field, and
// there is no exile-with-counter Effect.
// GAP (static): "Rex has all activated abilities of all cards in exile with
// brain counters on them." — no hook to grant abilities borrowed from exile.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rex, Cyber-Hound");
    let robot = reg.interner_mut().intern("Robot");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (fidelity): source_filter can't be pinned to THIS creature
            // only (no self-source ObjectFilter primitive); broadened to your
            // creatures per catalog precedent (Hystrodon).
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_mill_energy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_mill_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    vec![
        Effect::Mill { player: p, count: 2 },
        Effect::GainEnergy {
            player: trig.controller,
            amount: 2,
        },
    ]
}
