//! Item Shopkeep — `{1}{R}` 2/2 Human Citizen. "Whenever you attack,
//! target attacking equipped creature gains menace until end of turn."
//! Models the once-per-combat "you attack" trigger via the closest
//! catalog approximation (`CreatureAttacks` filtered to creatures you
//! control — fires per attacker rather than once per attack step).
//!
//! # GAPs
//! * "Whenever you attack" — the engine has no once-per-combat
//!   player-attacks condition; using `CreatureAttacks` filtered to
//!   You as the nearest available variant.
//! * Target filter "attacking equipped creature" — `ObjectFilter` has
//!   no `attacking_only()` / `equipped_only()` refinements, so the
//!   target is broadened to "target creature" at the def level.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Item Shopkeep");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — no "whenever you attack" (player-level,
            // once per combat) variant; using CreatureAttacks filtered
            // to creatures you control as the closest approximation.
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: grant_menace_to_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: target filter — ObjectFilter cannot express
            // "attacking" / "equipped"; broadened to target creature.
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn grant_menace_to_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Menace,
        duration: Duration::EndOfTurn,
    }]
}
