//! Sidisi, Undead Vizier — `{3}{B}{B}` 4/6 Legendary Zombie Snake.
//! Deathtouch; Exploit (ETB: you may sacrifice a creature); "When Sidisi exploits
//! a creature, you may search your library for a card, put it into your hand, then
//! shuffle."
//!
//! Deathtouch is a base keyword. Exploit has no dedicated TriggerCondition; the
//! ETB sacrifice and the separate "exploits a creature" trigger are collapsed into
//! a single ETB trigger that sacrifices a creature (best-effort) and tutors a card
//! to hand. The "you may" optionality on both the sacrifice and the search is GAP'd
//! (no optional-sacrifice cost shape).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sidisi, Undead Vizier");
    let zombie = reg.interner_mut().intern("Zombie");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exploit_then_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exploit_then_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may" optionality on both the exploit-sacrifice and the search is
    // not expressible; the "exploits a creature" trigger has no dedicated condition,
    // so the sacrifice and the tutor payoff are combined into the ETB.
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            count: 1,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: ObjectFilter::default(),
            reveal: false,
        },
    ]
}
