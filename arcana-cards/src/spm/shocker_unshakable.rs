//! Shocker, Unshakable — `{4}{R}{R}` 5/5 Legendary Human Rogue Villain.
//! "During your turn, Shocker has first strike."
//! "Vibro-Shock Gauntlets — When Shocker enters, he deals 2 damage to
//!  target creature and 2 damage to that creature's controller."
//!
//! The conditional "during your turn, has first strike" static is not
//! expressible (GAP). The ETB damage ability is a targeted trigger.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shocker, Unshakable");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(villain);

    // GAP: "During your turn, Shocker has first strike." — a conditional
    // turn-gated continuous keyword grant on itself is not expressible
    // with the available primitives.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: vibro_shock,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn vibro_shock(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, trig.controller);
    vec![
        Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(*id),
            amount: 2,
        },
        Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(controller),
            amount: 2,
        },
    ]
}
