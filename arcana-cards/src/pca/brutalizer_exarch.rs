//! Brutalizer Exarch — `{5}{G}` 3/3 Phyrexian Cleric.
//! "When this creature enters, choose one —
//!  • Search your library for a creature card, reveal it, then shuffle
//!    and put that card on top.
//!  • Put target noncreature permanent on the bottom of its owner's
//!    library."
//!
//! Modal selection is only available for spell abilities, not for a
//! triggered ETB ability, so the "choose one" gate is GAP'd. The second
//! mode — putting a target noncreature permanent on the bottom of its
//! owner's library — is the expressible, targeted mode and is wired as
//! the ETB trigger. Mode one (search-and-put-on-top) has no tutor-to-top
//! effect and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: modality — a triggered ETB "choose one —" has no modal mechanism
// (modal is spell-ability-only); only the targeted second mode is wired.
// GAP: mode one — "Search your library for a creature card, reveal it, then
// shuffle and put that card on top." There is no tutor-to-top-of-library
// effect (TutorToHand / TutorToBattlefield only).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brutalizer Exarch");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: bottom_noncreature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn bottom_noncreature(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::PutOnBottomOfLibrary { target: *id }]
}
