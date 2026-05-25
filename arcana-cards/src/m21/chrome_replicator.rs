//! Chrome Replicator — `{5}` (colorless) 4/4 Artifact Creature — Construct.
//! "When this creature enters, if you control two or more nonland, nontoken
//! permanents with the same name as one another, create a 4/4 colorless
//! Construct artifact creature token."
//!
//! GAP: intervening-if "two or more nonland nontoken permanents with the
//! same name as one another" requires name-matching logic not available in
//! script helpers.

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
    let name = reg.interner_mut().intern("Chrome Replicator");
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(reg.interner_mut().intern("Construct"));
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::default(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot check if two or more nonland nontoken permanents share a
    // name; emitting token creation unconditionally as best effort.
    let construct = reg.interner().lookup("Construct")
        .expect("Construct interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: construct,
        colors: ColorSet::default(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
