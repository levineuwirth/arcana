//! Norman Osborn // Green Goblin — `{1}{U}` MDFC Legendary Creature.
//!
//! Front face (Norman Osborn, Human Scientist Villain 1/1): Norman Osborn can't be
//! blocked. Whenever Norman Osborn deals combat damage to a player, he connives
//! (draw a card, then discard a card; if you discarded a nonland card, put a +1/+1
//! counter on this creature). `{1}{U}{B}{R}`: Transform Norman Osborn (sorcery speed).
//!
//! Back face (Green Goblin, Goblin Human Villain 3/3): Flying, menace.
//! Spells you cast from your graveyard cost {2} less. Goblin Formula — each nonland
//! card in your graveyard has mayhem equal to its mana cost.
//!
//! # GAPs
//! - "Norman Osborn can't be blocked" static ability: modeled as a triggered ability
//!   that grants CantBeBlocked (no static layer for "can't be blocked" on entry).
//!   Actually emitted as an ETB trigger granting CantBeBlocked with
//!   Duration::WhileSourceOnBattlefield — best available approximation.
//! - Connive: draw + discard modeled; the +1/+1 counter conditional on what was
//!   discarded is not expressible (discard content not observable post-discard).
//! - `{1}{U}{B}{R}`: Transform (sorcery speed) — activated ability not modeled
//!   (no ActivatedAbilityDef available for MDFC shape; GAP).
//! - Back face "spells you cast from your graveyard cost {2} less" — cost reduction
//!   static layer not in Effect catalog.
//! - "Goblin Formula" (mayhem) — no engine variant for this mechanic.
//! - Transform and Connive listed as Scryfall keywords; neither is in
//!   KeywordAbility enum — not emitted.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Norman Osborn");
    let human_sub = reg.interner_mut().intern("Human");
    let scientist_sub = reg.interner_mut().intern("Scientist");
    let villain_sub = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(scientist_sub);
    subtypes.0.insert(villain_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Green Goblin
    let back_name = reg.interner_mut().intern("Green Goblin");
    let goblin_sub = reg.interner_mut().intern("Goblin");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_villain_sub = reg.interner_mut().intern("Villain");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(goblin_sub);
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_villain_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Menace],
            // GAP: "spells you cast from your graveyard cost {2} less" — cost reduction not modeled
            // GAP: "Goblin Formula" (mayhem) — no Effect variant for this mechanic
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Front face: "Norman Osborn can't be blocked" — static effect
            // approximated as an ETB trigger granting CantBeBlocked
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_cant_be_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front face: "Whenever Norman Osborn deals combat damage to a player, he connives"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: connive_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: {1}{U}{B}{R}: Transform Norman Osborn (sorcery speed) —
        // activated ability not modeled in MDFC shape.
    )
}

fn etb_cant_be_blocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn connive_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Connive: draw a card, then discard a card.
    // GAP: if you discarded a nonland card, put a +1/+1 counter on this creature —
    // the conditional based on what was discarded is not expressible.
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
