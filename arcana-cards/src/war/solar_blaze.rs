//! Solar Blaze — `{2}{R}{W}` sorcery. "Each creature deals damage to
//! itself equal to its power."

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
    let name = reg.interner_mut().intern("Solar Blaze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each creature deals damage to itself equal to its \
                   power."
                .into(),
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
    // Per-creature damage scales on that creature's own power, so this
    // can't use ForEach (single fixed inner effect); emit one
    // DealDamage per id with its individually computed power.
    let ids =
        script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut out = Vec::new();
    for id in ids {
        let amount = script::power_of(state, id).max(0) as u32;
        out.push(Effect::DealDamage {
            source: id,
            target: DamageTarget::Object(id),
            amount,
        });
    }
    out
}
