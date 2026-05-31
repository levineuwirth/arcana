//! Undying Malice — `{B}` instant. "Until end of turn, target creature
//! gains \"When this creature dies, return it to the battlefield tapped
//! under its owner's control with a +1/+1 counter on it.\""
//!
//! Implemented via [`Effect::GrantTriggeredAbility`]: the resolver builds a
//! `SelfDies` triggered ability (id in the `GRANTED_TRIGGER_ID_BASE` range)
//! whose effect returns the dead creature with a +1/+1 counter
//! (`Effect::ReturnFromGraveyardWithCounters`, the Undying primitive). The
//! grant lasts until end of turn.
//!
//! GAP: the engine's return primitive places the creature untapped; the
//! "tapped" rider of the printed ability is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undying Malice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, target creature gains \"When this creature \
                   dies, return it to the battlefield tapped under its owner's \
                   control with a +1/+1 counter on it.\""
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let ability = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::SelfDies,
        intervening_if: None,
        effect: granted_dies_return,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    vec![Effect::GrantTriggeredAbility {
        target: *id,
        ability: Box::new(ability),
        duration: arcana_core::layers::Duration::EndOfTurn,
    }]
}

fn granted_dies_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardWithCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
