//! Burakos, Party Leader — `{3}{B}` 2/4 Legendary Orc.
//! Burakos is also a Cleric, Rogue, Warrior, and Wizard.
//! Whenever Burakos attacks, defending player loses X life and you create
//! X Treasure tokens, where X is the number of creatures in your party.
//! Choose a Background.
//!
//! The "is also a Cleric, Rogue, Warrior, and Wizard" static is modeled
//! as printed base subtypes (it has no other expression). The attack
//! trigger is GAP'd: "X = creatures in your party" has no script helper
//! (party = up to one each of Cleric/Rogue/Warrior/Wizard), so the
//! dynamic amount cannot be computed. "Choose a Background" is a
//! deck-construction keyword with no in-game effect to express.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Burakos, Party Leader");
    let orc = reg.interner_mut().intern("Orc");
    let cleric = reg.interner_mut().intern("Cleric");
    let rogue = reg.interner_mut().intern("Rogue");
    let warrior = reg.interner_mut().intern("Warrior");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    // "Burakos is also a Cleric, Rogue, Warrior, and Wizard."
    subtypes.0.insert(cleric);
    subtypes.0.insert(rogue);
    subtypes.0.insert(warrior);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "defending player loses X life and you create X Treasure
    // tokens, where X is the number of creatures in your party." There is
    // no script helper for party size (one each of Cleric/Rogue/Warrior/
    // Wizard), so the dynamic X cannot be computed.
    Vec::new()
}
