//! Mistakes Were Made — `{1}{G}` instant. "Destroy target artifact
//! or enchantment. Create a 1/1 green Squirrel creature token for each
//! fire extinguisher you can see from your seat." The destroy clause is
//! expressible; the token count depends on a real-world Un-set quantity
//! (fire extinguishers visible from the player's seat) that has no game
//! state representation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistakes Were Made");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target artifact or enchantment. Create a 1/1 green Squirrel creature token for each fire extinguisher you can see from your seat.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: token count is "for each fire extinguisher you can see from your
    // seat", a real-world quantity with no game-state representation and no
    // script:: helper. Emitting the destroy clause only; the token creation
    // cannot be faithfully expressed.
    vec![Effect::DestroyPermanent { target: *id }]
}
