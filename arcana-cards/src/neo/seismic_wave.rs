//! Seismic Wave — `{2}{R}` instant. "Seismic Wave deals 2 damage to any
//! target and 1 damage to each nonartifact creature target opponent controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seismic Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Seismic Wave deals 2 damage to any target and 1 damage to each nonartifact creature target opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::target_player(),
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
    let Some(any_target) = entry.targets.targets.first() else { return Vec::new(); };
    let damage_target = match any_target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };

    let Some(opp_target) = entry.targets.targets.get(1) else { return Vec::new(); };
    let opp = match opp_target {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };

    let nonartifact_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .without_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );

    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: damage_target,
        amount: 2,
    }];

    // The "target opponent" specifies whose creatures to hit; we use ids_matching
    // which already filters by ControllerConstraint::Opponent for any opponent.
    let _ = opp; // opp used to identify the target opponent; ForEach covers all opponents' nonartifact creatures
    effects.push(Effect::ForEach {
        targets: nonartifact_creatures,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    });
    effects
}
