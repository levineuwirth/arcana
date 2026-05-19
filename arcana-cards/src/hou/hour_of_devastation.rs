//! Hour of Devastation — `{3}{R}{R}` sorcery.
//! "All creatures lose indestructible until end of turn. Hour of Devastation
//! deals 5 damage to each creature and each non-Bolas planeswalker."
//!
//! GAP: "lose indestructible until end of turn" layer effect not expressible.
//! GAP: dealing damage to planeswalkers (no planeswalker filter in ObjectFilter).
//! The 5-damage-to-each-creature board damage is expressed.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hour of Devastation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "All creatures lose indestructible until end of turn. Hour of Devastation deals 5 damage to each creature and each non-Bolas planeswalker.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: "lose indestructible" layer effect not expressible
    // GAP: planeswalker damage not expressible (no planeswalker filter)
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(arcana_core::objects::NULL_OBJECT_ID),
            amount: 5,
        }),
    }]
}
