//! The Meep — `{2}{B}` 0/4 Legendary Alien.
//! "Ward—Pay 3 life." and "Whenever The Meep attacks, you may sacrifice
//! another creature. If you do, creatures you control have base power and
//! toughness X/X until end of turn, where X is the sacrificed creature's mana
//! value."

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
    let name = reg.interner_mut().intern("The Meep");
    let alien = reg.interner_mut().intern("Alien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Ward—Pay 3 life" — non-mana Ward cost is not an expressible
        // KeywordAbility::Ward (only mana-cost Ward is supported).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: meep_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn meep_attack(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature. If you do, creatures you
    // control have base P/T X/X until end of turn, where X is the sacrificed
    // creature's mana value." The X derives from the sacrificed creature's
    // mana value chosen mid-resolution and must feed a board-wide SetBasePT —
    // not expressible with the available optional-sacrifice / dynamic-X surface.
    Vec::new()
}
