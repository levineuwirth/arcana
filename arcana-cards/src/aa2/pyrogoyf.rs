//! Pyrogoyf — `{3}{R}` */1+* Lhurgoyf.
//!
//! Oracle:
//! * Pyrogoyf's power is equal to the number of card types among cards
//!   in all graveyards and its toughness is equal to that number plus 1.
//!   — a characteristic-defining ability. No `script::` helper counts
//!   card types across all graveyards and no primitive sets base P/T
//!   from such a count, so P/T are left as `*` / `*+1` (the CDA itself
//!   is GAP'd).
//! * Whenever this creature or another Lhurgoyf creature you control
//!   enters, that creature deals damage equal to its power to any
//!   target. — expressed via a battlefield ZoneChange watching
//!   Lhurgoyf creatures you control (the filter matches this creature
//!   too, covering the "this creature or another" wording).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pyrogoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA — P/T = number of card types among all graveyards
        // (+1 for toughness). Left as `*` / `*+1`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };

    let lhurgoyf_filter = script::subtype_filter(reg, "Lhurgoyf")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: lhurgoyf_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: entered_deal_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn entered_deal_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entering = trig.entering_object().unwrap_or(trig.source);
    let amount = script::power_of(state, entering).max(0) as u32;
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entering,
        target: dt,
        amount,
    }]
}
