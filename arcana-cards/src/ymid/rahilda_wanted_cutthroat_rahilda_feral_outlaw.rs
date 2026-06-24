//! Rahilda, Wanted Cutthroat // Rahilda, Feral Outlaw
//! Front: `{1}{R}` Legendary Creature — Human Werewolf 2/2
//!   First strike
//!   Daybound
//!   When Rahilda deals combat damage to a player, exile a nonland card
//!   from their library at random. During any turn you attacked with a
//!   Wolf or Werewolf, you may cast that card and spend mana as any color.
//!
//! Back: Legendary Creature — Werewolf
//!   Double strike
//!   Nightbound
//!   Same combat damage trigger.
//!
//! The combat-damage trigger prints on BOTH faces (Wanted Cutthroat and Feral
//! Outlaw), so it lives on the shared CardDefinition and is intentionally
//! ungated — it fires on either face. Only the trigger's EFFECT is unwired (see
//! GAPs below).
//!
//! GAPs:
//! - "Exile a nonland card from their library at random" — no Effect models
//!   random-exile from ANOTHER player's library with a nonland filter
//!   (`ImpulseExile` exiles the top N of the controller's OWN library); omitted
//!   from the trigger effect.
//! - "You may cast that card … spend mana as any color" — cast-an-exiled-card
//!   (owned by the damaged player) by the trigger controller, spending mana as
//!   any color, is not in the engine's Effect catalog.
//! - Daybound / Nightbound — day/night cycle is not modeled. Keywords
//!   omitted (not in the implemented keyword list).
//! - Transform between day/night faces not wired (no day/night cycle
//!   engine support).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rahilda, Wanted Cutthroat");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Rahilda, Feral Outlaw");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::DoubleStrike],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Combat-damage trigger — prints on BOTH faces, so it is shared on
            // the CardDefinition and intentionally ungated (fires on either face).
            // GAP is the effect (random opponent-library exile + impulse-cast),
            // not the trigger wiring.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: rahilda_damage_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn rahilda_damage_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile a nonland card from their library at random" — no Effect
    // for random-exile-from-library. "You may cast that card, spend mana as
    // any color" — not in Effect catalog. Returning empty.
    vec![]
}
