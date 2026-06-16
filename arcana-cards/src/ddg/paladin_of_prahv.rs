//! Paladin of Prahv — `{4}{W}{W}` 3/4 Human Knight. Whenever it deals damage,
//! you gain that much life. Forecast — `{1}{W}, Reveal this card from your
//! hand: Whenever target creature deals damage this turn, you gain that much
//! life.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paladin of Prahv");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let self_name = reg.interner().lookup("Paladin of Prahv");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: Forecast — `{1}{W}, Reveal this card from your hand: ...` — there is
    // no reveal-from-hand activation cost field, and the granted "whenever
    // target creature deals damage this turn, you gain that much life" delayed
    // trigger is not expressible here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                // Restrict the damage source to this card by name (closest
                // expressible approximation of "this creature deals damage").
                source_filter: ObjectFilter {
                    name: self_name,
                    ..ObjectFilter::default()
                },
                target_filter: TargetFilter::AnyTarget,
                combat_only: false,
            },
            intervening_if: None,
            effect: gain_that_much,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_that_much(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
