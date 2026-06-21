//! Hatchery Spider — `{5}{G}{G}` 5/7 Spider.
//!
//! Reach.
//! Undergrowth — When you cast this spell, reveal the top X cards of your
//! library, where X is the number of creature cards in your graveyard.
//! You may put a green permanent card with mana value X or less from
//! among them onto the battlefield. Put the rest on the bottom of your
//! library in a random order.
//!
//! Reach is expressible. The Undergrowth cast trigger is fired on
//! `SpellCast` (filtered to this card by name), but its effect is GAP'd:
//! both the reveal depth (X) and the mana-value ceiling (X) are
//! resolution-time dynamic amounts, and `RevealUntil`/`DigTopN` take a
//! static filter + static depth, so the "reveal X, put a green permanent
//! with mv ≤ X onto the battlefield" shape isn't expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hatchery Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let this_name = reg.interner_mut().intern("Hatchery Spider");
    let cast_filter = ObjectFilter {
        name: Some(this_name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(cast_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: undergrowth_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn undergrowth_reveal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal X (= creature cards in graveyard) and put a green
    // permanent with mana value ≤ X onto the battlefield — both the
    // reveal depth and the mv ceiling are dynamic; RevealUntil/DigTopN
    // only take a static filter + static depth.
    Vec::new()
}
