//! The Duke of Midrange — `{2}{B}{R}{G}` */1+* Legendary Lhurgoyf Wizard.
//! When The Duke of Midrange enters the battlefield, choose one of
//! Thoughtseize, Lightning Bolt, or Abrupt Decay. Cast a copy of the
//! chosen card without paying its mana cost.
//! The Duke of Midrange's power is equal to the number of card types
//! among cards in all graveyards and its toughness is equal to that
//! number plus 1.
//!
//! The `*` / `1+*` characteristic-defining P/T is recorded as
//! `PtValue::Star` / `PtValue::StarPlus(1)` and wired at Layer 7a via a
//! SelfEntersBattlefield self_pt_cda (Polygoyf pattern): the compute ORs the
//! TypeLine bits of every card in every graveyard, counts the distinct
//! card-type bits `n`, and sets base P/T to `(n, n + 1)`.
//!
//! GAP (ETB cast-a-copy): "choose one of Thoughtseize, Lightning Bolt, or
//! Abrupt Decay; cast a copy of the chosen card without paying its mana
//! cost." No expressible effect mints/casts a free copy of a specific named
//! card selected modally; `Effect::CopySpell` needs an on-stack spell.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::{Zone, ZoneKind};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Duke of Midrange");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_choose_and_cast_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
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

// Power = number of card types among cards in all graveyards;
// toughness = that number plus 1.
fn cda_pt(s: &GameState, _source: ObjectId) -> (i32, i32) {
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
    for o in s.objects.objects_in_zone_kind(ZoneKind::Graveyard) {
        seen |= o.characteristics.types.0 & CARD_TYPE_BITS;
    }
    let n = seen.count_ones() as i32;
    (n, n + 1)
}

fn etb_choose_and_cast_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one of Thoughtseize, Lightning Bolt, or Abrupt Decay;
    // cast a copy of the chosen card without paying its mana cost." No
    // expressible effect mints/casts a free copy of a specific named card
    // selected modally; Effect::CopySpell needs an on-stack spell.
    Vec::new()
}
