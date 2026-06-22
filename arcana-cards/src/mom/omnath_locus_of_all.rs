//! Omnath, Locus of All — `{W}{U}{B/P}{R}{G}` 4/4 Legendary Phyrexian
//! Elemental (all five colors).
//!
//! Oracle text:
//! * If you would lose unspent mana, that mana becomes black instead.
//! * At the beginning of your first main phase, look at the top card of
//!   your library. You may reveal that card if it has three or more
//!   colored mana symbols in its mana cost. If you do, add three mana in
//!   any combination of its colors and put it into your hand. If you
//!   don't reveal it, put it into your hand.
//!
//! Implemented: the bones (4/4 Legendary Phyrexian Elemental in WUBRG)
//! and the first-main-phase trigger SHAPE is wired.
//!
//! GAP: "If you would lose unspent mana, that mana becomes black" is a
//! mana-replacement effect with no representation — omitted.
//! GAP: the first-main-phase trigger's body inspects the top card's
//! colored-pip count and conditionally adds three mana of its colors
//! then puts it into hand — the look-top + pip-count gate + add-mana-of-
//! its-colors composite is not expressible, so the resolver returns no
//! effects.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omnath, Locus of All");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B/P}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PreCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: first_main_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn first_main_dig(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: look at top card; reveal if 3+ colored pips → add 3 mana of
    // its colors and draw it; else put into hand. The colored-pip gate
    // and "add mana of its colors" are not expressible.
    Vec::new()
}
