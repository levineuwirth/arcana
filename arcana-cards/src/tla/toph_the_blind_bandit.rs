//! Toph, the Blind Bandit — `{2}{G}` */3 Legendary Human Warrior Ally.
//!
//! Oracle:
//! * When Toph enters, earthbend 2. — GAP: "earthbend N" is a bespoke keyword
//!   action (turn a target land into a 0/0 creature with haste that's still a
//!   land, put N +1/+1 counters on it, return-on-death-or-exile) with no
//!   expressible `Effect` primitive.
//! * Toph's power is equal to the number of +1/+1 counters on lands you
//!   control. (Installed at Layer 7a via an ETB self-CDA — `self_pt_cda`
//!   returning `(sum of +1/+1 counters on your lands, 3)`. Asymmetric `*`/3;
//!   the `*` axis sums a counter kind over lands you control (no subtype
//!   name), so the no-registry compute resolves it.)

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toph, the Blind Bandit");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: earthbend_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            counters_on_your_lands,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn counters_on_your_lands(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n: i32 = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.is_land() && o.controller == who)
        .map(|o| o.count_counters(CounterKind::PlusOnePlusOne) as i32)
        .sum();
    (n, 3)
}

fn earthbend_two(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "earthbend 2" — animate a target land into a 0/0 land creature with
    // haste, add two +1/+1 counters, and a return-on-death delayed trigger.
    // No single Effect primitive expresses this keyword action.
    Vec::new()
}
