//! Nesting Dragon — `{3}{R}{R}` 5/4 Creature — Dragon.
//! Flying.
//! Landfall — Whenever a land you control enters, create a 0/2 red Dragon
//! Egg creature token with defender and "When this token dies, create a
//! 2/2 red Dragon creature token with flying and '{R}: This token gets
//! +1/+0 until end of turn.'"

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nesting Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    // Pre-intern the token subtypes used by the resolvers.
    let _egg = reg.interner_mut().intern("Dragon Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Landfall: "Whenever a land you control enters".
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: create_dragon_egg,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn create_dragon_egg(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let egg = reg.interner().lookup("Dragon Egg").unwrap_or_default();
    let mut egg_subtypes = SubtypeSet::default();
    egg_subtypes.0.insert(egg);

    // The Egg's death trigger: "When this token dies, create a 2/2 red Dragon
    // with flying and '{R}: …'." The inner Dragon's {R}: +1/+0 activated
    // ability is GAP'd — TokenDefinition.abilities only carries triggered
    // abilities, not activated ones.
    let egg_dies = TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfDies,
        intervening_if: None,
        effect: egg_dies_make_dragon,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };

    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: egg,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: egg_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Defender],
            abilities: vec![egg_dies],
        },
    }]
}

fn egg_dies_make_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    // GAP: the created Dragon's "{R}: This token gets +1/+0 until end of turn"
    // activated ability cannot live on a TokenDefinition (only triggered
    // abilities are supported on tokens).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
