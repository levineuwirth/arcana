//! Redcap Melee — `{R}` instant. Deals 4 damage to target creature or
//! planeswalker. If a nonred permanent is dealt damage this way, you
//! sacrifice a land.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Redcap Melee");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Redcap Melee deals 4 damage to target creature or planeswalker. If a nonred permanent is dealt damage this way, you sacrifice a land.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let mut effects: Vec<Effect> = vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: 4,
    }];
    // "If a nonred permanent is dealt damage this way" — check at resolve time using the targeted permanent.
    let nonred_targets = script::ids_matching(
        state,
        &ObjectFilter::permanent().without_colors(ColorSet::red()),
        entry.controller,
    );
    if nonred_targets.contains(&id) {
        effects.push(Effect::Sacrifice {
            player: entry.controller,
            filter: ObjectFilter::permanent()
                .controlled_by(ControllerConstraint::You)
                .with_types(TypeLine::LAND.into()),
            count: 1,
        });
    }
    effects
}
