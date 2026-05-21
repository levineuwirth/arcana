//! Grim Contest — `{1}{B}{G}` instant. "Choose target creature you
//! control and target creature an opponent controls. Each of those
//! creatures deals damage equal to its toughness to the other."

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
    let name = reg.interner_mut().intern("Grim Contest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature \
                       an opponent controls. Each of those creatures deals \
                       damage equal to its toughness to the other.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
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
    let Some(TargetChoice::Object(mine)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    // Toughness-based "fight" — each deals its toughness to the other.
    let mine_tou = script::toughness_of(state, *mine).max(0) as u32;
    let theirs_tou = script::toughness_of(state, *theirs).max(0) as u32;
    vec![
        Effect::DealDamage {
            source: *mine,
            target: DamageTarget::Object(*theirs),
            amount: mine_tou,
        },
        Effect::DealDamage {
            source: *theirs,
            target: DamageTarget::Object(*mine),
            amount: theirs_tou,
        },
    ]
}
