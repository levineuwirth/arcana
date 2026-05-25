//! Gemini Engine — `{6}` 3/4 Artifact Creature — Construct.
//! "Whenever this creature attacks, create a colorless Construct
//! artifact creature token named Twin that's attacking. Its power is
//! equal to this creature's power and its toughness is equal to this
//! creature's toughness. Sacrifice the token at end of combat."
//!
//! GAP: token power/toughness equal to this creature's power/toughness
//! — TokenDefinition requires fixed PtValue; dynamic P/T not supported.
//! Also "token is attacking" initial state not supported in CreateToken.
//! CreateTokenSacEot used for the sac-at-end-of-combat portion.

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
    let name = reg.interner_mut().intern("Gemini Engine");
    let construct = reg.interner_mut().intern("Construct");
    let _twin = reg.interner_mut().intern("Twin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: token P/T equal to this creature's P/T — dynamic P/T in
    // TokenDefinition not supported. Using fixed 3/4.
    // GAP: token enters attacking — not supported in CreateTokenSacEot.
    let twin = reg
        .interner()
        .lookup("Twin")
        .expect("Twin interned during register()");
    let construct = reg
        .interner()
        .lookup("Construct")
        .expect("Construct interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let token = TokenDefinition {
        name: twin,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateTokenSacEot { controller: trig.controller, token }]
}
