//! Dry Spell — `{1}{B}` sorcery. "Dry Spell deals 1 damage to each creature and each player."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dry Spell");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Dry Spell deals 1 damage to each creature and each player.".into(),
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
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects: Vec<Effect> = vec![
        Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        },
    ];
    // GAP: "each player" — no way to enumerate all players; deal to controller only as best effort
    effects.push(Effect::LoseLife { player: entry.controller, amount: 1 });
    effects
}
