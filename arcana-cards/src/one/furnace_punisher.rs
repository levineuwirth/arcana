//! Furnace Punisher — `{2}{R}` 3/3 Phyrexian Warrior with Menace.
//! "Menace"
//! "At the beginning of each player's upkeep, this creature deals 2 damage to
//!  that player unless they control two or more basic lands."
//!
//! The upkeep trigger fires on EACH player's upkeep (`whose: Any`). "That
//! player" is the upkeep's owner = the active player. The "unless they control
//! two or more basic lands" clause concerns that player (not the source
//! controller), so it can't ride an intervening-if helper (those are keyed to
//! the source's controller); it is evaluated at resolution against the active
//! player's basic-land count.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Furnace Punisher");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: each_upkeep_punish,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_upkeep_punish(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" = the upkeep owner = the active player.
    let them = state.active_player();
    let basics = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .controlled_by(ControllerConstraint::You);
    // count from that player's perspective
    let n = script::count_matching(state, &basics, them);
    if n >= 2 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(them),
        amount: 2,
    }]
}
