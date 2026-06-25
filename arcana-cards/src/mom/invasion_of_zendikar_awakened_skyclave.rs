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
//! - Back-face "it's a land in addition to its other types" — baked directly into the
//!   registered back-face type line (CREATURE | LAND).
//! - Back-face activated "{T}: Add one mana of any color" — modeled as five mana
//!   abilities, one per WUBRG color (command_tower idiom), face-gated to the back
//!   face (face 1); the shared {T} cost means activating one taps the source, so
//!   only one fires.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
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
            // "…and is still a land": the always-on extra card type is
            // baked directly into the back face's type line.
            types: TypeLine(TypeLine::CREATURE | TypeLine::LAND),
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
    // Back face "{T}: Add one mana of any color" — five mana abilities, one per
    // WUBRG color, face-gated to the back face (face 1).

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
            })
            .with_activated_ability(back_mana_ability("{T}: Add {W}.", add_white_mana))
            .with_activated_ability(back_mana_ability("{T}: Add {U}.", add_blue_mana))
            .with_activated_ability(back_mana_ability("{T}: Add {B}.", add_black_mana))
            .with_activated_ability(back_mana_ability("{T}: Add {R}.", add_red_mana))
            .with_activated_ability(back_mana_ability("{T}: Add {G}.", add_green_mana)),
    )
}

fn back_mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: Some(1),
        effect,
    }
}

fn add_white_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_red_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green_mana(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
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
