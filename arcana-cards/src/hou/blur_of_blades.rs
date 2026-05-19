//! Blur of Blades — `{1}{R}` instant.
//! "Put a -1/-1 counter on target creature. Blur of Blades deals 2 damage to that creature's
//! controller."
//! GAP: 'that creature's controller' requires resolving the target's controller from state;
//! no script helper exposes a permanent's controller. Implementing -1/-1 counter only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blur of Blades");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a -1/-1 counter on target creature. Blur of Blades deals 2 damage to that creature's controller.".into(),
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
    // GAP: 2 damage to that creature's controller (controller lookup not available)
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
