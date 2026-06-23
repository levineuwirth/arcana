//! Kodama of the East Tree — `{4}{G}{G}` 6/6 Legendary Creature — Spirit.
//!
//! * Reach — evergreen keyword.
//! * Whenever another permanent you control enters, if it wasn't put onto the
//!   battlefield with this ability, you may put a permanent card with equal or
//!   lesser mana value from your hand onto the battlefield. — `ZoneChange`
//!   (a permanent you control entering, from anywhere, to the battlefield);
//!   the effect is `PutFromHandOntoBattlefield` over your hand's permanent
//!   cards. The optional "you may" is handled by the put's own
//!   choose-or-decline pick.
//!   GAP: the "with equal or lesser mana value [than the entering permanent]"
//!   restriction (the put filter can't be sized dynamically by the triggering
//!   permanent's mana value) and the "if it wasn't put onto the battlefield
//!   with this ability" recursion guard (no such guard primitive here).
//! * Partner — not in the usable keyword surface; GAP (a deck-construction
//!   keyword with no in-game rules to wire).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kodama of the East Tree");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: put_permanent_from_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "you may put a permanent card ... from your hand onto the battlefield."
/// GAP: mana-value restriction (≤ the triggering permanent's mv) and the
/// "not put with this ability" recursion guard are unmodeled.
fn put_permanent_from_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent(),
        tapped: false,
    }]
}
