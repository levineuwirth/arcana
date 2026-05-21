//! End the Festivities — `{R}` sorcery. "End the Festivities deals 1
//! damage to each opponent and each creature and planeswalker they
//! control."

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
    let name = reg.interner_mut().intern("End the Festivities");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "End the Festivities deals 1 damage to each opponent and each creature and planeswalker they control.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for p in script::opponents(state, entry.controller) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
        // Creatures and planeswalkers that opponent controls — evaluated
        // with ControllerConstraint::You against the opponent's id.
        let filter = ObjectFilter::permanent()
            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))
            .controlled_by(ControllerConstraint::You);
        for id in script::ids_matching(state, &filter, p) {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(id),
                amount: 1,
            });
        }
    }
    effects
}
