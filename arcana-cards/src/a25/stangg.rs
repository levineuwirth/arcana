//! Stangg — `{4}{R}{G}` 3/4 Legendary Human Warrior. "When Stangg
//! enters, create Stangg Twin, a legendary 3/4 red and green Human
//! Warrior creature token. Exile that token when Stangg leaves the
//! battlefield. Sacrifice Stangg when that token leaves the battlefield."
//!
//! GAP: The token is Legendary but TokenDefinition has no supertypes field.
//! GAP: The linked triggered abilities (exile the token when Stangg leaves,
//! sacrifice Stangg when the token leaves) require token-identity tracking
//! and LTB-conditional delayed triggers not exposed in the current API.

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
    let name = reg.interner_mut().intern("Stangg");
    let _stangg_twin = reg.interner_mut().intern("Stangg Twin");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_stangg_twin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_stangg_twin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let stangg_twin = reg.interner().lookup("Stangg Twin")
        .expect("Stangg Twin interned during register()");
    let human = reg.interner().lookup("Human")
        .expect("Human interned during register()");
    let warrior = reg.interner().lookup("Warrior")
        .expect("Warrior interned during register()");
    
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    
    let token = TokenDefinition {
        name: stangg_twin,
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
