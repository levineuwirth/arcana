//! Hydro-Man, Fluid Felon — `{U}{U}` 2/2 Legendary Elemental Villain.
//! "Whenever you cast a blue spell, if Hydro-Man is a creature, he gets +1/+1
//!  until end of turn."
//! "At the beginning of your end step, untap Hydro-Man. Until your next turn,
//!  he becomes a land and gains '{T}: Add {U}.' (He's not a creature during
//!  that time.)"
//!
//! Two triggers. The first pumps +1/+1 on each blue spell you cast (the
//! "if Hydro-Man is a creature" intervening-if is not expressible with the
//! demonstrated condition helpers, so it is noted as a partial). The second
//! untaps him at end step; the "becomes a land until your next turn and gains
//! a mana ability" rider is not expressible and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hydro-Man, Fluid Felon");
    let elemental = reg.interner_mut().intern("Elemental");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Partial: the "if Hydro-Man is a creature" intervening-if is not
            // expressible with the demonstrated condition helpers; the pump
            // always fires on a blue spell cast.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_colors(ColorSet::blue())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn end_step_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap {
        target: trig.source,
    }]
    // GAP: "Until your next turn, he becomes a land and gains '{T}: Add {U}.'
    // (He's not a creature during that time.)" — a temporary become-a-land
    // type-replacement plus granted mana ability is not expressible.
}
