//! Skewer Slinger — `{1}{R}` 1/3 red Dwarf Knight.
//! Reach.
//! Whenever this creature blocks or becomes blocked by a creature, this
//! creature deals 1 damage to that creature.
//!
//! Abilities:
//!  - Reach → `KeywordAbility::Reach`.
//!  - SelfBlocksOrBecomesBlocked trigger → deal 1 damage to the OTHER
//!    combatant (read via `trig.other_combatant()`).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Skewer Slinger");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
            intervening_if: None,
            effect: ping_other_combatant,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn ping_other_combatant(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(id),
        amount: 1,
    }]
}
