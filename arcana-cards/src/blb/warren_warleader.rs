//! Warren Warleader — `{2}{W}{W}` 4/4 Rabbit Knight.
//!
//! * Offspring {2} — "Offspring" is not a usable `KeywordAbility` variant (the
//!   additional-cost / token-copy-on-ETB mechanic is unmodeled), so it is
//!   GAP'd.
//! * "Whenever you attack, choose one — create a 1/1 white Rabbit token that's
//!   tapped and attacking; or attacking creatures you control get +1/+1 until
//!   end of turn." — a modal "choose one" on a TRIGGERED ability has no
//!   dispatch in the usable API (modal support is spell-ability only), so the
//!   modal payoff is GAP'd. The trigger itself ("whenever you attack") has no
//!   exact condition variant; the closest is a you-controlled `CreatureAttacks`
//!   (which over-fires once per attacker), so the body is GAP'd entirely.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Warren Warleader");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(knight);

    // GAP: Offspring {2} — not a usable keyword.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: attack_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" (create a tapped-and-attacking Rabbit token /
    // attacking creatures you control get +1/+1) is not expressible on a
    // triggered ability.
    Vec::new()
}
