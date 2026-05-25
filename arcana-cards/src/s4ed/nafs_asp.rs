//! Nafs Asp — `{G}` 1/1 green Creature — Snake.
//! "Whenever this creature deals damage to a player, that player loses
//! 1 life at the beginning of their next draw step unless they pay {1}
//! before that draw step."
//! GAP: delayed conditional "unless opponent pays {1}" effect not expressible;
//! emitting LoseLife 1 to the damaged player as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nafs Asp");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_damage_to_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_damage_to_player(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: delayed "next draw step unless pay {1}" conditional not expressible;
    // emitting immediate LoseLife 1 as approximation
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    vec![Effect::LoseLife { player: p, amount: 1 }]
}
