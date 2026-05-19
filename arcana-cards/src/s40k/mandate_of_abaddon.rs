//! Mandate of Abaddon — `{3}{B}` sorcery. "Choose target creature you control.
//! Destroy all creatures with power less than that creature's power."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mandate of Abaddon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control. Destroy all creatures with power less than that creature's power.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
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
    let Some(TargetChoice::Object(chosen_id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let power = script::power_of(state, *chosen_id);
    if power <= 0 {
        return Vec::new();
    }
    // .with_max_power(n) matches power <= n; we want power < chosen, so max = power - 1.
    let filter = ObjectFilter::creature().with_max_power(power - 1);
    let ids = script::ids_matching(state, &filter, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
