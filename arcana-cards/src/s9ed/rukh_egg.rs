//! Rukh Egg — `{3}{R}` 0/3 red creature. "When this creature dies, create a
//! 4/4 red Bird creature token with flying at the beginning of the next end
//! step."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Rukh Egg");
    let bird = reg.interner_mut().intern("Bird");
    let egg = reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(egg);
    // Pre-intern token subtype
    let _bird_token = reg.interner_mut().intern("Bird");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_create_bird_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_create_bird_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird = reg.interner().lookup("Bird")
        .expect("Bird interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // The token should be created at the beginning of the next end step;
    // CreateToken fires now (closest available). GAP: delayed-creation at next
    // end step; using immediate CreateToken as best-effort.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
