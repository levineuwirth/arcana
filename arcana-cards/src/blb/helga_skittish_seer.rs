//! Helga, Skittish Seer — `{G}{W}{U}` 1/3 Legendary Creature — Frog Druid.
//!
//! * "Whenever you cast a creature spell with mana value 4 or greater, you
//!   draw a card, gain 1 life, and put a +1/+1 counter on Helga." — SpellCast
//!   trigger (you, creature, mv >= 4) → draw 1, gain 1, +1/+1 counter on self.
//! * "{T}: Add X mana of any one color, where X is Helga's power. Spend this
//!   mana only to cast ..." — a dynamic-amount, any-one-color mana ability
//!   with a spend restriction; the amount is X = power and there is no
//!   color-choice / spend-restriction mana primitive. GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Helga, Skittish Seer");
    let frog = reg.interner_mut().intern("Frog");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "{T}: Add X mana of any one color ... spend only on ..." — dynamic
    // any-one-color mana with a spend restriction is not expressible.
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SpellCast {
            filter: Some(ObjectFilter::creature().with_min_cmc(4)),
            caster: ControllerConstraint::You,
        },
        intervening_if: None,
        effect: on_big_creature,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn on_big_creature(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
