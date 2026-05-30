//! Brightcap Badger // Fungus Frolic — `{3}{G}` Badger Druid creature 3/4 (front face).
//! "Each Fungus and Saproling you control has '{T}: Add {G}.'"
//! "At the beginning of your end step, create a 1/1 green Saproling creature token."
//! Adventure face "Fungus Frolic" (`{2}{G}` instant): create two 1/1 green Saproling tokens.
//!
//! # GAPs
//! - "Each Fungus and Saproling you control has '{T}: Add {G}.'" — granting activated
//!   abilities to permanents you control is a static/layer effect not in the Effect catalog.
//!   GAP: omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brightcap Badger");
    let badger_sub = reg.interner_mut().intern("Badger");
    let druid_sub = reg.interner_mut().intern("Druid");
    let _ = reg.interner_mut().intern("Saproling");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger_sub);
    subtypes.0.insert(druid_sub);

    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Fungus Frolic");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create two 1/1 green Saproling creature tokens.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: fungus_frolic_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_create_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_create_saproling(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling_name = reg.interner().lookup("Saproling").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(saproling_name);
    let token = TokenDefinition {
        name: saproling_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        keywords: vec![],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn fungus_frolic_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling_name = reg.interner().lookup("Saproling").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(saproling_name);
    let token = TokenDefinition {
        name: saproling_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        keywords: vec![],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
    ]
}
