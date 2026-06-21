//! Knightfisher — `{3}{U}{U}` 4/5 Bird Knight.
//! Flying.
//! Whenever another nontoken Bird you control enters, create a 1/1 blue
//! Fish creature token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knightfisher");
    let bird = reg.interner_mut().intern("Bird");
    let knight = reg.interner_mut().intern("Knight");
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // "another nontoken Bird you control" — self-exclusion isn't
    // expressible on the filter, but the trigger is inert until this
    // creature is itself on the battlefield, so its own ETB won't fire.
    let bird_you_control = script::subtype_filter(reg, "Bird")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: bird_you_control,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_fish,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_fish(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let fish = reg.interner().lookup("Fish").unwrap_or_default();
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(fish);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: fish,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
