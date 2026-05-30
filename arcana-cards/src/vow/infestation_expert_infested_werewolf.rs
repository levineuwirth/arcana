//! Infestation Expert // Infested Werewolf — `{4}{G}` Creature — Human Werewolf // Werewolf 3/4
//!
//! Front: Whenever this creature enters or attacks, create a 1/1 green Insect creature token.
//! Daybound (GAP: Daybound/Nightbound day-night cycle not modeled; transform wired as a
//! manual triggered ability).
//!
//! Back (Infested Werewolf): Whenever this creature enters or attacks, create two 1/1 green
//! Insect creature tokens.
//! Nightbound (GAP: back-face-only triggered ability not modeled).
//! GAP: back-face-only "enters or attacks → two tokens" trigger not modeled.
//! GAP: precise Daybound/Nightbound day-night cycle conditions not modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infestation Expert");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    // Pre-intern Insect so resolvers can look it up
    let _insect_sub = reg.interner_mut().intern("Insect");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Infested Werewolf");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: whenever this creature enters the battlefield, create a 1/1 green Insect token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_one_insect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front: whenever this creature attacks, create a 1/1 green Insect token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_one_insect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn insect_token(reg: &CardRegistry) -> TokenDefinition {
    let insect_sub = reg.interner().lookup("Insect");
    let mut token_subtypes = SubtypeSet::default();
    if let Some(s) = insect_sub {
        token_subtypes.0.insert(s);
    }
    TokenDefinition {
        name: reg.interner().lookup("Insect").unwrap_or_default(),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn create_one_insect(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: insect_token(reg),
    }]
}
