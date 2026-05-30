//! Yuan Shao's Infantry — `{3}{R}` 2/2 red Human Soldier.
//! "Whenever this creature attacks alone, this creature can't be blocked this combat."
//!
//! GAP: The trigger condition "attacks alone" (i.e., is the only attacking creature)
//! has no dedicated TriggerCondition variant. Using SelfAttacks as closest match.
//! The "this combat" scoping on CantBeBlocked is modeled as EndOfTurn (close approximation).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Yuan Shao's Infantry");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "attacks alone" (sole attacker) is not a
                // distinct TriggerCondition; using SelfAttacks as closest match.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: cant_be_blocked_this_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cant_be_blocked_this_combat(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}
