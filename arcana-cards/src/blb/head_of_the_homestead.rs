//! Head of the Homestead — `{3}{G/W}{G/W}` 3/2 green/white Creature — Rabbit
//! Citizen. "When this creature enters, create two 1/1 white Rabbit creature
//! tokens."

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Head of the Homestead");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let citizen = reg.interner_mut().intern("Citizen");
    let _rabbit_tok = reg.interner_mut().intern("Rabbit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_two_rabbit_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_two_rabbit_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rabbit = reg.interner().lookup("Rabbit")
        .expect("Rabbit interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(rabbit);
    let make_token = || {
        let mut ts = SubtypeSet::default();
        ts.0.insert(rabbit);
        TokenDefinition {
            name: rabbit,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: ts,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
    ]
}
