//! Palani's Hatcher — `{3}{R}{G}` 5/3 Dinosaur (G/R).
//!
//! * Other Dinosaurs you control have haste. (static — GAP)
//! * When this creature enters, create two 0/1 green Dinosaur Egg
//!   creature tokens.
//! * At the beginning of combat on your turn, if you control one or
//!   more Eggs, sacrifice an Egg, then create a 3/3 green Dinosaur
//!   creature token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Palani's Hatcher");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: static "Other Dinosaurs you control have haste."
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_eggs,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_egg),
                effect: hatch_an_egg,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn egg_token(reg: &CardRegistry) -> TokenDefinition {
    let mut st = SubtypeSet::default();
    if let Some(d) = reg.interner().lookup("Dinosaur") {
        st.0.insert(d);
    }
    if let Some(e) = reg.interner().lookup("Egg") {
        st.0.insert(e);
    }
    let name = reg.interner().lookup("Egg").unwrap_or_default();
    TokenDefinition {
        name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn dino_token(reg: &CardRegistry) -> TokenDefinition {
    let mut st = SubtypeSet::default();
    if let Some(d) = reg.interner().lookup("Dinosaur") {
        st.0.insert(d);
    }
    let name = reg.interner().lookup("Dinosaur").unwrap_or_default();
    TokenDefinition {
        name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn etb_make_eggs(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: trig.controller, token: egg_token(reg) },
        Effect::CreateToken { controller: trig.controller, token: egg_token(reg) },
    ]
}

fn if_control_egg(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype_at_least(s, reg, you, "Egg", 1)
}

fn hatch_an_egg(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let egg_filter = script::subtype_filter(reg, "Egg").controlled_by(ControllerConstraint::You);
    vec![Effect::Sequence(vec![
        Effect::Sacrifice { player: trig.controller, filter: egg_filter, count: 1 },
        Effect::CreateToken { controller: trig.controller, token: dino_token(reg) },
    ])]
}
