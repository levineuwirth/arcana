//! Commander Mustard — `{3}{R}{W}` 5/5 Legendary Human Soldier with
//! Vigilance and Trample.
//! "Other Soldiers you control have vigilance, trample, and haste." (GAP:
//! no subtype-scoped keyword-anthem static primitive in the demonstrated
//! API.)
//! "{2}{R}{W}: Until end of turn, Soldiers you control gain 'Whenever this
//! creature attacks, it deals 1 damage to defending player.'"

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Commander Mustard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{R}{W}: Until end of turn, Soldiers you control gain \"Whenever this creature attacks, it deals 1 damage to defending player.\"".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{R}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_attack_ping,
        }),
    )
}

fn grant_attack_ping(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Soldier").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::Sequence(
        ids.into_iter()
            .map(|id| Effect::GrantTriggeredAbility {
                target: id,
                ability: Box::new(TriggeredAbilityDef {
                    id: GRANTED_TRIGGER_ID_BASE + 1,
                    trigger_condition: TriggerCondition::SelfAttacks,
                    intervening_if: None,
                    effect: ping_defender,
                    trigger_zones: vec![Zone::Battlefield],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                }),
                duration: Duration::EndOfTurn,
            })
            .collect(),
    )]
}

fn ping_defender(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(p),
        amount: 1,
    }]
}
