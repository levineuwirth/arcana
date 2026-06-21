//! Mirror-Style Master — `{4}{R}{R}` 3/3 Human Warrior.
//!
//! * "Backup 1 (When this creature enters, put a +1/+1 counter on target
//!   creature. …)" — GAP: Backup is not a usable `KeywordAbility` variant,
//!   and its grant-an-ability rider is not expressible; `keywords: vec![]`.
//! * "Whenever this creature attacks, for each attacking modified creature
//!   you control, create a tapped and attacking token that's a copy of that
//!   creature. Exile those tokens at end of combat." — GAP: there is no
//!   filter for "modified" creatures, no per-attacker token-copy fan-out
//!   with a tapped-and-attacking copy + end-of-combat exile in the
//!   demonstrated surface. The attack trigger is wired with an empty effect.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Mirror-Style Master");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: copy_attacking_modified,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_attacking_modified(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no "modified creature" filter, no per-attacker tapped-and-attacking
    // token-copy fan-out, and no end-of-combat exile scheduling for the copies.
    Vec::new()
}
