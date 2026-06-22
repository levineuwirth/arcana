//! Rosheen, Roaring Prophet — `{2}{R}{G}` Legendary 4/4 Creature —
//! Giant Shaman.
//! "When Rosheen enters, mill six cards. You may put a card with {X} in its
//!  mana cost from among them into your hand."
//! "{T}: Reveal any number of cards with {X} in their mana cost in your hand.
//!  Add {C}{C} for each card revealed this way. Spend this mana only on costs
//!  that contain {X}."
//!
//! Decomposition:
//! - Keyword line: "Mill" (Scryfall tag) is not in the usable keyword surface
//!   — `keywords: vec![]`; the mill effect itself is in the ETB ability below.
//! - ETB → mill six (expressible). GAP: "You may put a card with {X} in its
//!   mana cost from among them into your hand" (filter by {X} in mana cost over
//!   the milled cards) is not expressible.
//! - `{T}: …` → an activated ability declared with the tap cost.
//!   GAP: revealing any-number of {X}-cost cards and producing {C}{C} per
//!   reveal with the restricted-spend rider is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rosheen, Roaring Prophet");
    let giant = reg.interner_mut().intern("Giant");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Reveal any number of cards with {X} in their mana cost in your hand. \
                       Add {C}{C} for each card revealed this way. Spend this mana only on costs \
                       that contain {X}."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_x_for_mana,
            }),
    )
}

/// Mill six. GAP: "You may put a card with {X} in its mana cost from among them
/// into your hand" — filtering milled cards by {X}-in-mana-cost is not
/// expressible.
fn etb_mill(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 6 }]
}

/// GAP: reveal any-number of {X}-cost cards and produce {C}{C} per reveal with
/// the restricted-spend rider is not expressible.
fn reveal_x_for_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
