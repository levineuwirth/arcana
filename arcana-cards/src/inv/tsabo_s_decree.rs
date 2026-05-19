//! Tsabo's Decree — `{5}{B}` instant.
//! "Choose a creature type. Target player reveals their hand and discards all creature cards of that type.
//! Then destroy all creatures of that type that player controls. They can't be regenerated."
//! GAP: runtime choice of creature type driving both discard and destroy is not expressible.
//! Best effort: destroy all creatures an opponent controls (type choice omitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tsabo's Decree");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose a creature type. Target player reveals their hand and discards all creature cards of that type. Then destroy all creatures of that type that player controls. They can't be regenerated.".into(),
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
    // GAP: runtime creature-type choice driving targeted discard and typed board wipe is not expressible
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: arcana_core::objects::NULL_OBJECT_ID }),
    }]
}
