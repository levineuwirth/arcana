//! Bartered Cow — `{3}{W}` 3/3 white Ox.
//! "When this creature dies and when you discard this card, create a Food token."
//! GAP: "when you discard this card" trigger (fires from hand/graveyard context)
//! not in TriggerCondition catalog. SelfDies handles the first trigger only.

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
    let name = reg.interner_mut().intern("Bartered Cow");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
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
    let food = reg.interner().lookup("Food").expect("Food interned during register()");
    let mut food_subtypes = SubtypeSet::default();
    food_subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: food_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "when you discard this card" trigger not in catalog (only SelfDies handled).
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
