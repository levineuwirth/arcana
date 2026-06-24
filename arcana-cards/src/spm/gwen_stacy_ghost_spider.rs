//! Gwen Stacy // Ghost-Spider
//!
//! Front face: `{1}{R}` Legendary Creature — Human Performer Hero 2/1.
//! When Gwen Stacy enters, exile the top card of your library. You may play that card
//!   for as long as you control this creature.
//! {2}{U}{R}{W}: Transform Gwen Stacy. Activate only as a sorcery.
//!
//! Back face: Legendary Creature — Spider Human Hero, Flying, Vigilance, Haste.
//! Whenever you play a land from exile or cast a spell from exile, put a +1/+1 counter
//!   on Ghost-Spider.
//! Remove two counters from Ghost-Spider: Exile the top card of your library. You may
//!   play that card this turn.
//!
//! GAP: ETB "exile the top card, may play while controlling this creature" — per-source
//!      exile-with-play-permission tracking deferred.
//! Transform activation {2}{U}{R}{W} (sorcery speed) is wired as a front-face activated
//!      ability; card colors stay R per spec.
//! GAP: back-face triggered ability ("Whenever you play a land from exile or cast a spell
//!      from exile, put a +1/+1 counter") not expressible — neither TriggerCondition::SpellCast
//!      nor GameEvent::SpellCast carries a from-zone, and ZoneChange can't distinguish a
//!      play/cast-from-exile from any other exile→stack move. No cast-from-exile trigger primitive.
//! Back-face activated ability (remove two +1/+1 counters → exile top card, may play this turn)
//!      is wired via remove_self_counter + Effect::ImpulseExile, gated to face 1.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gwen Stacy");
    let human_sub = reg.interner_mut().intern("Human");
    let performer_sub = reg.interner_mut().intern("Performer");
    let hero_sub = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(performer_sub);
    subtypes.0.insert(hero_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Ghost-Spider
    let back_name = reg.interner_mut().intern("Ghost-Spider");
    let spider_sub = reg.interner_mut().intern("Spider");
    let back_human_sub = reg.interner_mut().intern("Human");
    let back_hero_sub = reg.interner_mut().intern("Hero");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spider_sub);
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(back_hero_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::Vigilance,
                KeywordAbility::Haste,
            ],
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: exile top card + may play (GAP).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {2}{U}{R}{W}: Transform Gwen Stacy (sorcery speed, face 0).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{R}{W}: Transform Gwen Stacy. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{R}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_gwen,
            })
            // Back-face: Remove two +1/+1 counters from Ghost-Spider: Exile the top
            // card of your library. You may play that card this turn (face 1).
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove two counters from Ghost-Spider: Exile the top card of your library. You may play that card this turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: ghost_spider_impulse,
            }),
            // GAP: back-face triggered ability (play land / cast spell from exile →
            // +1/+1 counter) not expressible — no cast-from-exile trigger primitive.
    )
}

fn ghost_spider_impulse(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile { player: ctx.controller, count: 1 }]
}

fn etb_exile_top(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of your library, may play for as long as you control this creature"
    // — per-source exile-with-play-permission tracking deferred; no Effect variant available.
    Vec::new()
}

fn transform_gwen(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
