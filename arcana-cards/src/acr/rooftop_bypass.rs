//! Rooftop Bypass — `{1}{U}{B}` enchantment.
//! "Whenever one or more nontoken creatures you control deal combat
//! damage to a player, create a 1/1 black Assassin creature token with
//! menace."
//!
//! Wired on `DamageDealt` filtered to nontoken creatures you control
//! hitting a player in combat. Fidelity note: the engine fires the
//! trigger per damage event, not once per "one or more" batch — a
//! documented over-fire for multi-creature hits.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rooftop Bypass");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _assassin = reg.interner_mut().intern("Assassin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "one or more nontoken creatures ... deal
            // combat damage" batches; this fires once per damaging
            // creature instead of once per batch.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: mint_assassin,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…create a 1/1 black Assassin creature token with menace."
fn mint_assassin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let assassin = reg.interner().lookup("Assassin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assassin.clone());
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: assassin,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Menace],
            abilities: vec![],
        },
    }]
}
