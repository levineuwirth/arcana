//! Agency Outfitter — `{4}{U}{U}` 4/3 Creature — Sphinx Detective. Flying.
//! "When this creature enters, you may search your graveyard, hand and/or
//! library for a card named Magnifying Glass and/or a card named Thinking Cap
//! and put them onto the battlefield. If you search your library this way,
//! shuffle."
//!
//! Modeled as the library-search portion: two `TutorToBattlefield` searches by
//! exact card name. PARTIAL — the engine's tutor searches the library only, so
//! the hand/graveyard search zones of this ability are not covered.

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
    let name = reg.interner_mut().intern("Agency Outfitter");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(detective);
    // Pre-intern the searched names so the resolver lookups succeed.
    let _mg = reg.interner_mut().intern("Magnifying Glass");
    let _tc = reg.interner_mut().intern("Thinking Cap");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_pair,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_pair(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mg = reg.interner().lookup("Magnifying Glass");
    let tc = reg.interner().lookup("Thinking Cap");
    vec![
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter { name: mg, ..ObjectFilter::default() },
            tapped: false,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter { name: tc, ..ObjectFilter::default() },
            tapped: false,
        },
    ]
}
