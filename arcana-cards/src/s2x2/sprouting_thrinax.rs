//! Sprouting Thrinax — `{B}{R}{G}` 3/3 Lizard.
//! "When this creature dies, create three 1/1 green Saproling
//! creature tokens."

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
    let name = reg.interner_mut().intern("Sprouting Thrinax");
    let lizard = reg.interner_mut().intern("Lizard");
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
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

fn on_dies(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let saproling = reg.interner().lookup("Saproling")
        .expect("Saproling interned during register()");
    let make_token = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(saproling);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: saproling,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    vec![make_token(), make_token(), make_token()]
}
