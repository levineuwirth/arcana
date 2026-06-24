//! Nethergoyf — `{B}` */1+* Lhurgoyf.
//! "Nethergoyf's power is equal to the number of card types among cards
//! in your graveyard and its toughness is equal to that number plus 1."
//! Escape—{2}{B}, exile any number of other cards from your graveyard
//! with four or more card types among them.
//!
//! P/T are captured as `*` / `*+1` (PtValue::Star / StarPlus(1)); the
//! characteristic-defining ability (power = # card types in your graveyard,
//! toughness = that + 1) is wired at Layer 7a via a `self_pt_cda` installed
//! on ETB. Escape is not in the usable keyword surface — GAP'd.

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
    let name = reg.interner_mut().intern("Nethergoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA: power `*` = # card types in your graveyard, toughness `*+1`,
        // resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        // GAP: Escape—{2}{B}, exile any number of other cards … — not in
        // the usable keyword surface.
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

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = number of card types among cards in YOUR graveyard;
// toughness = that number plus 1.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    // The eight real card types (Kindred is not a card type for this count).
    const CARD_TYPE_BITS: u16 = TypeLine::CREATURE
        | TypeLine::INSTANT
        | TypeLine::SORCERY
        | TypeLine::ENCHANTMENT
        | TypeLine::ARTIFACT
        | TypeLine::LAND
        | TypeLine::PLANESWALKER
        | TypeLine::BATTLE;
    let mut seen: u16 = 0;
    for o in s.objects.objects_in_zone(Zone::Graveyard(who)) {
        seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
    }
    let n = seen.count_ones() as i32;
    (n, n + 1)
}
