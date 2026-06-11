//! Seed the Land — `{2}{G}{G}` enchantment (Saviors of Kamigawa, 2005).
//! "Whenever a land enters, its controller creates a 1/1 green Snake
//! creature token."
//!
//! A battlefield-bound `ZoneChange` trigger over ALL lands; "its
//! controller" is read off the entering object via
//! `script::target_controller`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seed the Land");
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into()),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: snake_for_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…its controller creates a 1/1 green Snake creature token."
fn snake_for_controller(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(land) = trig.entering_object() else {
        return Vec::new();
    };
    let controller = script::target_controller(state, land, trig.controller);
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    vec![Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: snake,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
