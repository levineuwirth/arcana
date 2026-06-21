//! Servant of the Stinger — `{1}{B}` 1/3 Human Warlock with Deathtouch.
//! "Whenever this creature deals combat damage to a player, if you've
//!  committed a crime this turn, you may sacrifice this creature. If you do,
//!  search your library for a card, put it into your hand, then shuffle."

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Servant of the Stinger");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP intervening-if: "if you've committed a crime this turn" —
            // no crime-tracking condition predicate is available; the gate is
            // omitted (the trigger fires unconditionally as a best-effort
            // partial). The "you may sacrifice this creature. If you do,
            // search..." optional-sacrifice-then-tutor is also not expressible
            // as one effect; we tutor a card to hand directly.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: tutor_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "you may sacrifice this creature. If you do, ..." optional
    // self-sacrifice cost on the resolution is not expressible inside a
    // triggered-ability effect; the tutor is emitted unconditionally.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::default(),
        reveal: false,
    }]
}
