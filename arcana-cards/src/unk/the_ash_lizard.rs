//! The Ash Lizard — `{4}{R}{G}` 5/5 Legendary Lizard Warrior.
//! "Other Lizards, Treefolk, and Warriors you control get +2/+2."
//! "Whenever The Ash Lizard attacks, create a token that's a copy of one of the
//!  following cards at random: Ash Zealot; Bog-Strider Ash; Grotag Thrasher;
//!  Seedguide Ash; Unstoppable Ash; Viashino Warrior."
//!
//! The anthem is a pure static continuous effect (no trigger/cost) and is a GAP
//! for this card class. The attack trigger is wired as SelfAttacks, but its body
//! is a GAP — "create a token that's a copy of a named card at random" needs
//! registry-by-name token minting (CopyPermanent only copies an existing on-board
//! permanent), which is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Ash Lizard");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static anthem — "Other Lizards, Treefolk, and Warriors you control get
    // +2/+2" is a pure continuous static with no trigger/cost; not expressible in
    // this card class.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: random_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn random_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a token that's a copy of one of [named cards] at random."
    // CopyPermanent only copies an existing battlefield permanent; minting a token
    // copy of a card looked up by name from the registry is not expressible.
    Vec::new()
}
