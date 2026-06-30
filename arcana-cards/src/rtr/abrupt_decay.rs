//! Abrupt Decay — `{B}{G}` instant. "This spell can't be countered. Destroy
//! target nonland permanent with mana value 3 or less." Targeting is restricted
//! to nonland permanents; the mana-value-3 check is enforced at resolution.

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
    let name = reg.interner_mut().intern("Abrupt Decay");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        cant_be_countered: true,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "This spell can't be countered. Destroy target nonland permanent with mana value 3 or less.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = match entry.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let Some(obj) = state.objects.get(id) else { return Vec::new(); };
    if obj.characteristics.mana_value() <= 3 {
        vec![Effect::DestroyPermanent { target: id }]
    } else {
        Vec::new()
    }
}
