//! Daily Bugle Reporters — `{3}{W}` 2/3 Human Citizen.
//! When this creature enters, choose one —
//!   • Puff Piece — Put a +1/+1 counter on each of up to two target creatures.
//!   • Investigative Journalism — Return target creature card with mana value 2
//!     or less from your graveyard to your hand.
//!
//! GAP: a modal "choose one" on a TRIGGERED ability has no dispatch in the
//! demonstrated API (modal support is spell-ability only). We faithfully wire
//! the first mode (Puff Piece — up to two +1/+1 counters); the
//! Investigative-Journalism alternative is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daily Bugle Reporters");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_puff_piece,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: modal choose-one — wiring the Puff Piece mode's targets only.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(2),
                controller: None,
            }],
        }),
    )
}

/// Puff Piece — put a +1/+1 counter on each of up to two target creatures.
fn etb_puff_piece(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}

// GAP: alternative mode "Investigative Journalism — return target creature card
// with mana value 2 or less from your graveyard to your hand" cannot be offered
// as a player choice (no modal dispatch on triggered abilities).
