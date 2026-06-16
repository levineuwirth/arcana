//! Angelic Skirmisher — `{4}{W}{W}` 4/4 Angel with Flying.
//!
//! * Flying / Vigilance — base keyword line. (Scryfall lists both; "Flying"
//!   is the printed first line and Vigilance is part of the parsed keyword
//!   set — both are demonstrated `KeywordAbility` variants.)
//! * "At the beginning of each combat, choose first strike, vigilance, or
//!   lifelink. Creatures you control gain that ability until end of turn."
//!   The trigger (each combat) is expressible, but there is no demonstrated
//!   primitive for "choose one of three keywords" and no board-wide
//!   "creatures you control gain <chosen keyword>" grant. Effect GAP'd; the
//!   trigger is still wired so the catalog records it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angelic Skirmisher");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: choose_keyword_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn choose_keyword_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no demonstrated primitive to choose one of {first strike,
    // vigilance, lifelink} and grant it board-wide to creatures you control.
    Vec::new()
}
