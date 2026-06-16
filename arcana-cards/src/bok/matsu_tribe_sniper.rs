//! Matsu-Tribe Sniper — `{1}{G}` 1/1 green Creature — Snake Warrior Archer.
//! "{T}: This creature deals 1 damage to target creature with flying."
//! "Whenever this creature deals damage to a creature, tap that creature and
//! it doesn't untap during its controller's next untap step."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Matsu-Tribe Sniper");
    let snake = reg.interner_mut().intern("Snake");
    let warrior = reg.interner_mut().intern("Warrior");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(warrior);
    subtypes.0.insert(archer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 1 damage to target creature with flying."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .with_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_flyer,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: tap_and_lock,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn ping_flyer(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 1,
    }]
}

fn tap_and_lock(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap THAT creature and it doesn't untap …" — no PendingTrigger
    // accessor exposes the damaged object for a DamageDealt trigger (only
    // damaged_player / damage_amount exist), so "that creature" can't be
    // identified. The tap + don't-untap (Stun) effect is omitted.
    Vec::new()
}
