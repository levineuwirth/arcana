//! Acidic Soil — `{2}{R}` sorcery. "Acidic Soil deals damage to each
//! player equal to the number of lands they control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acidic Soil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Acidic Soil deals damage to each player equal to the number of lands they control.".into(),
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
    let mut out = Vec::new();
    for p in script::all_players(state) {
        let n = script::count_matching(
            state,
            &ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            p,
        );
        if n > 0 {
            out.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(p),
                amount: n,
            });
        }
    }
    out
}
