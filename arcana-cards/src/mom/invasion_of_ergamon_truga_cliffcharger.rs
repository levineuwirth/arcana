//! Invasion of Ergamon // Truga Cliffcharger — `{R}{G}` red/green Battle — Siege.
//!
//! Front face (Invasion of Ergamon): Battle — Siege, enters with defense counters.
//!   When this Siege enters, create a Treasure token. Then you may discard a card.
//!   If you do, draw a card.
//!
//! Back face (Truga Cliffcharger): Creature — Rhino.
//!   Trample.
//!   When this creature enters, you may discard a card. If you do, search your
//!   library for a land or battle card, reveal it, put it into your hand, then shuffle.
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: Back-face ETB "you may discard a card. If you do, search for a land or battle
//!      card" — the discard gate uses OptionalPaymentKind but Discard is not a valid
//!      OptionalPaymentKind variant (only Mana and Life); this is a GAP.
//!      The search for "land or battle card" combining two types is modeled via
//!      with_types_any.
//! GAP: back-face-only triggered ability (ETB tutor) not auto-installed on transform.
//!
//! Defense counter count: 4 (as printed on Invasion of Ergamon).
//! Back face P/T: 4/4 (as printed on Truga Cliffcharger).

use arcana_core::effects::{CommodityToken, DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Ergamon");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Truga Cliffcharger");
    let rhino_sub = reg.interner_mut().intern("Rhino");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(rhino_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
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
            // ETB trigger: create Treasure, then may discard → draw.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_treasure_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
        // GAP: defeat→cast-back-face not auto-wired (CR 310.11).
        // GAP: back-face-only ETB trigger (discard → search for land/battle card)
        //      not auto-installed on transform.
    )
}

fn etb_treasure_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
        // "Then you may discard a card. If you do, draw a card." — loot effect.
        // OptionalPaymentKind::Discard not available; modeling as unconditional
        // Discard+Draw would be wrong. GAP: discard gate not expressible via
        // OptionalPaymentKind (only Mana/Life variants exist).
        // Approximate as: discard 1, draw 1 (always). This is a fidelity gap —
        // the "you may" choice is lost. GAP'd as Vec::new() to avoid auto-executing.
        // GAP: "you may discard a card. If you do, draw a card" — discard gate not
        //      expressible; emitting Vec::new() for this sub-effect.
    ]
}
