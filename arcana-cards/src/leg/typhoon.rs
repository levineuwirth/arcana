//! Typhoon — `{2}{G}` sorcery. "Typhoon deals damage to each opponent
//! equal to the number of Islands that player controls." Per-opponent
//! variable damage where N depends on that opponent's Islands. We use
//! subtype_filter('Island') and the You/Opponent controller scoping;
//! in 1v1 this is exact (one opponent counted).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Typhoon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: per-opponent island-count precision in multiplayer (uses Opponent aggregate).
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Typhoon deals damage to each opponent equal to the number of Islands that player controls.".into(),
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
    let filter = script::subtype_filter(reg, "Island")
        .controlled_by(ControllerConstraint::Opponent);
    let n = script::count_matching(state, &filter, entry.controller);
    let mut effects = Vec::new();
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(opp),
            amount: n,
        });
    }
    effects
}
