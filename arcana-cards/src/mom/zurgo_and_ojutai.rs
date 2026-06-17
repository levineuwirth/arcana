//! Zurgo and Ojutai — `{2}{U}{R}{W}` 4/4 Legendary Orc Dragon with Flying and Haste.
//! "Zurgo and Ojutai has hexproof as long as it entered this turn.
//!  Whenever one or more Dragons you control deal combat damage to a player or
//!  battle, look at the top three cards of your library. Put one of them into
//!  your hand and the rest on the bottom of your library in any order. You may
//!  return one of those Dragons to its owner's hand."

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zurgo and Ojutai");
    let orc = reg.interner_mut().intern("Orc");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(dragon);

    let dragon_filter = script::subtype_filter(reg, "Dragon")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static "has hexproof as long as it entered this turn" — conditional self-keyword static not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: dragon_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: dig_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Look at the top three cards, put one into your hand, rest on bottom.
/// (The optional "return one of those Dragons to its owner's hand" rider is
/// not expressible alongside the dig and is omitted.)
fn dig_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may return one of those Dragons to its owner's hand" — combat-damage
    // source set is not addressable from the dig effect; rider omitted.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
