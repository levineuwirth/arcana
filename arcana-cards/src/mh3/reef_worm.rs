//! Reef Worm — `{3}{U}` 0/1 blue Worm creature. "When this creature dies,
//! create a 3/3 blue Fish creature token with 'When this token dies, create
//! a 6/6 blue Whale creature token with \"When this token dies, create a 9/9
//! blue Kraken creature token.\"'"
//! GAP: nested triggered abilities on created tokens are not expressible
//! with current engine API (TokenDefinition.abilities is vec![]).

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
    let name = reg.interner_mut().intern("Reef Worm");
    let worm = reg.interner_mut().intern("Worm");
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fish = reg.interner().lookup("Fish").expect("Fish interned during register()");
    let mut fish_subtypes = SubtypeSet::default();
    fish_subtypes.0.insert(fish);
    // GAP: the Fish token should carry "When this dies, create 6/6 Whale" trigger,
    // and the Whale should carry "When this dies, create 9/9 Kraken" trigger —
    // nested triggered abilities on tokens are not expressible (abilities: vec![])
    let fish_token = TokenDefinition {
        name: fish,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: fish_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token: fish_token }]
}
