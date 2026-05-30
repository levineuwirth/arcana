//! Alrund, God of the Cosmos // Hakka, Whispering Raven — MDFC.
//!
//! Front (Alrund): `{3}{U}{U}` Legendary Creature — God 1/1.
//!   Alrund gets +1/+1 for each card in your hand and each foretold card you
//!   own in exile. At the beginning of your end step, choose a card type, then
//!   reveal the top two cards of your library. Put all revealed cards of the
//!   chosen type into your hand and the rest on the bottom in any order.
//!
//! Back (Hakka): `{1}{U}` Legendary Creature — Bird 2/3. Flying.
//!   Whenever Hakka deals combat damage to a player, return it to its owner's
//!   hand, then scry 2.
//!
//! # GAP: Alrund's static "+1/+1 for each card in hand and foretold cards" —
//!   dynamic continuous P/T modification is not expressible in the static
//!   Characteristics layer; the base P/T 1/1 is registered but the bonus is
//!   not modeled.
//! # GAP: Alrund's end-step ability "choose a card type, reveal top 2, put
//!   matching into hand" — card-type choice prompt is not modeled; the effect
//!   fn returns Vec::new().
//! # GAP: Hakka's triggered ability "whenever Hakka deals combat damage to a
//!   player, return it to hand, then scry 2" — back-face-only triggered
//!   abilities are not auto-installed (engine debt); modeled here but will
//!   fire even when on front face (acceptable approximation).
//! # GAP: Keywords listed by Scryfall include Scry as a keyword ability;
//!   Scry is not a keyword in the engine surface — omitted from keywords vec.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetFilter,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alrund, God of the Cosmos");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Scry is not a keyword in the engine surface
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Hakka, Whispering Raven
    let back_name = reg.interner_mut().intern("Hakka, Whispering Raven");
    let bird_sub = reg.interner_mut().intern("Bird");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid back cost")),
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // Alrund's end step ability: choose card type, reveal top 2, put matching into hand
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: alrund_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Hakka's combat damage trigger (GAP: back-face-only; fires on both faces)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: hakka_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn alrund_end_step(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a card type, reveal top two cards, put matching into hand,
    // rest on bottom" — card-type choice prompt not modeled in the engine.
    Vec::new()
}

fn hakka_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Return Hakka to its owner's hand, then scry 2.
    vec![
        Effect::ReturnToHand { target: trig.source },
        Effect::Scry { player: trig.controller, count: 2 },
    ]
}
