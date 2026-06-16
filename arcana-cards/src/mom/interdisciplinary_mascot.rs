//! Interdisciplinary Mascot — `{6}{U}{U}` 5/5 Elemental Fractal with
//! Ward {3}.
//! Convoke; "When this creature enters, look at the top four cards of
//! your library. Put one of them into your hand and the rest on the
//! bottom of your library in a random order."
//!
//! Ward {3} is a usable parametrized keyword. Convoke is a cast-time
//! cost reducer not in the usable keyword set (GAP). The ETB dig is a
//! DigTopN(4) with an unfiltered pick, rest to the bottom at random.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Interdisciplinary Mascot");
    let elemental = reg.interner_mut().intern("Elemental");
    let fractal = reg.interner_mut().intern("Fractal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(fractal);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Convoke not in usable KeywordAbility set.
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost"))],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
