//! Underworld Hermit — `{4}{B}{B}` 3/3 black Human Peasant.
//! "When this creature enters, create a number of 1/1 green Squirrel
//! creature tokens equal to your devotion to black."
//!
//! ETB trigger; the token count is your devotion to black, computed at
//! resolution via `script::devotion`, then that many `Effect::CreateToken`
//! values are emitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underworld Hermit");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
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
                effect: create_squirrels,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_squirrels(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::devotion(state, trig.controller, ColorSet::black());
    let squirrel = reg.interner().lookup("Squirrel")
        .expect("Squirrel interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
