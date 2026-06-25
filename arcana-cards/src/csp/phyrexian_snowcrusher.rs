//! Phyrexian Snowcrusher — `{6}` 6/5 Snow Artifact Creature — Phyrexian Juggernaut.
//!
//! Oracle:
//! * This creature attacks each combat if able.  (SELF must-attack —
//!   CR 508.1a, installed on ETB)
//! * `{1}{S}`: This creature gets +1/+0 until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Phyrexian Snowcrusher");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // "This creature attacks each combat if able" — SELF must-attack,
    // installed on ETB.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{S}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
