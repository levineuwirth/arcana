//! Lightkeeper of Emeria — `{3}{W}` 2/4 Creature — Angel.
//! Multikicker {W}.
//! Flying.
//! When this creature enters, you gain 2 life for each time it was kicked.
//!
//! Decomposition:
//! 1. Keyword line: Flying. (Multikicker — GAP: not in the usable KeywordAbility
//!    set; no kicker cost field / "was kicked" accessor exists.)
//! 2. ETB trigger present, but its amount ("2 life for each time it was
//!    kicked") depends on the multikicker count, which there is no accessor for.
//!    GAP the effect body (return Vec::new()). The trigger condition is still
//!    recorded.

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
    let name = reg.interner_mut().intern("Lightkeeper of Emeria");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_life_per_kick,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_life_per_kick(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you gain 2 life for each time it was kicked." Multikicker count is
    // not modeled — no kicker cost field nor "times kicked" accessor exists, so
    // the dynamic amount cannot be computed. Effect omitted.
    Vec::new()
}
