//! Invasion of Ikoria // Zilortha, Apex of Ikoria
//!
//! Front face: `{X}{G}{G}` Battle — Siege (green) with 7 defense counters.
//! ETB: search your library and/or graveyard for a non-Human creature card with
//! mana value X or less and put it onto the battlefield. If you search your library,
//! shuffle.
//!
//! Back face: Legendary Creature — Dinosaur (5/5), Reach.
//! For each non-Human creature you control, you may have that creature assign its
//! combat damage as though it weren't blocked.
//!
//! Defeat-transform to the Zilortha creature back face is auto-wired by the engine
//! SBA (CR 310.11); Reach is intrinsic on the back-face characteristics.
//!
//! # GAPs
//! - "non-Human" ETB tutor filter wired via `ObjectFilter::without_subtype_sym`.
//! - "mana value X or less" — X is the variable cost paid at cast time; X is not
//!   accessible in the effect resolver. Searching for any creature (no CMC cap).
//! - "Search your library and/or graveyard" — only library tutor is available.
//! - Back face "for each non-Human creature you control, you may have that creature
//!   assign its combat damage as though it weren't blocked" — no combat-damage-
//!   assignment replacement (assign-as-though-unblocked) primitive exists in the
//!   engine; the static is genuinely inexpressible and left unwired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Ikoria");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Zilortha, Apex of Ikoria
    let back_name = reg.interner_mut().intern("Zilortha, Apex of Ikoria");
    let dinosaur_sub = reg.interner_mut().intern("Dinosaur");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dinosaur_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 7,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_resolve(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "non-Human" wired via without_subtype_sym.
    // GAP: mana value X or less — X not accessible at resolve time.
    // GAP: AND/OR graveyard search — only library search is available.
    let mut filter = ObjectFilter::creature();
    if let Some(human) = reg.interner().lookup("Human") {
        filter = filter.without_subtype_sym(human);
    }
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: false,
    }]
}
