//! Cultivator of Blades — `{3}{G}{G}` 1/1 Elf Artificer.
//! Fabricate 2 (ETB: put two +1/+1 counters on it OR create two 1/1 Servo
//! tokens). "Whenever this creature attacks, you may have other attacking
//! creatures get +X/+X until end of turn, where X is this creature's power."

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
    let name = reg.interner_mut().intern("Cultivator of Blades");
    let elf = reg.interner_mut().intern("Elf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Fabricate N is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Fabricate 2 ETB — modal "put two +1/+1 counters on it OR create
            // two Servo tokens" choice has no expressible primitive at ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: fabricate_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_other_attackers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn fabricate_two(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Fabricate 2 — controller's choice of (+1/+1 counters on self) vs
    // (create two Servos) is not modeled by an available choice primitive.
    Vec::new()
}

fn pump_other_attackers(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "other attacking creatures get +X/+X where X = this creature's power" —
    // no script helper enumerates the set of currently-attacking creatures, and
    // the "you may" + dynamic X over a combat-derived set is unexpressible.
    Vec::new()
}
