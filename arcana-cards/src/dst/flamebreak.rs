//! Flamebreak — `{R}{R}{R}` sorcery. "Flamebreak deals 3 damage to each
//! creature without flying and each player. Creatures dealt damage this
//! way can't be regenerated this turn." GAPs: 'without flying' (the
//! ObjectFilter surface has no `.without_keyword`) and the
//! 'can't-be-regenerated' rider.

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
    let name = reg.interner_mut().intern("Flamebreak");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flamebreak deals 3 damage to each creature without flying and each player. Creatures dealt damage this way can't be regenerated this turn.".into(),
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
    // GAP: 'without flying' filter not expressible; falling back to all
    // creatures, plus GAP for 'can't be regenerated this turn'.
    let mut effects = vec![Effect::ForEach {
        targets: script::ids_matching(
            state,
            &ObjectFilter::creature(),
            entry.controller,
        ),
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 3,
        }),
    }];
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 3,
        });
    }
    effects
}
