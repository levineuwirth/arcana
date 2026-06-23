//! Sifter Wurm — `{5}{G}{G}` 7/7 Wurm with Trample.
//! "When this creature enters, scry 3, then reveal the top card of your
//! library. You gain life equal to that card's mana value."
//!
//! Trample is a base keyword. The ETB trigger performs Scry 3 (fully
//! expressible). The "reveal the top card, gain life equal to its mana value"
//! tail is GAP'd: no `script::` helper exposes the mana value of the top card
//! of a library, and the catalog permits no other state access, so the gain-
//! life amount cannot be computed — emitting a literal would be materially
//! wrong. The Scry half is still wired.

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
    let name = reg.interner_mut().intern("Sifter Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card of your library; you gain life equal to that
    // card's mana value" — no script helper for the top card's mana value, so
    // the dynamic gain-life amount cannot be computed. Scry 3 is wired.
    vec![Effect::Scry { player: trig.controller, count: 3 }]
}
