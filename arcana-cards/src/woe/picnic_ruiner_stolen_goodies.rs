//! Picnic Ruiner // Stolen Goodies — `{1}{R}` Goblin Rogue 2/2
//!
//! Creature face:
//! Whenever this creature attacks while you control a creature with power 4 or greater,
//! this creature gains double strike until end of turn.
//! (GAP: "intervening-if: you control a creature with power 4 or greater" at attack trigger —
//!  TriggerCondition::Attacks exists but intervening_if with script::count_matching
//!  power-filter is not directly representable as a fn pointer in TriggeredAbilityDef;
//!  using a non-nil effect with runtime guard instead.)
//!
//! Adventure face "Stolen Goodies" (`{3}{G}` Sorcery):
//! Distribute three +1/+1 counters among any number of target creatures you control.
//! (GAP: "distribute N counters among any number of targets" — TargetCount::Any on
//!  creature targets with distributed counter allocation is not expressible as a single
//!  Effect variant; emitting Vec::new().)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Picnic Ruiner");
    let goblin_sub = reg.interner_mut().intern("Goblin");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin_sub);
    subtypes.0.insert(rogue_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid creature cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Adventure face "Stolen Goodies" — sorcery {3}{G}
    let adv_name = reg.interner_mut().intern("Stolen Goodies");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid adventure cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Distribute three +1/+1 counters among any number of target creatures you control.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: stolen_goodies_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn attack_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Intervening-if: you control a creature with power 4 or greater
    let big_creature_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_min_power(4);
    let count = script::count_matching(state, &big_creature_filter, trig.controller);
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    }]
}

fn stolen_goodies_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "distribute three +1/+1 counters among any number of target creatures you control"
    // — distributing a fixed pool of counters across a variable number of chosen targets
    // is not expressible with current engine Effect variants (AddCounters applies a fixed
    // count to one target; no "distribute N among M targets" variant exists).
    Vec::new()
}
