//! Burning-Tree Shaman — `{1}{R}{G}` 3/4 red-green Creature — Centaur Shaman.
//! "Whenever a player activates an ability that isn't a mana ability, this
//! creature deals 1 damage to that player."
//! GAP: trigger — "player activates a non-mana ability" has no catalog
//! equivalent. Using SelfBecomesTapped as rough approximation.

use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Burning-Tree Shaman");
    let centaur = reg.interner_mut().intern("Centaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "player activates a non-mana ability" has
                // no catalog equivalent; SelfBecomesTapped used as rough
                // approximation.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: on_ability_activated_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_ability_activated_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DealDamage {
        target: DamageTarget::Player(trig.controller),
        amount: 1,
        source: trig.source,
    }]
}
