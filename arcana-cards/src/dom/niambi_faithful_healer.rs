//! Niambi, Faithful Healer — `{1}{W}{U}` 2/2 Legendary white/blue Human Cleric.
//! "When Niambi enters, you may search your library and/or graveyard for a
//! card named Teferi, Timebender, reveal it, and put it into your hand. If
//! you search your library this way, shuffle."
//!
//! Library search by exact name is expressible via TutorToHand + ObjectFilter
//! with name lookup. Graveyard search ("and/or graveyard") is a GAP — only
//! library tutor is modelled; graveyard half is dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niambi, Faithful Healer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the search target name so lookup succeeds at resolve time.
    let _teferi = reg.interner_mut().intern("Teferi, Timebender");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_teferi,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_teferi(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and/or graveyard" search is not modelled; only library search.
    let nm = reg.interner().lookup("Teferi, Timebender");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}
