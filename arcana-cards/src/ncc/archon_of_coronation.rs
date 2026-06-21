//! Archon of Coronation — `{4}{W}{W}` 5/5 Creature — Archon.
//!
//! * Flying.
//! * "When this creature enters, you become the monarch." — the
//!   monarch designation is not modeled (no Effect for it); the ETB
//!   trigger is recorded with a GAP'd resolver.
//! * "As long as you're the monarch, damage doesn't cause you to lose
//!   life." — a conditional static; no primitive; GAP'd.

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
    let name = reg.interner_mut().intern("Archon of Coronation");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As long as you're the monarch, damage doesn't cause you to
    // lose life." — conditional static, no primitive.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_become_monarch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_become_monarch(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you become the monarch" — the monarch mechanic is not
    // modeled by the engine (no Effect to set it).
    Vec::new()
}
