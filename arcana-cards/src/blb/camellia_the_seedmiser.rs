//! Camellia, the Seedmiser — `{1}{B}{G}` 3/3 Legendary Squirrel Warlock.
//!
//! Menace
//! "Other Squirrels you control have menace." (static — not expressible; GAP'd.)
//! "Whenever you sacrifice one or more Foods, create a 1/1 green Squirrel
//!  creature token." (expressed below.)
//! "{2}, Forage: Put a +1/+1 counter on each other Squirrel you control."
//!  (Forage is not an available activation cost field; GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Camellia, the Seedmiser");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warlock);

    let food_filter: ObjectFilter = script::subtype_filter(reg, "Food");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: static — "Other Squirrels you control have menace."
    // GAP: activated — "{2}, Forage: …" (Forage is not an ActivationCost field).

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed { filter: food_filter },
            intervening_if: None,
            effect: make_squirrel,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_squirrel(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: squirrel,
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
