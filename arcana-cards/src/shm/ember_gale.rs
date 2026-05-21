//! Ember Gale — `{3}{R}` sorcery. Creatures target player controls
//! can't block this turn. Deals 1 damage to each white and/or blue
//! creature that player controls.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ember Gale");
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
                text: "Creatures target player controls can't block this turn. Ember Gale deals 1 damage to each white and/or blue creature that player controls.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let p = *p;
    // GAP: "can't block this turn" rider — no catalog primitive.
    let white_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::white()),
        p,
    );
    let blue_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::blue()),
        p,
    );
    let mut effects = Vec::new();
    let mut seen = Vec::new();
    for id in white_ids.into_iter().chain(blue_ids.into_iter()) {
        if seen.contains(&id) {
            continue;
        }
        seen.push(id);
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 1,
        });
    }
    effects
}
