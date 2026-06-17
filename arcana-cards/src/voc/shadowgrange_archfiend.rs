//! Shadowgrange Archfiend — `{6}{B}` 8/4 Demon with Madness.
//! "When this creature enters, each opponent sacrifices a creature with
//! the greatest power among creatures they control. You gain life equal
//! to the greatest power among creatures sacrificed this way."
//! Madness—{2}{B}, Pay 8 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shadowgrange Archfiend");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Madness {2}{B} + Pay 8 life is a non-mana alternative cast
        // cost; not a modeled KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_each_opp_sacrifices,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_each_opp_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each opponent sacrifices a creature. The "greatest power" constraint
    // on which creature is chosen, and the life-gain equal to the greatest
    // power sacrificed, are not expressible — GAP'd.
    let opps = script::opponents(state, trig.controller);
    let sacs: Vec<Effect> = opps
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect();
    // Multiple choice-posting Sacrifice effects must be wrapped in a
    // single Sequence (engine choice-parking invariant).
    vec![Effect::Sequence(sacs)]
}
