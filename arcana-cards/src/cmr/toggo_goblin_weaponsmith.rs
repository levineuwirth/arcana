//! Toggo, Goblin Weaponsmith — `{2}{R}` 2/2 Legendary Goblin Artificer.
//! Landfall — Whenever a land you control enters, create a colorless Equipment
//! artifact token named Rock with "Equipped creature has '{1}, {T}, Sacrifice
//! Rock: This creature deals 2 damage to any target'" and equip {1}.
//! Partner.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toggo, Goblin Weaponsmith");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    // Pre-intern the Rock token's name and Equipment subtype.
    let _rock = reg.interner_mut().intern("Rock");
    let _equipment = reg.interner_mut().intern("Equipment");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // Partner / Landfall (Scryfall keywords) are not usable keyword variants;
        // Landfall is the templating word for the trigger implemented below.
        keywords: vec![],
        ..Default::default()
    };

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
            effect: make_rock,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_rock(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rock = reg.interner().lookup("Rock").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP (partial): the Rock token's granted ability ("Equipped creature has
    // '{1}, {T}, Sacrifice Rock: deal 2 damage to any target'") and its equip {1}
    // ability are not expressible — TokenDefinition.abilities only carries
    // triggered abilities, not granted activated abilities or equip costs. The
    // colorless Equipment artifact token itself is minted faithfully.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: rock,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
