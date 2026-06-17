//! Spiteful Returned — `{1}{B}` 1/1 Enchantment Creature — Zombie (black).
//! "Bestow {3}{B}.
//!  Whenever this creature or enchanted creature attacks, defending player
//!  loses 2 life.
//!  Enchanted creature gets +1/+1."
//!
//! Bestow is not in the supported keyword surface → keywords empty, GAP'd
//! (the engine has no bestow-cast/aura-mode machinery exposed here). The
//! attack trigger is wired for the creature side (SelfAttacks → defending
//! player loses 2 life); the "or enchanted creature" and "+1/+1" halves are
//! aura-mode statics that depend on bestow and are GAP'd.

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
    let name = reg.interner_mut().intern("Spiteful Returned");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow {3}{B} — not a supported keyword.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: the "or enchanted creature attacks" half (bestow-aura side)
            // is not modeled; only the creature-itself attack is wired.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: defender_loses_2,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "Enchanted creature gets +1/+1" — bestow-aura static, not modeled.
    )
}

fn defender_loses_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: p,
        amount: 2,
    }]
}
