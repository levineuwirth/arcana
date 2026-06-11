//! Prowess of the Fair — `{1}{B}` Kindred Enchantment — Elf.
//! "Whenever another nontoken Elf is put into your graveyard from the
//! battlefield, you may create a 1/1 green Elf Warrior creature token."
//!
//! A graveyard-bound ZoneChange trigger on nontoken Elves you control.
//! Fidelity notes: the Kindred card type is not a TypeLine flag (the Elf
//! subtype is carried on the enchantment), and the "you may" choice is
//! resolved as always-create.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Prowess of the Fair");
    let elf = reg.interner_mut().intern("Elf");
    // Pre-intern the token's second subtype for the resolver lookup.
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf.clone());
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        // GAP: the "Kindred" card type is not a TypeLine flag; the Elf
        // subtype is retained.
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .with_subtypes_any(vec![elf])
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: mint_elf_warrior,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…you may create a 1/1 green Elf Warrior creature token."
fn mint_elf_warrior(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf.clone());
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: elf,
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
