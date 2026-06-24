//! Haughty Djinn — `{1}{U}{U}` */4 Djinn with Flying.
//!
//! Oracle:
//! * Flying — keyword.
//! * Haughty Djinn's power is equal to the number of instant and sorcery cards
//!   in your graveyard — a characteristic-defining ability wired at Layer 7a
//!   via a `SelfEntersBattlefield` `self_pt_cda`. Power `*` resolves to the
//!   count of instant/sorcery cards in the controller's graveyard; toughness is
//!   the printed fixed 4.
//! * Instant and sorcery spells you cast cost {1} less to cast — installed as a
//!   Layer 6 `spell_cost_modifier` (caster = You, generic_delta = -1) by the
//!   same ETB trigger, lasting `WhileSourceOnBattlefield`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haughty Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/4` — power is a CDA (`*` = instant/sorcery cards in your
        // graveyard), toughness is the printed fixed 4. Resolved at Layer 7a by
        // install_effects.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_effects,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Install both continuous effects on ETB: the Layer 7a self-CDA (power =
/// instant/sorcery cards in your graveyard, toughness = 4) and the Layer 6
/// spell cost reduction (instant/sorcery you cast cost {1} less).
fn install_effects(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let spell_filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::self_pt_cda(
                trig.source,
                cda_pt,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::spell_cost_modifier(
                trig.source,
                spell_filter,
                ControllerConstraint::You,
                -1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

/// Power = number of instant and sorcery cards in your graveyard; toughness =
/// printed 4.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Graveyard(who))
        .filter(|o| {
            let t = o.characteristics.types;
            t.has(TypeLine::INSTANT) || t.has(TypeLine::SORCERY)
        })
        .count() as i32;
    (n, 4)
}
