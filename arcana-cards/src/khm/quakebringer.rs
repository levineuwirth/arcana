//! Quakebringer — `{3}{R}{R}` 5/4 Creature — Giant Berserker.
//!
//! * Your opponents can't gain life. (Static replacement — not expressible;
//!   GAP'd.)
//! * At the beginning of your upkeep, Quakebringer deals 2 damage to each
//!   opponent. This ability triggers only if Quakebringer is on the
//!   battlefield or if it's in your graveyard and you control a Giant.
//! * Foretell {2}{R}{R} — not a supported keyword; GAP'd.
//!
//! GAP: "Your opponents can't gain life" static is not expressible.
//! GAP: Foretell is not in the supported keyword surface.
//! GAP: the graveyard-trigger gate "and you control a Giant" is not
//! expressible; the upkeep trigger is wired to also fire from the graveyard
//! (trigger_zones includes Graveyard) but without the Giant precondition.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quakebringer");
    let giant = reg.interner_mut().intern("Giant");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_burn_opponents,
            trigger_zones: vec![Zone::Battlefield, Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_burn_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 2,
            source: trig.source,
        })
        .collect()
}
