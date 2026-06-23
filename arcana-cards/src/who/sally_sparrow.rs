//! Sally Sparrow — `{2}{W}{U}` 2/3 Legendary Creature — Human Detective. W/U.
//!
//! Oracle text:
//! * You may cast creature spells as though they had flash.
//! * Whenever one or more other creatures you control leave the
//!   battlefield, investigate. This ability triggers only once each turn.
//!
//! Decomposition:
//! * GAP: "You may cast creature spells as though they had flash." — a static
//!   timing-permission ability (no trigger word, no activation cost); not
//!   expressible with the demonstrated triggered/activated primitives.
//! * Leave-the-battlefield trigger → one `TriggeredAbilityDef`. The
//!   ZoneChange variant requires a concrete `to` zone, so this is modeled as
//!   a creature you control going to the graveyard (the dominant
//!   leave-the-battlefield case). `OncePerTurn` honors "triggers only once
//!   each turn". Investigate is `Effect::CreateCommodityToken` for one Clue.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sally Sparrow");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let creature_you_control = ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: creature_you_control,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: investigate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
