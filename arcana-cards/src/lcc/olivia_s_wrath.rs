//! Olivia's Wrath — `{4}{B}` sorcery. Each non-Vampire creature gets
//! -X/-X until end of turn, X = Vampires you control.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Olivia's Wrath");
    let _vampire = reg.interner_mut().intern("Vampire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each non-Vampire creature gets -X/-X until end of turn, where X is the number of Vampires you control.".into(),
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
    let vampire_filter = script::subtype_filter(reg, "Vampire")
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &vampire_filter, entry.controller) as i32;
    // 'Each non-Vampire creature' — subtype exclusion ("Vampire" interned
    // in register; on a failed lookup skip the exclusion).
    let mut non_vampire = ObjectFilter::creature();
    if let Some(vampire) = reg.interner().lookup("Vampire") {
        non_vampire = non_vampire.without_subtype_sym(vampire);
    }
    let ids = script::ids_matching(
        state,
        &non_vampire,
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -x,
            toughness: -x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
