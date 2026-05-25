//! Brandywine Farmer — `{2}{G}` 1/1 Halfling Peasant.
//! "When this creature enters or leaves the battlefield, create a
//! Food token."
//!
//! GAP: "enters or leaves the battlefield" — no single trigger
//! condition covers both events. Using SelfEntersBattlefield for
//! the ETB trigger only; the "leaves the battlefield" half is not
//! expressible as a second trigger on a single TriggeredAbilityDef.

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
    let name = reg.interner_mut().intern("Brandywine Farmer");
    let halfling = reg.interner_mut().intern("Halfling");
    let peasant = reg.interner_mut().intern("Peasant");
    let _food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: only ETB half; "leaves the battlefield" half cannot
                // be expressed in a single TriggeredAbilityDef.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let food = reg.interner().lookup("Food").expect("Food interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: token_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
