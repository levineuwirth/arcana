//! Price of Progress — `{1}{R}` instant. "Price of Progress deals
//! damage to each player equal to twice the number of nonbasic lands
//! that player controls." Dynamic, per-player damage: count each
//! player's nonbasic lands and deal 2× that to them.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Price of Progress");
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
                text: "Price of Progress deals damage to each player equal to twice the number of nonbasic lands that player controls.".into(),
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
    let nonbasic = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .controlled_by(ControllerConstraint::You);
    script::all_players(state)
        .into_iter()
        .map(|p| {
            let n = script::count_matching(state, &nonbasic, p);
            Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(p),
                amount: n * 2,
            }
        })
        .collect()
}
