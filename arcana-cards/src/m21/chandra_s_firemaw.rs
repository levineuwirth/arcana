//! Chandra's Firemaw — `{3}{R}{R}` 4/2 Hellion with Haste.
//!
//! Oracle: "Haste. When this creature enters, you may search your library
//! and/or graveyard for a card named Chandra, Flame's Catalyst, reveal it,
//! and put it into your hand. If you search your library this way, shuffle."
//!
//! Haste is a base keyword. The ETB tutors a card with the exact name
//! "Chandra, Flame's Catalyst" from the library to hand (the canonical
//! search-by-name tutor; the shuffle is automatic). The optional graveyard
//! branch of the "library and/or graveyard" search is not separately
//! expressible (no by-name graveyard retrieval primitive), so only the
//! library search is wired.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Chandra's Firemaw");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    // Pre-intern the searched card name so the lookup in the resolver hits.
    let _searched = reg.interner_mut().intern("Chandra, Flame's Catalyst");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_chandra,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_chandra(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("Chandra, Flame's Catalyst");
    // Library search by exact card name → hand (shuffle is automatic).
    // GAP: the "and/or graveyard" alternative branch — no by-name retrieval
    // from graveyard primitive.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}
