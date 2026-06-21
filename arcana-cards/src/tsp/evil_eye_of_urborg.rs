//! Evil Eye of Urborg — `{4}{B}` 6/3 Eye.
//!
//! Oracle:
//!  * Non-Eye creatures you control can't attack. (static — GAP)
//!  * Whenever this creature becomes blocked by a creature, destroy that
//!    creature.
//!
//! The "non-Eye creatures can't attack" line is a static attack restriction
//! with no expressible marker — GAP'd. The becomes-blocked trigger destroys
//! the blocking creature, read via trig.other_combatant().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evil Eye of Urborg");
    let eye = reg.interner_mut().intern("Eye");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eye);

    // GAP: "Non-Eye creatures you control can't attack" — static attack
    // restriction, not a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesBlocked,
            intervening_if: None,
            effect: destroy_blocker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn destroy_blocker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: id }]
}
