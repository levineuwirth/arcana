//! Cultivator Colossus — `{4}{G}{G}{G}` */* Plant Beast with Trample.
//! "Cultivator Colossus's power and toughness are each equal to the
//! number of lands you control."
//! "When this creature enters, you may put a land card from your hand
//! onto the battlefield tapped. If you do, draw a card and repeat this
//! process."
//!
//! Trample is a base keyword; the */* P/T is represented by
//! PtValue::Star (its defining static is the lands-you-control count,
//! which is GAP'd as a CDA the demonstrated API can't install). The ETB
//! is modeled as a single iteration: put a land from hand onto the
//! battlefield tapped. The "draw a card and repeat this process" loop is
//! not expressible (no iterate-until-decline primitive), so the draw and
//! repeat are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cultivator Colossus");
    let plant = reg.interner_mut().intern("Plant");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "power and toughness are each equal to the number of lands you
    // control" — the */* defining static (a CDA) cannot be installed via the
    // demonstrated triggered/activated API; PtValue::Star marks the slot.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_put_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_put_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Single iteration: put a land card from hand onto the battlefield tapped.
    // GAP: "If you do, draw a card and repeat this process." — there is no
    // iterate-until-decline loop primitive, so the draw and the repeat are
    // omitted.
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}
