//! Mercadia's Downfall — `{2}{R}` instant. "Each attacking creature gets +1/+0
//! until end of turn for each nonbasic land defending player controls."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mercadia's Downfall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each attacking creature gets +1/+0 until end of turn for each nonbasic \
                       land defending player controls."
                    .into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .without_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    let attackers = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: attackers,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: n as i32,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
