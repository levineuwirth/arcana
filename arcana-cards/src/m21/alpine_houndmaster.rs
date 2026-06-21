//! Alpine Houndmaster — `{R}{W}` 2/2 Human Warrior.
//! "When this creature enters, you may search your library for a card
//! named Alpine Watchdog and/or a card named Igneous Cur, reveal them,
//! put them into your hand, then shuffle."
//! "Whenever this creature attacks, it gets +X/+0 until end of turn,
//! where X is the number of other attacking creatures."
//!
//! The ETB tutors both named cards into hand (two TutorToHand by name).
//! The attack trigger pumps the source by the number of OTHER attacking
//! creatures (all attacking creatures minus itself).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Alpine Houndmaster");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern the searched card names so the resolver's lookup resolves.
    let _watchdog = reg.interner_mut().intern("Alpine Watchdog");
    let _cur = reg.interner_mut().intern("Igneous Cur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_named,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_named(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let watchdog = reg.interner().lookup("Alpine Watchdog");
    let cur = reg.interner().lookup("Igneous Cur");
    // Two library searches each post a choice → wrap in a single Sequence.
    vec![Effect::Sequence(vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter {
                name: watchdog,
                ..ObjectFilter::default()
            },
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter {
                name: cur,
                ..ObjectFilter::default()
            },
            reveal: true,
        },
    ])]
}

fn attack_pump(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // X = number of OTHER attacking creatures (all attackers minus self).
    let attackers = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    );
    let x = attackers.saturating_sub(1);
    vec![Effect::Pump {
        target: trig.source,
        power: x as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
