//! Splinterfright — `{2}{G}` */* green Elemental.
//!
//! Oracle:
//! * Trample.
//! * "Splinterfright's power and toughness are each equal to the number of
//!   creature cards in your graveyard." — a characteristic-defining
//!   ability; no documented hook attaches a CDA P/T resolver, so power and
//!   toughness are left as the printed `*` (PtValue::Star). GAP (CDA).
//! * "At the beginning of your upkeep, mill two cards." — an upkeep
//!   StepBegins trigger milling two cards. (Scryfall lists "Mill" as a
//!   keyword, but it is the action of this ability, not a separate
//!   KeywordAbility variant — only Trample is emitted.)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Splinterfright");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP (CDA): P/T each equal to creature cards in your graveyard;
        // left as the printed `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
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
                effect: mill_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mill_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 2,
    }]
}
