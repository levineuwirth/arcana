//! Slavering Nulls — `{1}{R}` 2/1 red Creature — Goblin Zombie.
//! "Whenever this creature deals combat damage to a player, if you control a
//! Swamp, you may have that player discard a card."
//! Intervening-if "if you control a Swamp" — modeled as None, checked at
//! resolution via script.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slavering Nulls");
    let goblin = reg.interner_mut().intern("Goblin");
    let zombie = reg.interner_mut().intern("Zombie");
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_combat_damage_discard(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Check intervening-if: "if you control a Swamp".
    let swamp_filter = script::subtype_filter(reg, "Swamp")
        .with_types(TypeLine::LAND.into());
    let swamp_count = script::count_matching(state, &swamp_filter, trig.controller);
    if swamp_count == 0 {
        return Vec::new();
    }
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    vec![Effect::Discard {
        player: p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
