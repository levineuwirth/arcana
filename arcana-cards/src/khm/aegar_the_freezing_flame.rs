//! Aegar, the Freezing Flame — `{1}{U}{R}` legendary 3/3 Giant Wizard.
//! "Whenever a creature or planeswalker an opponent controls is dealt
//! excess damage, if a Giant, Wizard, or spell you controlled dealt
//! damage to it this turn, draw a card."
//!
//! GAP: "excess damage" is not a catalog trigger — the closest
//! expressible match is the generic `DamageDealt` event, which fires on
//! ANY damage dealt to a matching object. The trigger therefore over-
//! fires relative to oracle text.
//! GAP: intervening_if clause ("if a Giant, Wizard, or spell you
//! controlled dealt damage to it this turn") is not expressible against
//! the engine's `intervening_if` surface; left as `None`.
//! GAP: target filter restricted to opponent-controlled creatures —
//! planeswalker filtering is not in the demonstrated `TypeLine` const
//! surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aegar, the Freezing Flame");
    let giant = reg.interner_mut().intern("Giant");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::CREATURE.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    combat_only: false,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Trigger resolution: the controller draws one card. The oracle gates
/// this on "excess damage" + an intervening-if check, neither of which
/// is expressible against the demonstrated trigger API — see the file
/// doc-comment GAP notes.
fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
