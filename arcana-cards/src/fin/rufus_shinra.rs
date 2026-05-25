//! Rufus Shinra — `{1}{W}{B}` 2/4 legendary Human Noble. "Whenever
//! Rufus Shinra attacks, if you don't control a creature named
//! Darkstar, create Darkstar, a legendary 2/2 white and black Dog
//! creature token."
//!
//! GAP: the intervening-if clause ("if you don't control a creature
//! named Darkstar") is not expressible — `ObjectFilter` cannot filter
//! by card name, and `TriggeredAbilityDef.intervening_if` is set to
//! `None` per generator policy. The token is created on every attack,
//! losing the uniqueness gate.
//!
//! GAP: `TokenDefinition` has no supertypes field, so the "legendary"
//! supertype on the Darkstar token is dropped.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rufus Shinra");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    // Pre-intern the token's name and subtype so the trigger resolver
    // can read them via the non-mut interner.
    let _darkstar = reg.interner_mut().intern("Darkstar");
    let _dog = reg.interner_mut().intern("Dog");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: "if you don't control a creature named Darkstar"
                // — name-based ObjectFilter is not supported.
                intervening_if: None,
                effect: create_darkstar_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_darkstar_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let darkstar = reg
        .interner()
        .lookup("Darkstar")
        .expect("Darkstar interned during register()");
    let dog = reg
        .interner()
        .lookup("Dog")
        .expect("Dog interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    let token = TokenDefinition {
        name: darkstar,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
