//! Dragonfly Swarm — `{1}{U}{R}` */3 Dragon Insect.
//! Flying, ward {1}.
//! Its power is equal to the number of noncreature, nonland cards in your
//! graveyard. When this creature dies, if there's a Lesson card in your
//! graveyard, draw a card.
//!
//! Flying and Ward {1} are base keywords. The CDA (power = noncreature,
//! nonland cards in your graveyard, toughness = the printed fixed 3) is wired
//! at Layer 7a via a SelfEntersBattlefield self_pt_cda returning `(n, 3)`;
//! bones are `*`/`Fixed(3)`. The dies-trigger draw is gated by an
//! intervening-if (Lesson card in your graveyard).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonfly Swarm");
    let dragon = reg.interner_mut().intern("Dragon");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA: power = noncreature, nonland cards in your graveyard; tough 3.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: Some(if_lesson_in_graveyard),
                effect: dies_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: power = noncreature, nonland cards in your graveyard;
/// toughness is the printed fixed 3.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            noncreature_nonland_gy_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = noncreature, nonland cards in your graveyard; toughness fixed 3.
fn noncreature_nonland_gy_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter =
        ObjectFilter::new().without_types(TypeLine(TypeLine::CREATURE | TypeLine::LAND));
    let n = script::graveyard_matching(s, &filter, who, who) as i32;
    (n, 3)
}

fn if_lesson_in_graveyard(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    conditions::graveyard_has_subtype(s, reg, you, "Lesson")
}

fn dies_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
