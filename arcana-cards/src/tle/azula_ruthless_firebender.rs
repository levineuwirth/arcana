//! Azula, Ruthless Firebender — `{2}{B}` 3/3 Legendary Creature — Human Noble.
//!
//! Oracle:
//! * Firebending 1 (Whenever this creature attacks, add {R} until end of combat.) — GAP'd
//! * Whenever Azula attacks, you may discard a card. Then you get an experience
//!   counter for each player who discarded a card this turn.  (effect GAP'd)
//! * {2}{B}: Until end of turn, Azula gets +1/+1 for each experience counter you
//!   have and gains menace.  (pump GAP'd; menace grant expressed)
//!
//! Firebending is not in the keyword surface (no KeywordAbility variant) and
//! its mana-until-end-of-combat is not expressible — GAP'd. The attack
//! trigger's payload (optional discard + experience counters, a PLAYER
//! counter with no AddCounters-to-player path) is GAP'd; the trigger
//! condition is wired structurally. The activated ability grants menace
//! (expressible) but its "+1/+1 for each experience counter you have" is a
//! dynamic amount over an unreadable player-counter total — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Azula, Ruthless Firebender");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    // GAP: Firebending 1 — no KeywordAbility variant; mana-until-end-of-combat not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_discard_experience,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Until end of turn, Azula gets +1/+1 for each experience counter you have and gains menace.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_menace,
            }),
    )
}

fn attack_discard_experience(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card. Then you get an experience counter for each
    //      player who discarded a card this turn" — no optional-discard effect and
    //      experience is a PLAYER counter (AddCounters targets an ObjectId only).
    Vec::new()
}

fn pump_and_menace(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "+1/+1 for each experience counter you have" — experience is a player
    //      counter with no readable total; the dynamic pump is omitted.
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Menace,
        duration: Duration::EndOfTurn,
    }]
}
