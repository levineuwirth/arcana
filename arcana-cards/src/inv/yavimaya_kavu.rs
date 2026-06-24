//! Yavimaya Kavu — `{2}{R}{G}` */* Kavu (red-green).
//!
//! Power = the number of red creatures on the battlefield; toughness = the
//! number of green creatures on the battlefield. This is an ASYMMETRIC CDA
//! (different filter per axis), wired at Layer 7a via a SelfEntersBattlefield
//! self_pt_cda whose compute returns `(red_creatures, green_creatures)`. The
//! color filters carry no subtype name, so the no-registry compute fn can
//! count them directly. Bones are `*/*` (`PtValue::Star`).

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
    let name = reg.interner_mut().intern("Yavimaya Kavu");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA: power = red creatures on the battlefield; toughness = green.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

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

/// Layer 7a self-CDA: power = red creatures, toughness = green creatures
/// (both across the whole battlefield — no controller restriction).
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            red_green_creature_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = red creatures on the battlefield; toughness = green creatures on
/// the battlefield (all controllers).
fn red_green_creature_pt(s: &GameState, _source: ObjectId) -> (i32, i32) {
    // BASE-characteristics count — recursion-proof at Layer 7a. Must NOT use
    // the layer-aware `script::count_matching` here: matching a battlefield
    // creature recomputes its characteristics (all layers, incl. THIS 7a
    // CDA), so counting battlefield creatures from inside the CDA re-enters
    // the layer system and overflows the stack. Reading `o.characteristics`
    // directly is the stored base, never computed.
    let count = |c: ColorSet| s.objects.iter()
        .filter(|o| o.zone.is_battlefield()
            && o.characteristics.types.is_creature()
            && (o.characteristics.colors & c).0 != 0)
        .count() as i32;
    (count(ColorSet::red()), count(ColorSet::green()))
}
