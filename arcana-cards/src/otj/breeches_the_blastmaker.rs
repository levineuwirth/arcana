//! Breeches, the Blastmaker — `{1}{U}{R}` 3/3 Legendary Goblin Pirate.
//! Menace.
//! Whenever you cast your second spell each turn, you may sacrifice an
//! artifact. If you do, flip a coin. When you win the flip, copy that
//! spell. You may choose new targets for the copy. When you lose the
//! flip, Breeches deals damage equal to that spell's mana value to any
//! target.
//!
//! Menace is a base keyword. The trigger fires on a spell you cast. The
//! "second spell each turn" gating, the optional artifact-sacrifice cost
//! (OptionalPayment supports only mana/life, not sacrifice), and the
//! coin-flip branch (copy that spell / deal that-spell's-mana-value
//! damage — neither "that spell" nor its mana value is readable at the
//! trigger) are all GAPs, so the effect body is empty.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breeches, the Blastmaker");
    let goblin = reg.interner_mut().intern("Goblin");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP (frequency): "your second spell each turn" — the ordinal
                // gate is not expressible; fires on each spell you cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: breeches_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn breeches_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice an artifact" (OptionalPayment has no sacrifice
    // kind) gating a coin flip whose branches copy "that spell" or deal
    // damage equal to "that spell's mana value" — the triggering spell and
    // its mana value are not readable from this trigger.
    Vec::new()
}
