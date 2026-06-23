//! Genghis Frog — `{G}{U}` 1/3 Legendary Frog Mutant Rogue (G/U) with
//! Trample.
//!
//! Oracle:
//! * Trample.
//! * Whenever Genghis Frog or another Mutant you control enters, create
//!   a Mutagen token. (It's an artifact with "{1}, {T}, Sacrifice this
//!   token: Put a +1/+1 counter on target creature. Activate only as a
//!   sorcery.")
//!
//! Trample is a base keyword. The ETB trigger fires for any Mutant the
//! controller controls entering (including this creature itself), via a
//! `ZoneChange` filtered to Mutant creatures you control, and mints a
//! Mutagen artifact token. The token's printed activated ability is a
//! GAP — `TokenDefinition.abilities` holds only triggered abilities, so
//! the "{1}, {T}, Sacrifice: +1/+1 counter on target creature"
//! activation cannot be authored here; the bare artifact token is minted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genghis Frog");
    let frog = reg.interner_mut().intern("Frog");
    let mutant = reg.interner_mut().intern("Mutant");
    let rogue = reg.interner_mut().intern("Rogue");
    // Pre-intern the token's subtype name for resolution-time lookup.
    let _mutagen = reg.interner_mut().intern("Mutagen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(mutant);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Mutant")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: create_mutagen,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_mutagen(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg.interner().lookup("Mutagen").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen);
    // GAP: token's "{1}, {T}, Sacrifice: put a +1/+1 counter on target
    // creature" is an ACTIVATED ability; TokenDefinition.abilities only
    // accepts triggered abilities, so only the bare artifact is minted.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: mutagen,
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
