//! Blow Your House Down — `{2}{R}` sorcery. Up to three target creatures
//! can't block this turn. Destroy any of them that are Walls.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blow Your House Down");
    let _wall = reg.interner_mut().intern("Wall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Up to three target creatures can't block this turn. Destroy any of them that are Walls.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature()),
                count: TargetCount::UpTo(3),
                controller: None,
            }],
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
    // "Can't block this turn" is not modeled — GAP that and destroy Walls.
    // GAP: "can't block this turn" rider not modeled.
    let walls = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Wall"),
        entry.controller,
    );
    let mut effects: Vec<Effect> = Vec::new();
    for choice in &entry.targets.targets {
        if let TargetChoice::Object(id) = choice {
            if walls.contains(id) {
                effects.push(Effect::DestroyPermanent { target: *id });
            }
        }
    }
    effects
}
