//! Feywild Trickster — `{2}{U}` 2/2 blue Gnome Warlock.
//! `Whenever you roll one or more dice, create a 1/1 blue Faerie Dragon creature
//! token with flying.`
//!
//! GAP: trigger — "Whenever you roll one or more dice" has no matching
//! TriggerCondition variant in the engine. Using SelfEntersBattlefield as the
//! closest non-matching placeholder; the effect itself is correct but the trigger
//! condition will not fire from dice rolls.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feywild Trickster");
    let gnome = reg.interner_mut().intern("Gnome");
    let warlock = reg.interner_mut().intern("Warlock");
    // Pre-intern token subtypes for use at resolve time.
    let _faerie = reg.interner_mut().intern("Faerie");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger — "Whenever you roll one or more dice" is not a
            // supported TriggerCondition; no variant models dice-roll events.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_faerie_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_faerie_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let faerie = reg.interner().lookup("Faerie")
        .expect("Faerie interned during register()");
    let dragon = reg.interner().lookup("Dragon")
        .expect("Dragon interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(faerie);
    token_subtypes.0.insert(dragon);
    let token = TokenDefinition {
        name: faerie,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
