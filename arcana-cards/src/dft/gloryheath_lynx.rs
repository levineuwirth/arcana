//! Gloryheath Lynx — `{1}{W}` 2/3 Cat Mount with Lifelink and Saddle 2.
//!
//! * Lifelink (keyword).
//! * Whenever this creature attacks while saddled, search your library for a
//!   basic Plains card, reveal it, put it into your hand, then shuffle.
//! * Saddle 2 — the saddle mechanic itself (tap creatures with total power 2+
//!   to become saddled, sorcery-speed) is not modeled; GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Gloryheath Lynx");
    let cat = reg.interner_mut().intern("Cat");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: Saddle 2 — the saddle activation (tap creatures totaling power 2+,
    // becomes saddled until end of turn, sorcery-speed) is not modeled.
    // The "while saddled" condition on the attack trigger is also unmodeled;
    // the search trigger fires on every attack as a best-effort approximation.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: search_for_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn search_for_plains(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let plains = match reg.interner().lookup("Plains") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::default()
        .with_subtype_sym(plains)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
