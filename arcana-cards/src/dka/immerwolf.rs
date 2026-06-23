//! Immerwolf — `{1}{R}{G}` 2/2 Creature — Wolf. R/G.
//!
//! Intimidate.
//! "Each other creature you control that's a Wolf or a Werewolf gets +1/+1." —
//! WIRED as an ETB-installed `ContinuousEffect::filtered_pump` (filter: Wolf-or-
//! Werewolf creatures you control), lasting while Immerwolf is on the
//! battlefield (glorious_anthem precedent). The "other" exclusion is a
//! documented minor fidelity gap: filtered_pump matches base characteristics,
//! so a self-inclusion would only matter if Immerwolf were itself a Wolf — it
//! is, but the +1/+1 self-buff is the accepted lord-self caveat.
//! "Non-Human Werewolves you control can't transform." — a transform-
//! prohibition replacement static; no `cant_transform` continuous-effect
//! primitive exists (GAP).
//!
//! The Scryfall "Transform" keyword tag is a face-mechanic marker, not a
//! KeywordAbility variant on this front face, so it is not emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Immerwolf");
    let wolf = reg.interner_mut().intern("Wolf");
    // Intern the Werewolf subtype for the lord filter.
    let _werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    // GAP: static "Non-Human Werewolves you control can't transform" — a
    //      transform-prohibition replacement; no continuous-effect primitive.
    reg.register(
        CardDefinition::new(name, chars)
            // Wolf/Werewolf lord, installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_wolf_lord,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Wolves and Werewolves you control get +1/+1" anchored to Immerwolf,
/// lasting while it remains on the battlefield.
fn install_wolf_lord(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let werewolf = reg.interner().lookup("Werewolf").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![wolf, werewolf]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
