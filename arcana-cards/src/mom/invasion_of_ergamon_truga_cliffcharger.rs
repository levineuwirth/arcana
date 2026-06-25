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
//! Front ETB "Then you may discard a card. If you do, draw a card." is wired
//! via Effect::OptionalPayment { Discard(1) → draw 1 }.
//!
//! Defeat-transform to the Rhino back face is auto-wired by the engine SBA
//! (CR 310.11). The back face's "when this creature enters, you may discard a
//! card; if you do, search your library for a land or battle card" fires on the
//! SelfTransforms{to_face:Some(1)} trigger via OptionalPayment { Discard(1) →
//! TutorToHand(land or battle) }. Trample is intrinsic on the back-face chars.
//!
//! Defense counter count: 4 (as printed on Invasion of Ergamon).
//! Back face P/T: 4/4 (as printed on Truga Cliffcharger).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
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
            // Front ETB: create Treasure, then may discard → draw.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_treasure_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back-face ETB (fires on the defeat-transform): you may discard a
            // card; if you do, search your library for a land or battle card,
            // reveal it, put it into your hand, then shuffle.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: back_discard_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn back_discard_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::LAND | TypeLine::BATTLE));
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::TutorToHand {
            player: trig.controller,
            filter,
            reveal: true,
        }),
        else_effect: None,
    }]
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
        // "Then you may discard a card. If you do, draw a card." — optional loot.
        Effect::OptionalPayment {
            chooser: trig.controller,
            cost: OptionalPaymentKind::Discard(1),
            then: Box::new(Effect::DrawCards {
                player: trig.controller,
                count: 1,
            }),
            else_effect: None,
        },
    ]
}
