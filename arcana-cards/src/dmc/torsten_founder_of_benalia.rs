//! Torsten, Founder of Benalia — `{5}{G}{W}` 7/7 legendary Human
//! Soldier (G/W).
//!
//! * When Torsten enters, reveal the top seven cards of your library.
//!   Put any number of creature and/or land cards from among them into
//!   your hand and the rest on the bottom of your library in a random
//!   order. (DigTopN, looking at 7 with a creature-or-land take and the
//!   rest to the bottom in random order; the "any number" multi-take is
//!   narrowed to a single take — documented partial.)
//! * When Torsten dies, create seven 1/1 white Soldier creature tokens.

use arcana_core::effects::{DigRest, Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Torsten, Founder of Benalia");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_seven,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_seven(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 7,
        filter: Some(
            ObjectFilter::new().with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
        ),
        rest: DigRest::BottomRandom,
    }]
}

fn dies_make_soldiers(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut st = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Soldier") {
        st.0.insert(s);
    }
    let token_name = reg.interner().lookup("Soldier").unwrap_or_default();
    let token = TokenDefinition {
        name: token_name,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut out = Vec::new();
    for _ in 0..7 {
        out.push(Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        });
    }
    out
}
