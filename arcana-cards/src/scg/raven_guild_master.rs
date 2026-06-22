//! Raven Guild Master — `{1}{U}{U}` 1/1 Creature — Human Wizard Mutant.
//! "Whenever this creature deals combat damage to a player, that player exiles
//!  the top ten cards of their library."
//! Morph {2}{U}{U}.
//!
//! Decomposition:
//! - Keyword line: Morph — NOT in the usable keyword surface, so `keywords:
//!   vec![]` (GAP).
//! - "Whenever this creature deals combat damage to a player, …" →
//!   a DamageDealt(combat, to a player) triggered ability.
//!   GAP: "that player exiles the top ten cards of their library" has no
//!   expressible Effect (no exile-top-N-of-a-player's-library primitive), so
//!   the effect body returns an empty vec.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raven Guild Master");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword `Morph` not in usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: exile_ten,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// GAP: "that player exiles the top ten cards of their library" — no
/// exile-top-N-of-a-player's-library Effect primitive is available.
fn exile_ten(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
