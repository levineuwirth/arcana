//! Howlpack Piper // Wildsong Howler — `{3}{G}` Human Werewolf 2/2.
//! Front face: This spell can't be countered. {1}{G}, {T}: You may put a
//! creature card from your hand onto the battlefield. If it's a Wolf or
//! Werewolf, untap this creature. Activate only as a sorcery.
//! Daybound (GAP: day/night not modeled).
//! Back face (Wildsong Howler): Whenever this creature enters or transforms
//! into Wildsong Howler, look at the top six cards of your library. You may
//! reveal a creature card from among them and put it into your hand. Put the
//! rest on the bottom of your library in a random order.
//! Nightbound (GAP: day/night not modeled).
//!
//! The back-face triggered ability (Whenever this creature enters or transforms
//! into Wildsong Howler, look at the top six cards of your library, you may
//! reveal a creature card from among them and put it into your hand, put the
//! rest on the bottom in a random order) is wired via DigTopN on two triggers
//! (SelfEntersBattlefield + SelfTransforms{to_face: 1}), both gated to the back
//! face (face 1).
//!
//! GAP: "This spell can't be countered" is a static property, not expressible.
//! GAP: "If it's a Wolf or Werewolf, untap this creature" — conditional untap
//! keyed on the picked card; omitted.
//! GAP: Daybound/Nightbound keywords not in engine keyword set.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Howlpack Piper");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Daybound keyword not in engine keyword set
        // GAP: "This spell can't be countered" static property not expressible
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wildsong Howler");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            // GAP: Nightbound keyword not in engine keyword set
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Front face: "{1}{G}, {T}: You may put a creature card from your
            // hand onto the battlefield. ... Activate only as a sorcery."
            // GAP: "If it's a Wolf or Werewolf, untap this creature" rider omitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}, {T}: You may put a creature card from your hand onto the battlefield. If it's a Wolf or Werewolf, untap this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: put_creature_from_hand,
            })
            .with_transform_back(back)
            // Back face: "Whenever this creature enters or transforms into
            // Wildsong Howler, look at the top six cards of your library. You
            // may reveal a creature card from among them and put it into your
            // hand. Put the rest on the bottom of your library in a random
            // order." Two triggers (ETB + transforms-into), both back-face.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: dig_top_six_for_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: dig_top_six_for_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Both back-face triggers fire only while the back face is showing.
            .with_trigger_face_gate(1, 1)
            .with_trigger_face_gate(2, 1),
    )
}

fn dig_top_six_for_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Look at top six, may take a creature to hand, rest to bottom (random).
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(ObjectFilter::creature()),
        rest: DigRest::BottomRandom,
    }]
}

fn put_creature_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put a creature card from your hand onto the battlefield" —
    // optional pick over the controller's hand.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
