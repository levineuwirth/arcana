//! Villainous Wrath — `{3}{B}{B}` sorcery, "Target opponent loses life
//! equal to the number of creatures they control. Then destroy all
//! creatures."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;
use arcana_core::targets::ObjectFilter;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Villainous Wrath");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent loses life equal to the number of creatures they control. Then destroy all creatures.".into(),
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
    let TargetChoice::Player(opponent) = target else { return Vec::new(); };
    let count = script::count_matching(state, &ObjectFilter::creature(), *opponent);
    let all_creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![
        Effect::LoseLife { player: *opponent, amount: count },
        Effect::ForEach {
            targets: all_creatures,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
    ]
}
