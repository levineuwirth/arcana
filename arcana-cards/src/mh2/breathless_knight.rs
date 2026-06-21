//! Breathless Knight — `{1}{W}{B}` 2/2 Spirit Knight with Flying and Lifelink.
//!
//! Oracle:
//!  * Flying, lifelink.
//!  * Whenever this creature or another creature you control enters, if that
//!    creature entered from a graveyard or you cast it from a graveyard, put a
//!    +1/+1 counter on this creature.
//!
//! Keywords are wired. The trigger fires on a creature you control entering,
//! but its intervening-if ("entered from a graveyard / you cast it from a
//! graveyard") has no condition helper. Firing unconditionally would be
//! materially wrong, so the effect is GAP'd while the trigger shape is recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breathless Knight");
    let spirit = reg.interner_mut().intern("Spirit");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: etb_graveyard_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_graveyard_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: intervening-if "that creature entered from a graveyard or you cast it
    // from a graveyard" has no condition helper; firing the +1/+1 counter
    // unconditionally would be materially wrong, so the effect is GAP'd.
    Vec::new()
}
