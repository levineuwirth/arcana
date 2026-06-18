//! Brainstealer Dragon — `{5}{B}{B}` 6/6 black Dragon Horror with Flying.
//!
//! Oracle:
//! * Flying.
//! * At the beginning of your end step, exile the top card of each opponent's
//!   library. You may play those cards for as long as they remain exiled. If you
//!   cast a spell this way, you may spend mana as though it were mana of any
//!   color to cast it.
//! * Whenever a nonland permanent an opponent owns enters the battlefield under
//!   your control, they lose life equal to its mana value.
//!
//! The Flying keyword is a base characteristic. Both triggered abilities are
//! wired with their correct trigger conditions, but their effects are GAP'd:
//! there is no demonstrated primitive for "exile the top card of EACH OPPONENT's
//! library with play-permission" (`ImpulseExile` only acts on your own library),
//! and "lose life equal to its mana value" has no mana-value scripting helper in
//! the demonstrated `script::` surface.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brainstealer Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_steal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: opponent_loses_mana_value,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_steal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of each opponent's library, you may play them
    // (and spend mana as any color)" — no demonstrated primitive exiles from an
    // OPPONENT's library with play-permission (ImpulseExile acts on your own
    // library only); the any-color spend rider is also unmodeled.
    Vec::new()
}

fn opponent_loses_mana_value(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "they lose life equal to its mana value" — no mana-value scripting
    // helper in the demonstrated script:: surface, and the owner ("they") is not
    // derivable from the demonstrated accessors for a controller-change ZoneChange.
    Vec::new()
}
