//! Defile — `{B}` instant, "Target creature gets -1/-1 until end of turn for
//! each Swamp you control." The amount is computed from the number of Swamps
//! (a land subtype) controlled — use `script::ids_matching` with a Swamp
//! subtype filter to count them.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Defile");
    let _swamp = reg.interner_mut().intern("Swamp");
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
                text: "Target creature gets -1/-1 until end of turn for each Swamp you control.".into(),
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
    let swamp_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    // Count lands named/typed Swamp; use subtype_filter for the Swamp subtype
    let swamp_subtype_filter = script::subtype_filter(reg, "Swamp")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &swamp_subtype_filter, entry.controller);
    if n == 0 {
        return Vec::new();
    }
    let _ = swamp_filter;
    vec![Effect::Pump {
        target: *id,
        power: -(n as i32),
        toughness: -(n as i32),
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
