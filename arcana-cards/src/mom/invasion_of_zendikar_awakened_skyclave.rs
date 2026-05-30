//! Invasion of Zendikar // Awakened Skyclave — `{3}{G}` green Battle — Siege (front face).
//! Enters with 4 defense counters. When this Siege enters, search your library for up to
//! two basic land cards, put them onto the battlefield tapped, then shuffle.
//! Back face: Creature — Elemental with Vigilance and Haste (and is also a land).
//!
//! # GAPs
//! - "Search library for up to two basic land cards, put onto battlefield tapped" —
//!   TutorToBattlefield with `tapped: true` is not shown in catalog as "tapped: true";
//!   using `tapped: false` as best effort (GAP: lands enter untapped instead of tapped).
//!   Actually the catalog shows `tapped: false` as a parameter, so tapped: true should work.
//! - "up to two" basic lands — TutorToBattlefield is single-card; emitting it twice for
//!   the up-to-two is a GAP (each tutors one; the player must find two separately).
//!   Actually, emit two separate TutorToBattlefield calls with up-to-1 semantics as
//!   best effort: the engine can't do "you may" on each individually.
//! - Back-face static "it's a land in addition to its other types" — static type-adding
//!   layer effect not in Effect catalog; GAP: omitted.
//! - Back-face activated "{T}: Add one mana of any color" — activated ability on back face
//!   not separately modeled (see MDFC/Transform back-face activated ability note).
//! - defeat→cast-back-face not auto-wired (CR 310.11).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Zendikar");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Awakened Skyclave — Creature — Elemental
    let back_name = reg.interner_mut().intern("Awakened Skyclave");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![
                KeywordAbility::Vigilance,
                KeywordAbility::Haste,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };
    // GAP: defeat→cast-back-face not auto-wired (CR 310.11).
    // GAP: back face "is a land in addition to its other types" — static type-adding not modeled.
    // GAP: back face "{T}: Add one mana of any color" — mana activated ability on back face not modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_search,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_search(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for up to two basic land cards and put them onto the battlefield tapped.
    // Emitting two TutorToBattlefield effects as best effort for "up to two".
    // GAP: "tapped: true" — TutorToBattlefield has `tapped` field; using tapped: true.
    let basic_land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: basic_land_filter.clone(),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: basic_land_filter,
            tapped: true,
        },
    ]
}
