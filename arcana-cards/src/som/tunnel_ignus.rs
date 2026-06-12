//! Tunnel Ignus — `{1}{R}` 2/1 red Elemental. "Whenever a land enters
//! under an opponent's control, if that player had another land enter
//! the battlefield under their control this turn, this creature deals
//! 3 damage to that player."
//! "That player" is the entering land's controller (via
//! `trig.entering_object()`); the "another land this turn" clause is
//! checked at resolution via the event log — the triggering land's own
//! entry is already logged, so a count of 2+ means another entered.
//! GAP (fidelity): a true intervening-if is also checked at stack-add;
//! this gate runs only at resolution.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tunnel Ignus");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter {
                        types_any: Some(TypeLine::LAND.into()),
                        ..Default::default()
                    }
                    .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                // "if that player had another land enter this turn" is
                // checked at resolution in the effect fn (the iif hook
                // can't see the entering land's controller).
                intervening_if: None,
                effect: deal_damage_to_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_damage_to_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" — the controller of the land that just entered.
    let Some(them) = trig.entering_object()
        .and_then(|id| state.objects.get(id))
        .map(|o| o.controller) else { return Vec::new(); };
    // "if that player had another land enter the battlefield under their
    // control this turn" — the triggering land's own entry is already in
    // the log, so 2+ entries means another land entered.
    let lands_under_them = ObjectFilter {
        types_any: Some(TypeLine::LAND.into()),
        ..Default::default()
    }
    .controlled_by(ControllerConstraint::You);
    if script::entered_this_turn_matching(state, &lands_under_them, them) < 2 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(them),
        amount: 3,
        source: trig.source,
    }]
}
