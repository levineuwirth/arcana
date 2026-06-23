//! Ashiok's Forerunner — `{3}{U}{B}` 3/3 Human Wizard with Flash.
//!
//! Oracle:
//! * Flash
//! * When this creature enters, you may search your library and/or
//!   graveyard for a card named Ashiok, Sculptor of Fears, reveal it,
//!   and put it into your hand. If you search your library this way,
//!   shuffle.
//!
//! Implemented: Flash keyword + the ETB tutor. The library-search half is
//! `TutorToHand` filtered to the exact card name "Ashiok, Sculptor of
//! Fears" (reveal + shuffle handled by the engine). The graveyard-search
//! half of the "and/or" is GAP'd — there is no name-filtered
//! graveyard→hand retrieval primitive (`ReturnFromGraveyardToHand` is
//! id-targeted, not a named library-style search). The library tutor is
//! the dominant, fully expressible path.

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
    let name = reg.interner_mut().intern("Ashiok's Forerunner");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_ashiok,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_ashiok(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Ashiok, Sculptor of Fears");
    // GAP: the "and/or your graveyard" branch — no name-filtered
    // graveyard→hand search primitive. The library search is wired below.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}
