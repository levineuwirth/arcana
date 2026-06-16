//! The Bringer of the Red Beard — `{1}{R}` Legendary 1/1 Human Warrior.
//!
//! "When The Bringer of the Red Beard enters, create a red Equipment
//! artifact token named Red Beard with 'Equipped creature is red' and
//! equip {1}." Plus two static buffs.
//!
//! Decomposition:
//! * ETB trigger → create a plain red Equipment artifact token. The
//!   token's granted static ("Equipped creature is red") and its
//!   equip {1} activated ability are not expressible as a
//!   `TokenDefinition` here (see GAPs in the resolver).
//! * "Red creatures you control get +1/+0." — pure static continuous
//!   anthem. GAP (no triggered/activated wrapper).
//! * "Equipped creatures you control get +1/+0." — pure static. GAP.

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
    let name = reg.interner_mut().intern("The Bringer of the Red Beard");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern the token's name + subtype for the resolver.
    let _red_beard = reg.interner_mut().intern("Red Beard");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "Red creatures you control get +1/+0." — pure continuous anthem, not a triggered/activated ability.
    // GAP: static "Equipped creatures you control get +1/+0." — pure continuous anthem, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_create_red_beard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: create a red Equipment artifact token named Red Beard.
fn etb_create_red_beard(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Red Beard").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: token's "Equipped creature is red" granted static + equip {1} activated
    // ability are not expressible via TokenDefinition; emit the bare Equipment token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::red(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
