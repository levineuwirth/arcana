//! Enduring Victory — `{4}{W}` instant. "Destroy target attacking or
//! blocking creature. Bolster 1." The combat-state restriction is
//! enforced via `ObjectFilter::creature().attacking_or_blocking_only()`;
//! bolster 1 remains a GAP.

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
    let name = reg.interner_mut().intern("Enduring Victory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target attacking or blocking creature. Bolster 1.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().attacking_or_blocking_only(),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: bolster 1 (choose the creature with least toughness you
    // control, add a +1/+1 counter) is not in the catalog; only the
    // destroy is emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
