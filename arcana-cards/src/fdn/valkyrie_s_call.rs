//! Valkyrie's Call — `{3}{W}{W}` enchantment.
//! "Whenever a nontoken, non-Angel creature you control dies, return
//! that card to the battlefield under its owner's control with a
//! +1/+1 counter on it. It has flying and is an Angel in addition to
//! its other types."
//!
//! GAP: "non-Angel" cannot be expressed (the demonstrated ObjectFilter
//! surface has no subtype EXCLUSION), so the trigger also fires for
//! Angels. GAP: the permanent "has flying and is an Angel" grant has
//! no permanent-duration grant and no add-subtype effect, so that
//! rider is omitted; the return + counter are faithful.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valkyrie's Call");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "non-Angel" — subtype exclusion is not available on
                // the demonstrated filter surface; fires for Angels too.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: valkyrie_raise,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…return that card to the battlefield under its owner's control
/// with a +1/+1 counter on it." GAP: the permanent flying/Angel grant
/// is omitted (no permanent-duration keyword grant, no add-subtype
/// effect).
fn valkyrie_raise(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: dead },
        Effect::AddCounters {
            target: dead,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
