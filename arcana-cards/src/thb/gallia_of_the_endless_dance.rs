//! Gallia of the Endless Dance — `{R}{G}` 2/2 Legendary Creature — Satyr.
//!
//! Oracle:
//! * Haste
//! * "Other Satyrs you control get +1/+1 and have haste." — wired via a
//!   `SelfEntersBattlefield` trigger that installs a `filtered_pump` (+1/+1,
//!   layer 7c) plus a `filtered_keyword` (Haste, layer 6) over Satyrs you
//!   control, anchored to Gallia with `Duration::WhileSourceOnBattlefield`.
//!   (The filter matches base characteristics, so Gallia herself is included —
//!   a documented minor "OTHER" self-inclusion fidelity gap.)
//! * "Whenever you attack with three or more creatures, you may discard a card
//!   at random. If you do, draw two cards." — GAP: no "attack with N or more
//!   creatures" trigger-count variant.
//!
//! Haste is a base keyword.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gallia of the Endless Dance");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: trigger "Whenever you attack with three or more creatures, you may
    //      discard a card at random; if you do, draw two cards" — no
    //      count-of-attackers TriggerCondition variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_satyr_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "Satyrs you control get +1/+1 and have haste", anchored to
/// Gallia and lasting until she leaves the battlefield.
fn install_satyr_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let satyr = reg
        .interner()
        .lookup("Satyr")
        .expect("Satyr interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(satyr);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
