//! Bringer of the Red Dawn — `{7}{R}{R}` 5/5 red Bringer with Trample.
//!
//! Oracle:
//! * "You may pay {W}{U}{B}{R}{G} rather than pay this spell's mana cost."
//!   — an alternative casting cost (static casting modifier), GAP'd here:
//!   not a triggered/activated ability and no alternative-cost field is
//!   exposed by this card class.
//! * Trample — keyword.
//! * "At the beginning of your upkeep, you may untap target creature and
//!   gain control of it until end of turn. That creature gains haste until
//!   end of turn." — upkeep trigger targeting a creature; we untap it,
//!   gain control until end of turn, and grant haste. The "you may" is a
//!   resolution-time choice (not an intervening-if).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bringer of the Red Dawn");
    let bringer = reg.interner_mut().intern("Bringer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bringer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_steal_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn upkeep_steal_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Untap { target: *id },
        Effect::ChangeControlEot { target: *id, new_controller: trig.controller },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
