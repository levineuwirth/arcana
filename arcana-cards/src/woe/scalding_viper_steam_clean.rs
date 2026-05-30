//! Scalding Viper // Steam Clean — `{1}{R}` // `{1}{U}` Elemental Snake creature 2/1.
//! Whenever an opponent casts a spell with mana value 3 or less, this creature deals
//! 1 damage to that player.
//! Adventure "Steam Clean" (`{1}{U}` sorcery): Return target nonland permanent to its
//! owner's hand.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scalding Viper");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let snake_sub = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);
    subtypes.0.insert(snake_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid creature cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Adventure face "Steam Clean" — sorcery {1}{U}, return target nonland permanent.
    let adv_name = reg.interner_mut().intern("Steam Clean");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid adventure cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Return target nonland permanent to its owner's hand.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::new().without_types(TypeLine::LAND.into()),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: steam_clean_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_max_cmc(3)),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: viper_sting,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn viper_sting(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let caster = match trig.triggering_caster() {
        Some(p) => p,
        None => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(caster),
        amount: 1,
    }]
}

fn steam_clean_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}
