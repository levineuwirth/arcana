//! Delivery Moogle — `{3}{W}` 3/2 white Moogle with Flying.
//!
//! Oracle:
//! * Flying → `keywords`.
//! * "When this creature enters, search your library and/or graveyard for
//!   an artifact card with mana value 2 or less, reveal it, and put it
//!   into your hand. If you search your library this way, shuffle." — an
//!   ETB tutor. The library search is modeled with `TutorToHand` (artifact
//!   filter, mv ≤ 2; shuffle is automatic). The optional graveyard search
//!   branch ("and/or graveyard") has no combined library-or-graveyard
//!   tutor primitive — GAP'd (library branch implemented).

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
    let name = reg.interner_mut().intern("Delivery Moogle");
    let moogle = reg.interner_mut().intern("Moogle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moogle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tutor_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_tutor_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and/or graveyard" — no combined library-or-graveyard tutor;
    // the library branch is implemented below.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .with_max_cmc(2),
        reveal: true,
    }]
}
