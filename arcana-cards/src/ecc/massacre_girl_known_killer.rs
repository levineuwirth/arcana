//! Massacre Girl, Known Killer — `{2}{B}{B}` 4/4 legendary black Human
//! Assassin.
//!
//! * Menace.
//! * Creatures you control have wither. — GAP: a static continuous ability
//!   granting a keyword to your other creatures has no expressible
//!   primitive (no board-wide static keyword-grant effect); omitted.
//! * Whenever a creature an opponent controls dies, if its toughness was
//!   less than 1, draw a card.
//!
//! GAP (ability 3): the intervening-if "its toughness was less than 1"
//! tests the DYING creature's last-known toughness; the available
//! `conditions::`/`intervening_if` predicates only test the source/you,
//! and dropping the gate would over-fire (draw on every opponent-creature
//! death). The effect is GAP'd to avoid a materially wrong card; the
//! trigger condition shell is retained.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Massacre Girl, Known Killer");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: draw_if_low_toughness_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_if_low_toughness_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if its toughness was less than 1" — the dying creature's
    // last-known toughness can't be tested with the available predicates,
    // and dropping the gate would over-fire.
    Vec::new()
}
