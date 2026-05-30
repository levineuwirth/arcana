//! Goldbug, Humanity's Ally // Goldbug, Scrappy Scout
//!
//! Front: Legendary Artifact Creature — Robot {1}{W}{U}, 3/3
//!   More Than Meets the Eye {W}{U} (alternate cast cost — GAP: not modeled as keyword)
//!   Prevent all combat damage that would be dealt to attacking Humans you control.
//!     (GAP: static damage prevention not modeled)
//!   Whenever you cast your second spell each turn, convert Goldbug (transform).
//!
//! Back: Legendary Artifact — Vehicle (Goldbug, Scrappy Scout)
//!   Living metal (During your turn, this Vehicle is also a creature.) (GAP: not modeled)
//!   Human spells you control can't be countered. (GAP: static counterspell protection not modeled)
//!   Whenever Goldbug and at least one Human attack, draw a card and convert Goldbug.
//!     (GAP: "at least one Human attacks alongside" condition not fully modeled; wired as SelfAttacks,
//!      actual Human co-attacker check is omitted)
//!
//! Keywords: More Than Meets the Eye, Living Metal, Convert are not in the engine keyword surface —
//!   omitted from keywords vec.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goldbug, Humanity's Ally");
    let back_name = reg.interner_mut().intern("Goldbug, Scrappy Scout");

    let mut front_subtypes = SubtypeSet::default();
    let robot_sub = reg.interner_mut().intern("Robot");
    front_subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        keywords: vec![],
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    back_subtypes.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::ARTIFACT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: whenever you cast your second spell each turn, convert (transform).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: second_spell_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (shared): whenever Goldbug attacks (and at least one Human attacks), draw a card and convert.
            // GAP: "at least one Human attacks alongside" condition not modeled; fires on any attack.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_draw_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn second_spell_transform(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new();
    let count = script::spells_cast_this_turn(state, &filter, trig.controller);
    if count == 2 {
        vec![Effect::Transform { target: trig.source }]
    } else {
        Vec::new()
    }
}

fn attack_draw_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Transform { target: trig.source },
    ]
}
