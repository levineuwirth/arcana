//! Raphael, Ninja Destroyer — `{2}{R}{R}` 4/4 Legendary Mutant Ninja Turtle.
//!
//! * "Raphael must be blocked if able." — a static combat-restriction
//!   on attackers; not expressible with the demonstrated primitives.
//!   GAP'd below.
//! * Enrage — Whenever Raphael is dealt damage, add that much {R}.
//!   (The "you don't lose this mana as steps and phases end" rider is
//!   a fidelity gap — the engine drains mana normally.)
//!
//! `Enrage` is not a `KeywordAbility` variant in the usable surface; it
//! is decomposed into the triggered ability below, so `keywords` is
//! empty.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raphael, Ninja Destroyer");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Raphael must be blocked if able." — a static must-be-blocked
    // combat restriction; no demonstrated primitive expresses it.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
            intervening_if: None,
            effect: enrage_add_red,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enrage_add_red(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0) as usize;
    if n == 0 {
        return Vec::new();
    }
    // GAP (fidelity): "you don't lose this mana as steps and phases end"
    // — the engine drains mana on phase/step end as normal.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); n],
    }]
}
