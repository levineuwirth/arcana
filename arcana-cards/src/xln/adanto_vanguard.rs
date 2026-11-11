//! Adanto Vanguard — `{1}{W}` 1/1 Creature — Vampire Soldier.
//!
//! * Static: "As long as this creature is attacking, it gets +2/+0." — a
//!   conditional continuous static, not a triggered/activated ability.
//! * "Pay 4 life: This creature gains indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration, SourceCondition, SelfModification};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::zones::Zone;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adanto Vanguard");
    let vampire = reg.interner_mut().intern("Vampire");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    // Live attacking bonus; A1.5br replaces the entry installer.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1, trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None, effect: install_condition,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay 4 life: This creature gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    life: 4,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: gain_indestructible,
            }),
    )
}

fn gain_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}

// A1.5bq: live, source-only applicability. A1.5br owns immediate static
// installation and ability loss; until then this uses the existing ETB path.
fn install_condition(_: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect { effect: ContinuousEffect::conditional_self(
        trig.source, SourceCondition::Attacking,
        SelfModification::ModifyPt { power: 2, toughness: 0 }, Duration::WhileSourceOnBattlefield,
    ) }]
}
