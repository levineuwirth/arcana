//! Chakram Slinger — `{4}{R}` 2/4 Human Warrior.
//! "Partner with Chakram Retriever (When this creature enters, target
//!  player may put Chakram Retriever into their hand from their
//!  library, then shuffle.)
//!  {R}, {T}: This creature deals 2 damage to target player or
//!  planeswalker."
//!
//! Partner / Partner with have no KeywordAbility variant, but the
//! reminder-text ETB is expressible: an ETB trigger targeting a player
//! that tutors a card named "Chakram Retriever" into that player's
//! hand (shuffle is automatic). The activated ability deals 2 damage
//! to a target player (the "or planeswalker" half is not expressible
//! as a single target filter — modeled as target player).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chakram Slinger");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    // Pre-intern the partner's name so the tutor effect can look it up.
    let _retriever = reg.interner_mut().intern("Chakram Retriever");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Partner / Partner with — no KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fetch_partner,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, {T}: Chakram Slinger deals 2 damage to target \
                       player or planeswalker."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_two_to_player,
            }),
    )
}

fn etb_fetch_partner(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let nm = reg.interner().lookup("Chakram Retriever");
    vec![Effect::TutorToHand {
        player: *p,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}

fn deal_two_to_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Player(*p),
        amount: 2,
    }]
}
