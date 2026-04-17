//! Phase 1 sample card definitions.
//!
//! These four cards drive the Lightning Bolt milestone test
//! (addendum Section 14) — the minimum viable card set to exercise
//! mana production, creature combat, and a targeted removal spell.
//!
//! Production card definitions live in the separate `arcana-cards`
//! crate and are code-generated from Scryfall data by `arcana-gen`.
//! This module is kept inside `arcana-core` so the core crate is
//! self-contained for integration testing.

use crate::effects::Effect;
use crate::events::DamageTarget;
use crate::mana::{ManaCost, ManaUnit};
use crate::objects::Characteristics;
use crate::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry, SpellAbilityDef,
};
use crate::stack::StackEntry;
use crate::state::GameState;
use crate::targets::{TargetChoice, TargetRequirement};
use crate::types::*;

// =============================================================================
// Basic lands
// =============================================================================

/// Register Mountain: basic land with `{T}: Add {R}`.
pub fn register_mountain(registry: &mut CardRegistry) -> CardId {
    let name = registry.interner_mut().intern("Mountain");
    let subtype = registry.interner_mut().intern("Mountain");
    let mut subtypes = crate::types::SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::BASIC),
        ..Default::default()
    };
    registry.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                effect: add_red_mana,
            }),
    )
}

/// Register Forest: basic land with `{T}: Add {G}`.
pub fn register_forest(registry: &mut CardRegistry) -> CardId {
    let name = registry.interner_mut().intern("Forest");
    let subtype = registry.interner_mut().intern("Forest");
    let mut subtypes = crate::types::SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::BASIC),
        ..Default::default()
    };
    registry.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                effect: add_green_mana,
            }),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

// =============================================================================
// Grizzly Bears
// =============================================================================

/// Register Grizzly Bears: vanilla 2/2 creature for `{1}{G}`.
pub fn register_grizzly_bears(registry: &mut CardRegistry) -> CardId {
    let name = registry.interner_mut().intern("Grizzly Bears");
    let subtype = registry.interner_mut().intern("Bear");
    let mut subtypes = crate::types::SubtypeSet::default();
    subtypes.0.insert(subtype);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    registry.register(CardDefinition::new(name, chars))
}

// =============================================================================
// Lightning Bolt
// =============================================================================

/// Register Lightning Bolt: instant, `{R}`, "Lightning Bolt deals 3
/// damage to any target."
pub fn register_lightning_bolt(registry: &mut CardRegistry) -> CardId {
    let name = registry.interner_mut().intern("Lightning Bolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    registry.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Lightning Bolt deals 3 damage to any target.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
                effect: bolt_effect,
            }),
    )
}

fn bolt_effect(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match entry.targets.targets.first() {
        Some(t) => t,
        None => return Vec::new(),
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            crate::targets::ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            crate::targets::ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 3,
    }]
}

// =============================================================================
// Bulk registration
// =============================================================================

/// Register every Phase 1 sample card. Convenient shortcut for
/// tests and the arcana-cli debugger.
pub fn register_all_phase1_samples(registry: &mut CardRegistry) -> Phase1Ids {
    Phase1Ids {
        mountain: register_mountain(registry),
        forest: register_forest(registry),
        grizzly_bears: register_grizzly_bears(registry),
        lightning_bolt: register_lightning_bolt(registry),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Phase1Ids {
    pub mountain: CardId,
    pub forest: CardId,
    pub grizzly_bears: CardId,
    pub lightning_bolt: CardId,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mountain_is_a_basic_land_with_one_mana_ability() {
        let mut r = CardRegistry::new();
        let id = register_mountain(&mut r);
        let def = r.get(id).unwrap();
        assert!(def.base_characteristics.types.is_land());
        assert!(def.base_characteristics.supertypes.is_basic());
        assert_eq!(def.activated_abilities.len(), 1);
        assert!(def.activated_abilities[0].is_mana_ability);
    }

    #[test]
    fn bolt_has_any_target_requirement() {
        let mut r = CardRegistry::new();
        let id = register_lightning_bolt(&mut r);
        let def = r.get(id).unwrap();
        let sa = def.spell_ability.as_ref().expect("Bolt has a spell ability");
        assert_eq!(sa.target_requirements.len(), 1);
    }

    #[test]
    fn grizzly_bears_is_2_2_creature() {
        let mut r = CardRegistry::new();
        let id = register_grizzly_bears(&mut r);
        let def = r.get(id).unwrap();
        assert!(def.base_characteristics.types.is_creature());
        assert_eq!(def.base_characteristics.power, Some(PtValue::Fixed(2)));
        assert_eq!(def.base_characteristics.toughness, Some(PtValue::Fixed(2)));
    }

    #[test]
    fn register_all_phase1_samples_succeeds() {
        let mut r = CardRegistry::new();
        let ids = register_all_phase1_samples(&mut r);
        assert_ne!(ids.mountain, ids.forest);
        assert_ne!(ids.grizzly_bears, ids.lightning_bolt);
        assert_eq!(r.len(), 4);
    }
}
