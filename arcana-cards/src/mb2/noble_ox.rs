//! Noble Ox — `{3}{W}` 2/4 Noble Ox with Flash.
//!
//! * Flash.
//! * When Noble Ox enters the battlefield, all unblocked creatures
//!   attacking you become blocked by Noble Ox. (GAP — no effect makes
//!   attacking creatures become blocked.)
//! * Noble Ox can block any number of creatures. (static — GAP)

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
    let name = reg.interner_mut().intern("Noble Ox");
    let noble = reg.interner_mut().intern("Noble");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(noble);
    subtypes.0.insert(ox);

    // GAP: "Noble Ox can block any number of creatures" is a static
    // blocking permission; no expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_become_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_become_blocked(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "all unblocked creatures attacking you become blocked by
    // Noble Ox" — no Effect makes already-attacking creatures become
    // blocked by a chosen blocker.
    Vec::new()
}
