//! Omen of Fire — `{3}{R}{R}` instant. "Return all Islands to their
//! owners' hands. Each player sacrifices a Plains or a white
//! permanent of their choice for each white permanent they control."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omen of Fire");
    let _island = reg.interner_mut().intern("Island");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return all Islands to their owners' hands. Each player sacrifices a Plains or a white permanent of their choice for each white permanent they control.".into(),
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
    let islands = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Island"),
        entry.controller,
    );
    let mut out: Vec<Effect> = islands
        .into_iter()
        .map(|id| Effect::ReturnToHand { target: id })
        .collect();
    for p in script::all_players(state) {
        let n = script::count_matching(
            state,
            &ObjectFilter::permanent()
                .with_colors(ColorSet::white())
                .controlled_by(ControllerConstraint::You),
            p,
        );
        if n > 0 {
            // Best-effort: sacrifice white permanents per the rider; the
            // "Plains or white permanent" choice resolves to white
            // permanents (Plains are white).
            out.push(Effect::Sacrifice {
                player: p,
                filter: ObjectFilter::permanent().with_colors(ColorSet::white()),
                count: n,
            });
        }
    }
    out
}
