//! Thicket Basilisk — `{3}{G}{G}` 2/4 green Creature — Basilisk.
//! "Whenever this creature blocks or becomes blocked by a non-Wall creature,
//! destroy that creature at end of combat."
//! GAP: "destroy at end of combat" — DelayedWhen::EndOfCombat not available;
//! using DestroyPermanent immediately as best effort; also "non-Wall" subtype
//! exclusion filter not available.

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
    let name = reg.interner_mut().intern("Thicket Basilisk");
    let basilisk = reg.interner_mut().intern("Basilisk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basilisk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: blocks_destroy_combatant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blocks_destroy_combatant(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(other) = trig.other_combatant() else { return Vec::new(); };
    // GAP: "non-Wall" exclusion and "at end of combat" delay not modeled;
    // destroying immediately as approximation
    vec![Effect::DestroyPermanent { target: other }]
}
