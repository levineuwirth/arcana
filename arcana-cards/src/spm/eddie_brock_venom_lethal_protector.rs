//! Eddie Brock // Venom, Lethal Protector
//! Front: `{2}{B}` Legendary Creature — Human Hero Villain 3/3
//! When Eddie Brock enters, return target creature card with mana value 1 or less from your
//! graveyard to the battlefield.
//! {3}{B}{R}{G}: Transform Eddie Brock. Activate only as a sorcery.
//!
//! Back: Legendary Creature — Symbiote Hero Villain (same {2}{B}? — MDFC, no cost on back)
//! Menace, trample, haste
//! Whenever Venom attacks, you may sacrifice another creature. If you do, draw X cards,
//! then you may put a permanent card with mana value X or less from your hand onto the
//! battlefield, where X is the sacrificed creature's mana value.
//!
//! {3}{B}{R}{G}: Transform Eddie Brock (sorcery speed) is wired as a front-face
//!      activated ability (face 0). This is a transforming DFC: the back face has no
//!      mana cost and is reachable only via the transform ability.
//! GAP: Back-face triggered ability (Venom attacks, may sacrifice, draw X, put permanent) —
//!      the dynamic X (= sacrificed creature's mana value) is not computable: no
//!      PendingTrigger accessor exposes the sacrificed creature's mana value to drive
//!      DrawCards count and the "put a permanent with MV ≤ X from hand" follow-on. The
//!      whole payload is X-dependent, so the trigger is left unwired rather than no-op'd.
//! GAP: "Activate only as a sorcery" modeled via is_instant_speed: false.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eddie Brock");
    let human_sub = reg.interner_mut().intern("Human");
    let hero_sub = reg.interner_mut().intern("Hero");
    let villain_sub = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(hero_sub);
    subtypes.0.insert(villain_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Venom, Lethal Protector
    let back_name = reg.interner_mut().intern("Venom, Lethal Protector");
    let symbiote_sub = reg.interner_mut().intern("Symbiote");
    let hero_back_sub = reg.interner_mut().intern("Hero");
    let villain_back_sub = reg.interner_mut().intern("Villain");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(symbiote_sub);
    back_subtypes.0.insert(hero_back_sub);
    back_subtypes.0.insert(villain_back_sub);

    let back_chars = Characteristics {
        name: back_name,
        mana_cost: None,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    // ETB trigger: return target creature card with mana value 1 or less from your graveyard to battlefield
    let etb_target = TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::creature().with_max_cmc(1).controlled_by(ControllerConstraint::You),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back_face)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: eddie_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_target],
            })
            // {3}{B}{R}{G}: Transform Eddie Brock. Activate only as a sorcery (face 0).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{R}{G}: Transform Eddie Brock. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_eddie,
            }),
            // GAP: back-face "Whenever Venom attacks" trigger left unwired — the
            // dynamic X (sacrificed creature's mana value) driving draw-X and the
            // put-a-permanent-MV-≤-X-from-hand follow-on is not computable.
    )
}

fn transform_eddie(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn eddie_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
