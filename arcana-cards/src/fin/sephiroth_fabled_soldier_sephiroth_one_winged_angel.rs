//! Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel
//!
//! Front: Legendary Creature — Human Avatar Soldier, {2}{B}, 3/3.
//! Whenever Sephiroth enters or attacks, you may sacrifice another creature. If you do, draw a card.
//! Whenever another creature dies, target opponent loses 1 life and you gain 1 life.
//! If this is the fourth time this ability has resolved this turn, transform Sephiroth.
//!
//! Back: Legendary Creature — Angel Nightmare Avatar.
//! Flying.
//! Super Nova — As this creature transforms into Sephiroth, One-Winged Angel, you get an emblem
//! with "Whenever a creature dies, target opponent loses 1 life and you gain 1 life." (GAP: emblems
//! not modeled.)
//! Whenever Sephiroth attacks, you may sacrifice any number of other creatures. If you do, draw
//! that many cards.
//!
//! GAP: "Super Nova" — emblem creation not modeled.
//! GAP: "you may sacrifice another creature. If you do, draw a card" — OptionalPaymentKind has no
//! Sacrifice variant; modeled as plain draw (sacrifice not enforced).
//! GAP: "If this is the fourth time this ability has resolved this turn" — per-ability-resolution
//! count tracking not available; transform fires on every resolution (overfires).
//! GAP: "Whenever Sephiroth attacks, you may sacrifice any number of other creatures. If you do,
//! draw that many cards." — back-face-only triggered ability not modeled.
//! GAP: "Whenever Sephiroth enters" — SelfEntersBattlefield used as separate trigger (correct);
//! combined "enters or attacks" modeled as two separate triggers.
//! Keywords: Super Nova not in engine keyword list; not emitted.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sephiroth, Fabled SOLDIER");

    let human_sub = reg.interner_mut().intern("Human");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(avatar_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Sephiroth, One-Winged Angel");
    let angel_sub = reg.interner_mut().intern("Angel");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let back_avatar_sub = reg.interner_mut().intern("Avatar");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(angel_sub);
    back_subtypes.0.insert(nightmare_sub);
    back_subtypes.0.insert(back_avatar_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: Sephiroth enters the battlefield — draw (GAP: may sacrifice for draw).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: Sephiroth attacks — draw (GAP: may sacrifice for draw).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3: whenever another creature dies, each opponent loses 1 life, you gain 1.
            // GAP: fourth-time tracking not modeled; transform fires every time (overfires).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Any),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: creature_dies_drain_and_maybe_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability not modeled (Sephiroth attacks, sac N, draw N).
    )
}

fn etb_draw(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature. If you do, draw a card" — sacrifice gate not
    // enforceable; draw unconditionally as best-effort.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn creature_dies_drain_and_maybe_transform(
    state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opponents
        .into_iter()
        .map(|opp| Effect::LoseLife {
            player: opp,
            amount: 1,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    // GAP: "If this is the fourth time this ability has resolved this turn" — per-resolution
    // count not tracked; transform fires every time (overfires).
    effects.push(Effect::Transform {
        target: trig.source,
    });
    effects
}
