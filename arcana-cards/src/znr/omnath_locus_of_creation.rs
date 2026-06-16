//! Omnath, Locus of Creation — `{R}{G}{W}{U}` 4/4 Legendary Creature — Elemental
//! (colors G/R/U/W). Keyword: Landfall.
//!
//! Oracle text:
//! * When Omnath enters, draw a card.
//! * Landfall — Whenever a land you control enters, you gain 4 life if this is
//!   the first time this ability has resolved this turn. If it's the second
//!   time, add {R}{G}{W}{U}. If it's the third time, Omnath deals 4 damage to
//!   each opponent and each planeswalker you don't control.
//!
//! The ETB draw is wired faithfully. The Landfall ability uses the canonical
//! landfall condition (a land you control entering the battlefield), but its
//! resolution branches on "the first / second / third time this ability has
//! resolved THIS TURN" — a per-turn resolution-ordinal gate. The listed
//! primitives expose no per-turn resolution counter for an ability, so which of
//! the three mutually-exclusive branches to run cannot be determined. Firing
//! any one branch unconditionally (or all of them) would be materially wrong,
//! so the landfall resolution is GAP'd whole (see `landfall_resolve`).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omnath, Locus of Creation");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // Landfall is not a standalone KeywordAbility variant — it is the
        // ability-word label on the triggered ability below.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When Omnath enters, draw a card."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Landfall — Whenever a land you control enters, …"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: the controller draws one card.
fn etb_draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

/// Landfall resolution.
fn landfall_resolve(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the three effects (gain 4 life / add {R}{G}{W}{U} / deal 4 damage to
    // each opponent and each planeswalker you don't control) are gated on "the
    // first / second / third time this ability has resolved this turn". The
    // listed primitives provide no per-turn ability-resolution-ordinal counter,
    // so the correct branch is undeterminable. Each branch on its own is
    // expressible (GainLife; AddMana of {R}{G}{W}{U}; a per-opponent/PW damage
    // Sequence), but firing any one unconditionally — or all three — would be a
    // materially wrong card, so the whole resolution is GAP'd.
    Vec::new()
}
