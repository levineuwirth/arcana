//! Invoke Justice — `{W}{W}{W}{W}{W}` sorcery, "Return target permanent card
//! from your graveyard to the battlefield. Put four +1/+1 counters on a
//! creature or Vehicle you control."
//!
//! # GAP
//! GAP: ReturnFromGraveyardToBattlefield targets a creature card specifically;
//! spec targets any permanent card. AddCounters on Vehicle subtype not
//! separately filterable — best effort applies counters to target creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invoke Justice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent card from your graveyard to the battlefield. Put four +1/+1 counters on a creature or Vehicle you control.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: reanimate target is creature only; spec allows any permanent card
    // GAP: AddCounters on a creature or Vehicle you control (second untargeted effect)
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 4,
        },
    ]
}
