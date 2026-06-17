//! Magnanimous Magistrate — `{5}{W}` 3/4 Creature — Human Advisor (white).
//!
//! * "This creature enters with five reprieve counters on it." — modeled as a
//!   SelfEnters trigger that adds five Named("reprieve") counters to itself.
//! * "Whenever another nontoken creature you control dies, if its mana value
//!   was 1 or greater, you may remove that many reprieve counters from this
//!   creature. If you do, return that card to the battlefield under its
//!   owner's control." — the "remove that-many (= the dead creature's mana
//!   value) reprieve counters, then return THAT card" chain requires a
//!   dynamic counter cost tied to a may-gate and a per-card reanimation of
//!   the specific dying card; not expressible. GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magnanimous Magistrate");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    reg.interner_mut().intern("reprieve");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: the death-triggered "remove that-many reprieve counters, then
    // return that card" chain (dynamic counter cost + per-card reanimation).
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None,
        effect: enters_with_reprieve,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn enters_with_reprieve(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(reprieve) = reg.interner().lookup("reprieve") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(reprieve),
        count: 5,
    }]
}
