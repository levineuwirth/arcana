//! Judgment Bolt — `{3}{R}` instant. "Judgment Bolt deals 5 damage to
//! target creature and X damage to that creature's controller, where X
//! is the number of Equipment you control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Judgment Bolt");
    let _equipment = reg.interner_mut().intern("Equipment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Judgment Bolt deals 5 damage to target creature and X damage to that creature's controller, where X is the number of Equipment you control.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let controller = script::target_controller(state, id, entry.controller);
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Equipment"),
        entry.controller,
    );
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 5,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(controller),
            amount: x,
        },
    ]
}
