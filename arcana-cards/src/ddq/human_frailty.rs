//! Human Frailty — `{B}` instant. "Destroy target Human creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Human Frailty");
    let _ = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let human_filter = script::subtype_filter(reg, "Human");
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target Human creature.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(human_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // Validate target is still Human at resolution.
    let humans = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Human"),
        entry.controller,
    );
    if !humans.contains(id) {
        return Vec::new();
    }
    vec![Effect::DestroyPermanent { target: *id }]
}
