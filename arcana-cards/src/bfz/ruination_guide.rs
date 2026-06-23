//! Ruination Guide — `{2}{U}` 3/2 colorless (Devoid) Eldrazi Drone.
//!
//! Oracle:
//! * Devoid — the card has no color; modeled as `ColorSet::colorless()`.
//! * Ingest — "Whenever this creature deals combat damage to a player, that
//!   player exiles the top card of their library." GAP: there is no
//!   self-only DamageDealt source restriction AND no exile-top-of-library
//!   effect, so the Ingest trigger is unexpressible.
//! * "Other colorless creatures you control get +1/+0." — a static
//!   continuous anthem, now wired as a `SelfEntersBattlefield` trigger that
//!   installs a `ContinuousEffect::filtered_pump` over colorless creatures
//!   you control (glorious_anthem idiom). NOTE: `filtered_pump` matches base
//!   characteristics, so the "OTHER" exclusion isn't expressed — Ruination
//!   Guide is itself colorless and self-includes (documented minor gap).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Ruination Guide");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    // GAP: Ingest — no self-only combat-damage trigger restriction and no
    // exile-top-of-library effect.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_colorless_anthem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: install "colorless creatures you control get +1/+0" anchored to
/// Ruination Guide, lasting while it is on the battlefield. Colorless is
/// expressed as "no color present" via `without_colors` of all five.
fn install_colorless_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .without_colors(
            ColorSet::white()
                | ColorSet::blue()
                | ColorSet::black()
                | ColorSet::red()
                | ColorSet::green(),
        );
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
