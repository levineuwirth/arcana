//! Bog Elemental — `{3}{B}{B}` 5/4 Elemental.
//!
//! * Protection from white — Protection is not a usable `KeywordAbility`
//!   variant, so it is GAP'd.
//! * "At the beginning of your upkeep, sacrifice this creature unless you
//!   sacrifice a land." — wired via Effect::OptionalPayment { Sacrifice(Land)
//!   → avoid penalty; else_effect = destroy this creature }. No self-sacrifice
//!   Effect variant exists, so the "sacrifice this creature" punishment is a
//!   direct DestroyPermanent of the source (the established idiom, e.g.
//!   me4/phantasmal_forces).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bog Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // GAP: "Protection from white" — Protection is not a usable keyword.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
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
            effect: upkeep_sacrifice_unless_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_sacrifice_unless_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice this creature unless you sacrifice a land" — pay = sacrifice a
    // land (penalty avoided); decline = sacrifice (destroy) this creature.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Land),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent { target: trig.source })),
    }]
}
