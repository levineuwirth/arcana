//! Liege of the Pit — `{5}{B}{B}{B}` 7/7 Demon with Flying and Trample.
//! "At the beginning of your upkeep, sacrifice a creature other than
//!  this creature. If you can't, this creature deals 7 damage to you.
//!  Morph {B}{B}{B}{B}."
//!
//! The upkeep tax sacrifices a creature you control. The "other than
//! this creature" self-exclusion and the "if you can't, deal 7 to you"
//! alternative both lack expressible primitives (no self-exclusion in a
//! Sacrifice filter, no can't-pay branch). Morph has no KeywordAbility
//! variant. Those are GAP'd; the sacrifice is wired (mirrors Lord of
//! the Pit).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liege of the Pit");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Morph {B}{B}{B}{B} — no KeywordAbility::Morph variant.
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            // GAP: "other than this creature" self-exclusion not expressible
            //      in a Sacrifice filter; "if you can't, deals 7 damage to
            //      you" has no can't-pay branch primitive.
            effect: upkeep_sacrifice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        count: 1,
    }]
}
