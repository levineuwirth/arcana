//! The Fall of Kroog — `{4}{R}{R}` sorcery. "Choose target opponent.
//! Destroy target land that player controls. The Fall of Kroog deals
//! 3 damage to that player and 1 damage to each creature they
//! control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("The Fall of Kroog");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target opponent. Destroy target land that player \
                       controls. The Fall of Kroog deals 3 damage to that \
                       player and 1 damage to each creature they control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new()
                                .with_types(TypeLine::LAND.into())
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
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.get(1) {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: 3,
        });
        let filter =
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
        let ids = script::ids_matching(state, &filter, *p);
        for id in ids {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(id),
                amount: 1,
            });
        }
    }
    let _ = NULL_OBJECT_ID;
    effects
}
