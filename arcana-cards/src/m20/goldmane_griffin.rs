//! Goldmane Griffin — `{3}{W}{W}` 3/2 Griffin with Flying, Vigilance.
//!
//! Oracle:
//! * Flying, vigilance.
//! * When this creature enters, you may search your library and/or
//!   graveyard for a card named Ajani, Inspiring Leader, reveal it, and
//!   put it into your hand. If you search your library this way,
//!   shuffle.
//!
//! Implemented as a library tutor-by-name to hand (TutorToHand with a
//! name filter; shuffle is automatic). The "and/or graveyard" search
//! half is not separately expressible — documented partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goldmane Griffin");
    let griffin = reg.interner_mut().intern("Griffin");
    // Pre-intern the searched name so the resolver's lookup succeeds.
    let _ajani = reg.interner_mut().intern("Ajani, Inspiring Leader");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: tutor_ajani,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tutor_ajani(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard half of "library and/or graveyard" — only the
    // library tutor-by-name is expressed.
    let nm = reg.interner().lookup("Ajani, Inspiring Leader");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}
