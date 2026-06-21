//! Michelangelo, Mutant BFF — `{2}{G}{G}` 4/4 Legendary Mutant Ninja Turtle.
//!
//! Oracle:
//! * "Each creature you control with a counter on it can't be blocked
//!   by more than one creature." — a continuous static combat
//!   restriction; not a triggered/activated ability, GAP'd.
//! * "Whenever Michelangelo enters or attacks, create a Mutagen
//!   token." — split into an enters trigger and an attacks trigger,
//!   each minting a Mutagen artifact token. The token's printed
//!   activated ability ("{1}, {T}, Sacrifice: put a +1/+1 counter on
//!   target creature") is an ACTIVATED ability which TokenDefinition
//!   cannot carry (its `abilities` are triggered only), so the bare
//!   token is created and the activation is GAP'd.

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
    let name = reg.interner_mut().intern("Michelangelo, Mutant BFF");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);
    // Pre-intern the Mutagen token name for the resolver lookup.
    let _mutagen = reg.interner_mut().intern("Mutagen");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Each creature you control with a counter on it can't
    // be blocked by more than one creature" — a continuous combat
    // restriction, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_mutagen,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_mutagen,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_mutagen(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Mutagen").unwrap_or_default();
    let mutagen_sub = reg.interner().lookup("Mutagen").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen_sub);
    // GAP: the token's printed activated ability ("{1}, {T}, Sacrifice
    // this token: Put a +1/+1 counter on target creature. Activate only
    // as a sorcery.") cannot be attached — TokenDefinition.abilities is
    // a Vec<TriggeredAbilityDef>, which holds no activated abilities.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
