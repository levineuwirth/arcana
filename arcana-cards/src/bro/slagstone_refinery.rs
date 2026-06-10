//! Slagstone Refinery — `{4}` artifact (The Brothers' War, 2022).
//! "Whenever this artifact or another nontoken artifact you control is
//! put into a graveyard from the battlefield or is put into exile from
//! the battlefield, create a tapped Powerstone token."
//! Modeled as a ZoneChange trigger on nontoken artifacts you control
//! going battlefield → graveyard, minting a Powerstone via
//! `Effect::CreateCommodityToken`. The "or is put into exile" half of
//! the trigger and the token entering tapped are documented GAPs.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slagstone Refinery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "or is put into exile from the
                // battlefield": only one destination zone is expressible
                // per ZoneChange condition; the exile half is unmodeled.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .nontoken()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: make_powerstone,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn make_powerstone(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Powerstone token enters TAPPED — entering-tapped riders
    // on CreateCommodityToken are not expressible.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Powerstone,
        count: 1,
    }]
}
