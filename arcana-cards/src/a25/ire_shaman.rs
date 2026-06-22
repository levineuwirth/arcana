//! Ire Shaman — `{1}{R}` 2/1 Orc Shaman.
//!
//! * Menace.
//! * Megamorph {R} (not an expressible keyword — the morph/megamorph cast
//!   mechanic is not modeled, so it is omitted from `keywords`).
//! * When this creature is turned face up, exile the top card of your
//!   library. Until end of turn, you may play that card. — modeled as an
//!   `ImpulseExile` of 1; the trigger CONDITION is GAP'd (no "turned face up"
//!   variant; `SelfTransforms` is the closest analogue).

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
    let name = reg.interner_mut().intern("Ire Shaman");
    let orc = reg.interner_mut().intern("Orc");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger — "When this creature is turned face up". No
            // morph "turned face up" TriggerCondition exists; SelfTransforms
            // (turn to front face) is the closest analogue.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: impulse_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn impulse_top_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
