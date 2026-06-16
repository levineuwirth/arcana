//! Sphinx of the Revelation — `{3}{W}{U}` 4/5 Artifact Creature — Sphinx.
//! Flying, lifelink.
//! "Whenever you gain life, you get that many {E} (energy counters)."
//! "{W}{U}{U}, {T}, Pay X {E}: Draw X cards."
//!
//! Bones + the two keywords (Flying, Lifelink) are faithful. The life-gain
//! trigger fires on LifeGained, but the energy gained is DYNAMIC ("that many")
//! and no PendingTrigger accessor exposes the gained life amount (only
//! damage_amount() exists), so the GainEnergy amount cannot be computed —
//! GAP'd whole rather than hardcoding a literal. The activated ability's mana
//! ({W}{U}{U}) + tap cost is modeled, but "Pay X {E}" is not an expressible
//! ActivationCost field (energy spend is not a cost), and the "draw X cards"
//! depends on that X — both GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx of the Revelation");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: gain_energy_that_many,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{U}, {T}, Pay X {E}: Draw X cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_x,
            }),
    )
}

fn gain_energy_that_many(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you get that many {E}" is dynamic on the gained-life amount, but no
    //      PendingTrigger accessor exposes the LifeGained amount; cannot compute.
    Vec::new()
}

fn draw_x(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Pay X {E}: Draw X cards" — energy spend is not an ActivationCost
    //      field (no {E}-pay cost), so X is undetermined and the draw cannot be
    //      modeled.
    Vec::new()
}
