//! The Covert Blue Mage — `{2}{U}` 1/1 Legendary Creature — Human Rogue.
//! If The Covert Blue Mage is your commander, your starting deck can contain
//! any number of cards beyond the maximum deck size.
//! When The Covert Blue Mage enters, create a colorless Equipment artifact
//! token named Covert Sunglasses. It has "Equipped creature can't be blocked"
//! and equip {2}{W/U}.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Covert Blue Mage");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let _equipment = reg.interner_mut().intern("Equipment");
    let _token_name = reg.interner_mut().intern("Covert Sunglasses");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "If ~ is your commander, your starting deck can contain any number
    // of cards beyond the maximum deck size." is a deck-construction rule with
    // no in-game effect; not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_equipment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: the Equipment's "Equipped creature can't be blocked" granted static
    // and its equip {2}{W/U} activated ability cannot live on a TokenDefinition
    // (only triggered abilities are supported on tokens). The bare colorless
    // Equipment artifact token is minted faithfully.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: equipment,
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
