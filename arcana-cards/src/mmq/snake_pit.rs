//! Snake Pit — `{3}{G}` enchantment.
//! "Whenever an opponent casts a blue or black spell, you may create a
//! 1/1 green Snake creature token."
//!
//! A color-filtered opponent `SpellCast` trigger. Fidelity note: the
//! "you may" is resolved as always-yes (no free may-gate exists; the
//! token is always created).

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Snake Pit");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_colors(ColorSet::blue() | ColorSet::black()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: make_snake,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may create a 1/1 green Snake creature token." (may resolved
/// as always-yes.)
fn make_snake(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Snake") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: reg.interner().lookup("Snake").unwrap_or_default(),
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
