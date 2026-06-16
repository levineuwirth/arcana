//! Sproutback Trudge — `{7}{G}{G}` 9/7 Fungus Beast with Trample.
//! "This spell costs {X} less, where X is the life you gained this turn."
//! "At the beginning of your end step, if you gained life this turn, you
//! may cast this creature from your graveyard."
//!
//! Trample is expressible. The cost-reduction static and the
//! cast-from-graveyard end-step trigger have no expressible primitives —
//! the trigger keeps its structure with a GAP'd (empty) effect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sproutback Trudge");
    let fungus = reg.interner_mut().intern("Fungus");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(beast);

    // GAP: "This spell costs {X} less to cast" — no cost-reduction primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if you gained life this turn" — no matching
            // conditions:: predicate for life-gained-this-turn.
            intervening_if: None,
            effect: cast_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cast_from_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may cast this creature from your graveyard" — no
    // cast-from-graveyard permission Effect in the available catalog.
    Vec::new()
}
