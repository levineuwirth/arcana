//! Battering Ram — `{2}` 1/1 Artifact Creature — Construct.
//! At the beginning of combat on your turn, this creature gains banding
//! until end of combat.
//! Whenever this creature becomes blocked by a Wall, destroy that Wall at
//! end of combat.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battering Ram");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    // Pre-intern the Wall subtype for the blocked-by filter.
    let _wall = reg.interner_mut().intern("Wall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    let wall_filter = script::subtype_filter(reg, "Wall");
    reg.register(
        CardDefinition::new(name, chars)
            // Begin combat on your turn: gain banding.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_banding,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Becomes blocked by a Wall: destroy that Wall.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesBlockedBy {
                    filter: wall_filter,
                },
                intervening_if: None,
                effect: destroy_blocking_wall,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_banding(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: precise "until end of combat" duration is not expressible;
    // EndOfTurn is the nearest available grant duration.
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Banding,
        duration: Duration::EndOfTurn,
    }]
}

fn destroy_blocking_wall(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at end of combat" timing is not expressible (DelayedAction has no
    // end-of-combat window nor a Destroy action); the Wall is destroyed
    // immediately on resolution.
    let Some(id) = trig.other_combatant() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: id }]
}
