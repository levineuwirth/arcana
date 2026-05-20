//! Destructive Force — `{5}{R}{R}` sorcery. "Each player sacrifices five
//! lands of their choice. Destructive Force deals 5 damage to each
//! creature."
//!
//! Uses `script::all_players` to sacrifice per player, then ForEach over
//! every creature for the 5 damage sweep.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Destructive Force");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player sacrifices five lands of their choice. Destructive Force deals 5 damage to each creature.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effs: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            count: 5,
        })
        .collect();
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    effs.push(Effect::ForEach {
        targets: creatures,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 5,
        }),
    });
    effs
}
