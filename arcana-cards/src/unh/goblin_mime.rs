//! Goblin Mime — `{1}{R}` 2/2 red Goblin Mime. "When you speak,
//! sacrifice this creature." An Un-set card whose trigger is a
//! real-world event ("you speak") with no game-state representation.
//! The effect (sacrifice this creature) is expressible; the trigger
//! is not, so it is GAP-ed onto the closest available condition.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Mime");
    let goblin = reg.interner_mut().intern("Goblin");
    let mime = reg.interner_mut().intern("Mime");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mime);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger — "When you speak" is an out-of-game (Un-set)
            // event with no TriggerCondition representation; mapped onto
            // the closest available condition as a placeholder.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Sacrifice this creature." The controller sacrifices one creature
/// (Goblin Mime sacrifices itself).
fn sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        count: 1,
    }]
}
