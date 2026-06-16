//! The Brewing Chef — `{2}{G}{W}{U}` 3/3 Legendary Human Chef.
//! "When The Brewing Chef enters, look at the top six cards of your library. You
//! may reveal a card from among them that creates a token and put it into your
//! hand. Put the rest on the bottom of your library in a random order." /
//! "Creature tokens you control get +1/+1 for each differently named token you
//! control."

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("The Brewing Chef");
    let human = reg.interner_mut().intern("Human");
    let chef = reg.interner_mut().intern("Chef");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(chef);
    // GAP: "Creature tokens you control get +1/+1 for each differently named
    // token you control" — a token-filtered, distinct-name-scaled continuous
    // anthem is not expressible in this card class; static omitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: dig_six,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_six(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "a card ... that creates a token" filter is not an
    // ObjectFilter predicate — any card may be taken here (filter: None).
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
