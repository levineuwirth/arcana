//! Pterafractyl — `{X}{G}{U}` 1/0 green/blue Dinosaur Fractal with Flying.
//!
//! Oracle:
//! * Flying.
//! * This creature enters with X +1/+1 counters on it. (CR 121.6a)
//! * When this creature enters, you gain 2 life.
//!
//! The keyword lands on `characteristics.keywords`; the ETB life-gain is a
//! plain non-targeted triggered ability.
//!
//! GAP: "enters with X +1/+1 counters on it" — the enters-with-counters
//! face spec is not part of the documented effect/keyword surface, so the
//! ETB counter placement is omitted. The bones P/T (1/0) are transcribed
//! verbatim.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Pterafractyl");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let fractal = reg.interner_mut().intern("Fractal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(fractal);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_gain_two_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_gain_two_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 2 }]
}
