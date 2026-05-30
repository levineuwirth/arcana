//! Marang River Regent // Coil and Catch — `{4}{U}{U}` Dragon creature 6/7 with Flying (front) /
//! `{3}{U}` Instant — Omen Adventure (back).
//!
//! Creature face:
//!   Flying.
//!   When this creature enters, return up to two other target nonland permanents to
//!   their owners' hands.
//!
//! Adventure face (Coil and Catch) `{3}{U}` Instant — Omen:
//!   Draw three cards, then discard a card.
//!   (Then shuffle this card into its owner's library.)
//!
//! # Notes
//! - "Up to two other target nonland permanents" — TargetCount::UpTo(2) with a filter
//!   for nonland permanents. The trigger reads the first target and bounces it;
//!   for multi-target we return effects for each target present.
//! - The Omen parenthetical "(Then shuffle this card into its owner's library.)" is
//!   an Omen-subtype rule; modeled as a regular Adventure (the engine handles exile
//!   on Adventure resolution; shuffle-into-library is not modeled — GAP).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marang River Regent");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // Adventure face: Coil and Catch — {3}{U} Instant — Omen
    let adv_name = reg.interner_mut().intern("Coil and Catch");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Draw three cards, then discard a card.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: coil_and_catch_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    // ETB trigger: return up to two other target nonland permanents to their owners' hands
    let nonland_permanent_req = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::new().without_types(TypeLine::LAND.into()),
        ),
        count: TargetCount::UpTo(2),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_bounce,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![nonland_permanent_req],
            }),
    )
}

fn coil_and_catch_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Omen subtype rule "(shuffle into library)" not modeled — Adventure exile applies
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn etb_bounce(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnToHand { target: *id })
        } else {
            None
        }
    }).collect()
}
