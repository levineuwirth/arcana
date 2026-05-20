//! Cryoclasm — `{2}{R}` sorcery. "Destroy target Plains or Island.
//! Cryoclasm deals 3 damage to that land's controller."
//!
//! GAP: 'that land's controller' refers to the destroyed permanent's
//! controller; the catalog has no way to read that post-destroy.
//! Only the destroy is modeled. The target is a Land (subtype-OR not
//! expressible on a single TargetFilter).

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
    let name = reg.interner_mut().intern("Cryoclasm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target Plains or Island. Cryoclasm deals 3 damage to that land's controller.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::new().with_types(TypeLine::LAND.into())),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: 'deal 3 to that land's controller' — post-destroy controller lookup not modeled.
    vec![Effect::DestroyPermanent { target: *id }]
}
