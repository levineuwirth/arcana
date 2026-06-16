//! Doubtless One — `{3}{W}` */* white Cleric Avatar.
//!
//! * "Doubtless One's power and toughness are each equal to the number
//!   of Clerics on the battlefield." — a characteristic-defining static
//!   (CR 604.3); there is no demonstrated CDA P/T primitive, so the base
//!   P/T is recorded as 0/0 and the dynamic value is GAP'd.
//! * "Whenever this creature deals damage, you gain that much life." —
//!   a `DamageDealt` trigger reading `trig.damage_amount()`. NOTE: the
//!   demonstrated `ObjectFilter` has no self-id predicate, so the
//!   `source_filter` cannot be pinned to this creature alone — closest
//!   available variant; restricting the source to "this creature" is a
//!   GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doubtless One");
    let cleric = reg.interner_mut().intern("Cleric");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cleric);
    subtypes.0.insert(avatar);

    // GAP: static "power and toughness are each equal to the number of
    // Clerics on the battlefield" — characteristic-defining P/T is not
    // expressible with the demonstrated API; base P/T recorded as 0/0.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::new(),
                target_filter: TargetFilter::AnyTarget,
                combat_only: false,
            },
            intervening_if: None,
            effect: gain_life_equal_to_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "you gain that much life" — gain life equal to the damage dealt.
fn gain_life_equal_to_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
