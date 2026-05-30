//! Invasion of Arcavios // Invocation of the Founders — {3}{U}{U} Battle — Siege (front)
//! ETB: Search your library, graveyard, and/or outside the game for an instant or sorcery card
//!      you own, reveal it, and put it into your hand. If you searched your library, shuffle.
//! Defeated → Invocation of the Founders
//! Back: Enchantment. Whenever you cast an instant or sorcery spell from your hand, you may copy
//!       that spell. You may choose new targets for the copy.
//! GAP: "search outside the game" not expressible.
//! GAP: "search library and/or graveyard" combined — using TutorToHand (library search only;
//!      graveyard portion not expressible via engine API).
//! GAP: Back face "whenever you cast an instant or sorcery from your hand, copy it" triggered
//!      ability not modeled (back-face-only triggered abilities not auto-installed on transform).
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).
//! Defense counter count not stated in oracle; defaulting to 4 (typical Siege value).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Arcavios");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Invocation of the Founders");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: SubtypeSet::default(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_transform_back(back)
            // ETB trigger: search for instant or sorcery
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_search,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face triggered ability (copy instant/sorcery you cast from hand) not modeled.
        // GAP: defeat→cast-back-face not auto-wired.
    )
}

fn etb_search(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "search library and/or graveyard and/or outside the game" — only library search
    //      expressible via TutorToHand; graveyard and outside-game portions are not modeled.
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
