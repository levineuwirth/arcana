//! Merrow Skyswimmer — `{3}{W/U}{W/U}` 2/2 Merfolk Soldier.
//!
//! Oracle:
//! * Convoke
//! * Flying, vigilance
//! * When this creature enters, create a 1/1 white and blue Merfolk
//!   creature token.
//!
//! Flying and Vigilance are base keywords. Convoke is not in the usable
//! keyword surface for this card class (an alternative-cost reduction), so
//! it is GAP'd. The ETB makes a 1/1 W/U Merfolk token.
//!
//! GAP: Convoke — no usable KeywordAbility variant in this card class.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Merrow Skyswimmer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(
        TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: make_merfolk_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        },
    ))
}

fn make_merfolk_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merfolk = reg.interner().lookup("Merfolk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: merfolk,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
