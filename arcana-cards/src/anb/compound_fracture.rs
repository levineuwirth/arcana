//! Compound Fracture — `{B}` instant. "Target creature gets -1/-1
//! until end of turn. It gets an additional -1/-1 until end of turn
//! for each card named Compound Fracture in your graveyard."
//!
//! The base -1/-1 plus an additional -1/-1 per copy of this card in
//! your graveyard. The graveyard count is dynamic, computed via
//! `script::graveyard_matching` against a name filter.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Compound Fracture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -1/-1 until end of turn. It gets an additional -1/-1 until end of turn for each card named Compound Fracture in your graveyard.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let nm = reg.interner().lookup("Compound Fracture");
    let extra = script::graveyard_matching(
        state,
        &ObjectFilter { name: nm, ..ObjectFilter::default() },
        entry.controller,
        entry.controller,
    );
    let total = 1 + extra as i32;
    vec![Effect::Pump {
        target: *id,
        power: -total,
        toughness: -total,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
