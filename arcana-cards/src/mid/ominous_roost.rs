//! Ominous Roost — `{2}{U}` enchantment.
//! "When this enchantment enters and whenever you cast a spell from
//! your graveyard, create a 1/1 blue Bird creature token with flying
//! and 'This token can block only creatures with flying.'"
//!
//! The ETB half mints the Bird faithfully (the block-restriction
//! printed ability is a GAP — tokens carry no authored abilities). The
//! "cast a spell FROM YOUR GRAVEYARD" half has no zone-qualified
//! SpellCast condition; the closest variant is wired with the effect
//! stubbed so every ordinary cast doesn't wrongly mint a Bird.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ominous Roost");
    let _bird = reg.interner_mut().intern("Bird");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mint_bird,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: trigger — "whenever you cast a spell FROM YOUR
                // GRAVEYARD": SpellCast has no cast-from-zone qualifier;
                // closest variant wired, effect stubbed to avoid minting a
                // Bird on every ordinary cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: graveyard_cast_stub,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…create a 1/1 blue Bird creature token with flying…"
fn mint_bird(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's printed "This token can block only creatures with
    // flying." restriction cannot be authored on a TokenDefinition.
    let bird = reg.interner().lookup("Bird").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: bird,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

/// Stub for the cast-from-graveyard half (condition inexpressible).
fn graveyard_cast_stub(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot check that the spell was cast from the graveyard, so
    // minting here would over-fire on every spell. Stubbed.
    Vec::new()
}
