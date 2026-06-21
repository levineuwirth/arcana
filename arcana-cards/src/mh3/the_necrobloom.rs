//! The Necrobloom — `{1}{W}{B}{G}` 2/7 Legendary Plant.
//! "Landfall — Whenever a land you control enters, create a 0/1 green Plant
//!  creature token. If you control seven or more lands with different
//!  names, create a 2/2 black Zombie creature token instead.
//!  Land cards in your graveyard have dredge 2."
//!
//! Landfall is an ability word, not a KeywordAbility, so the keyword vec is
//! empty. The landfall trigger is wired as a land-ETB ZoneChange and mints
//! the base 0/1 green Plant. The "if you control seven or more lands with
//! different names, create a 2/2 black Zombie instead" upgrade needs a
//! distinct-names count with no demonstrated helper, so it is GAP'd. The
//! dredge static on graveyard land cards has no demonstrated hook — GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Necrobloom");
    let plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    // Pre-intern the token subtype so the resolver can rebuild it.
    let _plant_token = reg.interner_mut().intern("Plant");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    // GAP (static): "Land cards in your graveyard have dredge 2" — no
    // demonstrated grant-keyword-to-graveyard-cards hook.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: landfall_make_plant,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn landfall_make_plant(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you control seven or more lands with different names, create
    // a 2/2 black Zombie instead" — no distinct-names count helper; always
    // mints the base 0/1 green Plant token.
    let plant = reg.interner().lookup("Plant").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: plant,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
