//! Fire Tempest — `{5}{R}{R}` sorcery. "Fire Tempest deals 6 damage
//! to each creature and each player."

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
    let name = reg.interner_mut().intern("Fire Tempest");
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
                text: "Fire Tempest deals 6 damage to each creature and each player.".into(),
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
    let mut effects = Vec::new();
    for id in script::ids_matching(state, &ObjectFilter::creature(), entry.controller) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 6,
        });
    }
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 6,
        });
    }
    effects
}
