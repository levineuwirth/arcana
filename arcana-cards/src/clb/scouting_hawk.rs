//! Scouting Hawk — `{2}{W}` 1/1 Bird with Flying.
//! "Flying
//!  Keen Sight — When this creature enters, if an opponent controls more
//!  lands than you, search your library for a basic Plains card, put it
//!  onto the battlefield tapped, then shuffle."
//!
//! Flying is a base keyword. The ETB tutor (basic Plains → battlefield
//! tapped) is wired. The intervening-if "an opponent controls more lands
//! than you" is a cross-player count comparison with no matching
//! `conditions::` predicate, so it is GAP'd (`intervening_if: None`),
//! making the search unconditional — a documented over-fire.

use arcana_core::effects::KeywordAbility;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scouting Hawk");
    let bird = reg.interner_mut().intern("Bird");
    let _plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "an opponent controls more lands than
            // you" — cross-player count comparison has no conditions::
            // helper; left None so the search fires unconditionally.
            intervening_if: None,
            effect: etb_search_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_search_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::default()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet(SupertypeSet::BASIC));
    if let Some(plains) = reg.interner().lookup("Plains") {
        filter = filter.with_subtype_sym(plains);
    }
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: true,
    }]
}
