//! Symbiotic Beast — `{4}{G}{G}` 4/4 green Insect Beast. "When this creature
//! dies, create four 1/1 green Insect creature tokens." Dies trigger; create
//! four 1/1 green Insect tokens.

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
    let name = reg.interner_mut().intern("Symbiotic Beast");
    let insect = reg.interner_mut().intern("Insect");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_death_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_death_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let insect = reg.interner().lookup("Insect").expect("Insect interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let make_token = || {
        let mut st = SubtypeSet::default();
        st.0.insert(insect);
        TokenDefinition {
            name: insect,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: st,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
    ]
}
