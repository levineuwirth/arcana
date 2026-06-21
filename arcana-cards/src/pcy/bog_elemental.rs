//! Bog Elemental — `{3}{B}{B}` 5/4 Elemental.
//!
//! * Protection from white — Protection is not a usable `KeywordAbility`
//!   variant, so it is GAP'd.
//! * "At the beginning of your upkeep, sacrifice this creature unless you
//!   sacrifice a land." — an upkeep trigger whose "unless you sacrifice a
//!   land" gate has no expressible cost shape (`OptionalPaymentKind` is
//!   Mana/Life only; sacrifice-a-permanent is not a payment kind). The whole
//!   conditional sacrifice is GAP'd rather than firing an unconditional
//!   self-sacrifice (which would be a materially wrong card).

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
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice this creature unless you sacrifice a land" — neither side
    // of the choice is an expressible OptionalPayment cost (sacrifice is not a
    // payment kind), so the conditional self-sacrifice is omitted.
    Vec::new()
}
