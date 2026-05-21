//! Thunder of Hooves — `{3}{R}` sorcery. "Thunder of Hooves deals X
//! damage to each creature without flying and each player, where X
//! is the number of Beasts on the battlefield." X is the count of
//! Beast permanents on the battlefield.

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
    let name = reg.interner_mut().intern("Thunder of Hooves");
    let _beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Thunder of Hooves deals X damage to each creature without flying and each player, where X is the number of Beasts on the battlefield.".into(),
                target_requirements: vec![],
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
    let beasts = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Beast"),
        entry.controller,
    );
    let x = beasts.len() as u32;
    let mut effects = Vec::new();
    // GAP: 'without flying' creature filter is not expressible in
    // ObjectFilter — damage hits every creature.
    for id in script::ids_matching(state, &ObjectFilter::creature(), entry.controller) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: x,
        });
    }
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: x,
        });
    }
    effects
}
