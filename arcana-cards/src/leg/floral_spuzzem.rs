//! Floral Spuzzem — `{3}{G}` 2/2 green Elemental.
//! "Whenever this creature attacks and isn't blocked, you may destroy target
//! artifact defending player controls. If you do, this creature assigns no
//! combat damage this turn."
//! GAP: trigger condition — "attacks and isn't blocked" is not a distinct
//! TriggerCondition; using SelfAttacks as best-effort.
//! GAP: effect — "assigns no combat damage this turn" prevention not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Floral Spuzzem");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter {
                                types_any: Some(TypeLine(TypeLine::ARTIFACT)),
                                ..Default::default()
                            },
                        ),
                        count: TargetCount::UpTo(1),
                        controller: Some(arcana_core::targets::ControllerConstraint::Opponent),
                    },
                ],
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attacks and isn't blocked" — only attacks captured.
    // GAP: "assigns no combat damage" prevention not expressible.
    let mut effects = Vec::new();
    if let Some(target) = trig.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    effects
}
