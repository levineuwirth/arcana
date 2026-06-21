//! Thought Harvester — `{3}{U}` 2/4 Creature — Eldrazi Drone.
//!
//! Oracle:
//! * Devoid (this card has no color).
//! * Flying.
//! * Whenever you cast a colorless spell, target opponent exiles the top card
//!   of their library.
//!
//! Decomposition: Devoid → colorless `colors`; Flying → `keywords`; the
//! colorless-spell trigger → one `TriggeredAbilityDef`. The effect body —
//! "exile the top card of target opponent's library" — has no matching
//! primitive (no exile-from-library-top effect; Mill goes to graveyard), so
//! the trigger fires and chooses its target but the effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thought Harvester");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_colors(ColorSet::colorless())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: opponent_exiles_top,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn opponent_exiles_top(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of their library" — no exile-from-library-top
    // primitive (Mill puts cards in the graveyard, not exile).
    Vec::new()
}
