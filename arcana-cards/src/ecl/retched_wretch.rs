//! Retched Wretch — `{2}{B}` 4/2 Creature — Goblin.
//! "When this creature dies, if it had a -1/-1 counter on it, return it to
//! the battlefield under its owner's control and it loses all abilities."
//! GAP: intervening-if check (had -1/-1 counter) not expressible — trigger fires
//! unconditionally and always returns to battlefield.
//! GAP: "loses all abilities" modifier on ETB return is not expressible.

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
    let name = reg.interner_mut().intern("Retched Wretch");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                // GAP: intervening-if "if it had a -1/-1 counter" not modeled — fires always
                intervening_if: None,
                effect: dies_return_to_battlefield,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_return_to_battlefield(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    // GAP: "loses all abilities" on the returned permanent is not expressible
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
