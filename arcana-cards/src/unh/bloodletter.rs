//! Bloodletter — `{2}{B}` 2/3 Zombie. "When the names of three or
//! more nonland permanents begin with the same letter, sacrifice this
//! creature. If you do, it deals 2 damage to each creature and each
//! player."
//!
//! The trigger condition (a board-wide scan of permanent name initials)
//! has no matching `TriggerCondition` variant — it is GAP'd to the
//! closest unit trigger. The resolution effect (sacrifice self, then
//! deal 2 to each creature and each player) is fully expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodletter");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "when the names of three or more nonland
                // permanents begin with the same letter" is a board-wide
                // name-initial scan with no matching TriggerCondition
                // variant; using the closest unit trigger as a placeholder.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sacrifice_and_blast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_and_blast(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    // Sacrifice this creature.
    effects.push(Effect::ForEach {
        targets: vec![trig.source],
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    });
    // It deals 2 damage to each creature.
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    for id in creatures {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(id),
            amount: 2,
        });
    }
    // ...and each player.
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 2,
        });
    }
    effects
}
