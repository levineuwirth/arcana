//! Stonecoil Serpent — `{X}` 0/0 Artifact Creature — Snake.
//! "Reach, trample, protection from multicolored. This creature enters
//! with X +1/+1 counters on it."
//!
//! Reach and Trample are evergreen keywords; protection from multicolored
//! is NOT an expressible KeywordAbility variant (GAP'd). The "enters with X
//! +1/+1 counters" rider is the X-cost enters-with mechanic — there is no
//! Effect for enters-with-X-counters keyed to the spell's X, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonecoil Serpent");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: "protection from multicolored" — Protection is not an
        // expressible KeywordAbility variant.
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enters_with_x_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enters_with_x_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "enters with X +1/+1 counters" — the count is the spell's
    // cast X value, which is not available to an ETB trigger resolver
    // (no x_value accessor on PendingTrigger); emitting a literal would
    // be materially wrong, so the whole effect is omitted.
    Vec::new()
}
