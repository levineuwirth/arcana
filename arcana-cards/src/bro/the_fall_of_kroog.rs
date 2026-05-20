//! The Fall of Kroog — `{4}{R}{R}` sorcery.
//! "Choose target opponent. Destroy target land that player controls. The Fall
//! of Kroog deals 3 damage to that player and 1 damage to each creature they control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
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
                text: "Choose target opponent. Destroy target land that player controls. The Fall of Kroog deals 3 damage to that player and 1 damage to each creature they control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types(TypeLine::LAND.into()),
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
    let mut targets = entry.targets.targets.iter();
    let Some(player_target) = targets.next() else { return Vec::new(); };
    let Some(land_target) = targets.next() else { return Vec::new(); };

    let opp = match player_target {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    let TargetChoice::Object(land_id) = land_target else { return Vec::new(); };

    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );

    let mut effects = vec![
        Effect::DestroyPermanent { target: *land_id },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(opp),
            amount: 3,
        },
        Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        },
    ];
    effects
}
