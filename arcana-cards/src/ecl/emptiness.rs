//! Emptiness — `{4}{W/B}{W/B}` 3/5 Elemental Incarnation (B/W).
//! When this creature enters, if {W}{W} was spent to cast it, return target
//!   creature card with mana value 3 or less from your graveyard to the battlefield.
//! When this creature enters, if {B}{B} was spent to cast it, put three -1/-1
//!   counters on up to one target creature.
//! Evoke {W/B}{W/B}.
//!
//! Both ETB triggers are gated by a "which colored mana was spent to cast it"
//! intervening-if that the engine cannot evaluate; firing them unconditionally
//! would be wrong, so the effects are GAP'd. Evoke is not in the supported
//! keyword surface.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Emptiness");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W/B}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Evoke {W/B}{W/B} — not in the supported keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_white,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_black,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_white(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if {W}{W} was spent to cast it" is a mana-spent intervening-if the
    // engine cannot evaluate (no record of which colored mana paid the cost).
    // Firing the reanimation unconditionally would be wrong, so the whole
    // ability is GAP'd.
    Vec::new()
}

fn etb_black(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if {B}{B} was spent to cast it" — same mana-spent gate; the
    // -1/-1 counters would otherwise fire unconditionally, which is wrong.
    Vec::new()
}
