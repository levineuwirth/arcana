//! Asari Captain — `{3}{R}{W}` 4/3 Human Samurai with Haste.
//! "Whenever a Samurai or Warrior you control attacks alone, it gets +1/+0
//! until end of turn for each Samurai or Warrior you control."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Asari Captain");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    // intern the watched subtypes so the resolver can rebuild the filter.
    let samurai_sym = reg.interner_mut().intern("Samurai");
    let warrior_sym = reg.interner_mut().intern("Warrior");
    let watch_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![samurai_sym, warrior_sym]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::AttacksAlone {
                filter: watch_filter,
            },
            intervening_if: None,
            effect: pump_lone_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn samurai_or_warrior_filter(reg: &CardRegistry) -> ObjectFilter {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Samurai") {
        syms.push(s);
    }
    if let Some(w) = reg.interner().lookup("Warrior") {
        syms.push(w);
    }
    ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(syms)
}

fn pump_lone_attacker(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.lone_attacker() else {
        return Vec::new();
    };
    let n = script::count_matching(state, &samurai_or_warrior_filter(reg), trig.controller);
    vec![Effect::Pump {
        target: id,
        power: n as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
