//! Pathbreaker Ibex — `{4}{G}{G}` 3/3 green Goat. "Whenever this
//! creature attacks, creatures you control gain trample and get +X/+X
//! until end of turn, where X is the greatest power among creatures
//! you control."
//!
//! The trample grant is applied to each creature you control via
//! `ForEach` + `GrantKeyword`. The +X/+X pump is GAP'd: X is the
//! GREATEST power among creatures you control, and there is no
//! `script::` helper to compute a max-over-set amount — only
//! `power_of` for a single id. Emitting a literal where the text is
//! dynamic would be a wrong card, so the pump is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pathbreaker Ibex");
    let goat = reg.interner_mut().intern("Goat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    use arcana_core::targets::ObjectFilter;
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    // GAP: +X/+X pump where X is the greatest power among creatures you
    // control — no script helper computes a max-over-set amount, only
    // power_of for a single id. Trample grant is applied below.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: arcana_core::objects::NULL_OBJECT_ID,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        }),
    }]
}
