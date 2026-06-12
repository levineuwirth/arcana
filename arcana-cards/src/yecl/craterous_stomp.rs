//! Craterous Stomp — `{1}{R}` Kindred Instant — Giant. "Craterous Stomp
//! deals 3 damage to target creature an opponent controls. Each other
//! creature that player controls becomes a Coward in addition to its other
//! types and gains 'This creature can't block Giants or Warriors.'"
//! The grant is modeled as a filtered can't-block static until end of
//! turn. GAP (narrowed): the restriction is a full can't-block (attacker
//! scoping "Giants or Warriors" not expressible); "that player" is
//! approximated as ControllerConstraint::Opponent (exact in two-player);
//! the targeted creature is not excludable (no exclude-by-id filter); the
//! Coward type-add is not modeled.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Craterous Stomp");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Craterous Stomp deals 3 damage to target creature an opponent controls. Each other creature that player controls becomes a Coward in addition to its other types and gains \"This creature can't block Giants or Warriors.\"".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
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
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 3,
        },
        // "Each other creature that player controls ... gains 'This
        // creature can't block Giants or Warriors.'" GAP (narrowed): full
        // can't-block (no attacker scoping), Opponent stands in for "that
        // player" (exact in 2p), target not excludable, Coward type-add
        // not modeled.
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_cant_block(
                entry.source,
                ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                Duration::EndOfTurn,
            ),
        },
    ]
}
