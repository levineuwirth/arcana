//! Souls of the Lost — `{1}{B}` */*+1 Spirit.
//! "As an additional cost to cast this spell, discard a card or sacrifice a
//!  permanent. Fathomless descent — Souls of the Lost's power is equal to the
//!  number of permanent cards in your graveyard and its toughness is equal to
//!  that number plus 1."

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Souls of the Lost");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/*+1` — power is a CDA (`*`), toughness is `*+1`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };
    // GAP: "As an additional cost to cast this spell, discard a card or sacrifice
    // a permanent." — additional cast cost, not a triggered/activated ability and
    // not expressible on this card class.
    // Fathomless descent — power = permanent cards in your graveyard, toughness
    // = that number plus 1 — installed as a Layer 7a self-CDA on ETB.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: power = permanent cards in your graveyard, toughness =
/// that number plus 1.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = number of permanent cards in your graveyard; toughness = +1.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Graveyard(who))
        .filter(|o| o.characteristics.types.is_permanent())
        .count() as i32;
    (n, n + 1)
}
